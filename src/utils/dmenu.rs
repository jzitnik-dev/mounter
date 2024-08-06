use std::process::{exit, Command};
use std::str;

use crate::preferences::{config::get_value, preferences::Preferences};

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
