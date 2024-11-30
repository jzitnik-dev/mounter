use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ValidationResult {
    Correct,
    ValueError,
    KeyError,
}

pub fn is_valid(key: &str, value: &str) -> ValidationResult {
    match key {
        "dmenu.password_dialog.rofi.flags"
        | "mount.flags"
        | "dmenu.flags"
        | "logging.program.notify.flags" => ValidationResult::Correct,
        "sudo" | "dmenu.use" => match value {
            "true" | "false" => ValidationResult::Correct,
            _ => ValidationResult::ValueError,
        },
        "dmenu.command" => {
            if value.len() > 0 {
                ValidationResult::Correct
            } else {
                ValidationResult::ValueError
            }
        }
        "dmenu.password_dialog.program" => match value {
            "yanity" | "yad" | "kdialog" | "rofi" => ValidationResult::Correct,
            _ => ValidationResult::ValueError,
        },
        "logging.program" => match value {
            "cli" | "notify" => ValidationResult::Correct,
            _ => ValidationResult::ValueError,
        },
        _ => ValidationResult::KeyError,
    }
}

pub enum IsPresentResponse {
    Present(String),
    NotPresent(String),
}

pub fn is_present(config: &HashMap<String, String>, key: &str) -> IsPresentResponse {
    let defaults: HashMap<&str, &str> = [
        ("sudo", "true"),
        ("dmenu.use", "false"),
        ("dmenu.command", "dmenu"),
        ("dmenu.flags", ""),
        ("dmenu.password_dialog.program", "rofi"),
        ("dmenu.password_dialog.rofi.flags", ""),
        ("mount.flags", ""),
        ("logging.program", "cli"),
        ("logging.program.notify.flags", ""),
    ]
    .iter()
    .cloned()
    .collect();

    let conf = config.get(key);

    match conf {
        Some(value) => IsPresentResponse::Present(value.clone()),
        None => {
            let value = defaults
                .get(key)
                .map(|&v| v.to_string())
                .unwrap_or_else(|| panic!("Invalid key provided!"));
            IsPresentResponse::NotPresent(value)
        }
    }
}

pub fn get_value(config: &HashMap<String, String>, key: &str) -> String {
    match is_present(config, key) {
        IsPresentResponse::Present(value) => value,
        IsPresentResponse::NotPresent(value) => value,
    }
}
