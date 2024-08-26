use std::process::{exit, Command};
use std::str;

use crate::preferences::{config::get_value, preferences::Preferences};

pub fn run_gui_password_dialog(dialog: String) -> Option<String> {
    let mut command = match dialog.as_str() {
        "zenity" => {
            let mut command = Command::new("zenity");
            command.arg("--password");
            command.arg("--title=\"Enter password for your mount point\"");

            command
        }
        "yad" => {
            let mut command = Command::new("yad");
            command.arg("--entry");
            command.arg("--hide-text");
            command.arg("--title=\"Enter password for your mount point\"");
            command.arg("--text=\"Enter password for your mount point:\"");
            command.arg("--width=300");

            command
        }
        "kdialog" => {
            let mut command = Command::new("kdialog");
            command.arg("--password");
            command.arg("Enter password for your mount point");

            command
        }
        "rofi" => {
            let mut command = Command::new("rofi");
            command.arg("-dmenu");
            command.arg("-p");
            command.arg("Enter password for your mount point");
            command.arg("-theme-str");
            command.arg("entry {placeholder-text: \"Enter your password\";}");
            command.arg("-password");
            command.arg("-lines");
            command.arg("1");

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

    if !dmenu_flags.trim().is_empty() {
        shell_command.push_str(&format!(" {}", dmenu_flags.replace(";", " ")));
    }

    shell_command.push_str(&format!(" -p \"{}\"", message));

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
        Err(_) => panic!("got non UTF-8 data"),
    });

    String::from(log.trim())
}
