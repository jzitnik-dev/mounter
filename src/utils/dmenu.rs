use std::process::{exit, Command};
use std::{str, vec};

use crate::preferences::{config::get_value, preferences::Preferences};

use super::flag_merge::{add_flags, flag_merge, parse_flags, stringify_flags, Flag};

pub fn run_gui_password_dialog(pref: &Preferences, message: &str) -> Option<String> {
    let dialog = get_value(&pref.config, "dmenu.password_dialog.program");

    let mut command = match dialog.as_str() {
        "zenity" => {
            let mut command = Command::new("zenity");
            command.arg("--password");
            command.arg(format!("--title=\"{}\"", message));

            command
        }
        "yad" => {
            let mut command = Command::new("yad");
            command.arg("--entry");
            command.arg("--hide-text");
            command.arg(format!("--title=\"{}\"", message));
            command.arg(format!("--text=\"{}\"", message));
            command.arg("--width=300");

            command
        }
        "kdialog" => {
            let mut command = Command::new("kdialog");
            command.arg("--password");
            command.arg(message);

            command
        }
        "rofi" => {
            let mut command = Command::new("rofi");

            let userflags = get_value(&pref.config, "dmenu.password_dialog.rofi.flags");
            let flags = vec![
                Flag {
                    name: String::from("-dmenu"),
                    value: None,
                },
                Flag {
                    name: String::from("-p"),
                    value: Some(String::from(message)),
                },
                Flag {
                    name: String::from("-theme-str"),
                    value: Some(String::from(
                        "entry {placeholder-text: \"Enter your password\";}",
                    )),
                },
                Flag {
                    name: String::from("-password"),
                    value: None,
                },
                Flag {
                    name: String::from("-lines"),
                    value: Some(String::from("1")),
                },
            ];
            let user_flags = parse_flags(userflags).unwrap_or_else(|e| {
                eprintln!("Error while parsing flags: {}", e);
                exit(1);
            });

            let final_flags = flag_merge(&flags, &user_flags, &vec![]);

            add_flags(&mut command, final_flags);

            command
        }
        _ => {
            eprintln!("Invalid gui password dialog.");
            exit(1);
        }
    };

    let output = command.output().expect("Failed to execute command");

    let mut log = String::new();
    log.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data"),
    });

    log = log.trim().to_string();

    if log.len() == 0 {
        return None;
    }

    Some(log)
}

pub fn run_dmenu_list(prefs: &Preferences, options: &Vec<String>, message: &str) -> String {
    let echo_command = format!("echo -e '{}'", options.join("\\n"));

    run_dmenu_global(prefs, echo_command, message)
}

pub fn run_dmenu_global(prefs: &Preferences, echo_command: String, message: &str) -> String {
    let dmenu_command = get_value(&prefs.config, "dmenu.command");
    let dmenu_flags = get_value(&prefs.config, "dmenu.flags");
    let mut shell_command = format!("{} | {}", echo_command, dmenu_command);

    let default_flags = vec![Flag {
        name: String::from("-p"),
        value: Some(format!("\"{}\"", message)),
    }];
    let user_flags = parse_flags(dmenu_flags).unwrap_or_else(|e| {
        eprintln!("Error while parsing flags: {}", e);
        exit(1);
    });

    let final_flags = flag_merge(&default_flags, &user_flags, &vec![]);
    let final_flags_str = stringify_flags(final_flags);
    shell_command.push_str(&format!(" {}", final_flags_str));

    let mut command = Command::new("sh");
    command.arg("-c").arg(shell_command);

    let output = command.output().expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    let mut log = String::new();
    log.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("Got non UTF-8 data from command"),
    });

    String::from(log.trim())
}
