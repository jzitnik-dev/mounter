use std::{
    collections::HashMap,
    process::{exit, Command},
};

use crate::preferences::config::get_value;

use super::flag_merge::{add_flags, flag_merge, parse_flags, Flag};

fn send_notification(config: &HashMap<String, String>, message: &str) {
    let mut command = Command::new("notify-send");

    let userflags = get_value(config, "logging.program.notify.flags");
    let flags = vec![Flag {
        name: String::from("--app-name"),
        value: Some(String::from("mounter")),
    }];

    let user_flags = parse_flags(userflags).unwrap_or_else(|e| {
        eprintln!("Error while parsing flags: {}", e);
        exit(1);
    });

    let final_flags = flag_merge(&flags, &user_flags, &vec![]);

    add_flags(&mut command, final_flags);

    command.arg(message);

    command.output().expect("Failed to execute command");
}

pub fn console_error(config: &HashMap<String, String>, message: &str) {
    let config_type = get_value(config, "logging.program");

    if config_type == "cli" {
        eprintln!("{}", message);
    }

    if config_type == "notify" {
        send_notification(config, message);
    }
}

pub fn console_log(config: &HashMap<String, String>, message: &str) {
    let config_type = get_value(config, "logging.program");

    if config_type == "cli" {
        eprintln!("{}", message);
    }

    if config_type == "notify" {
        send_notification(config, message);
    }
}
