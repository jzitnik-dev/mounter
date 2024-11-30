use std::process::exit;
use dialoguer::Password;
use crate::preferences::preferences::Preferences;
use super::{dmenu::run_gui_password_dialog, logging::console_error};

pub fn ask_for_sudo(use_dmenu: bool, preferences: &Preferences) -> String {
    if use_dmenu {
        run_gui_password_dialog(&preferences, "Enter password for sudo").unwrap_or_else(|| {
            console_error(&preferences.config, "Password dialog canceled!");
            exit(1);
        })
    } else {
        Password::new()
            .with_prompt("Enter password for sudo")
            .interact()
            .expect("Failed to read password")
    }
}
