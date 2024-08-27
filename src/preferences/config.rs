use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ValidationResult {
    Correct,
    ValueError,
    KeyError,
}

pub fn is_valid(key: &str, value: &str) -> ValidationResult {
    match key {
        "dmenu.password_dialog.rofi.flags" | "mount.flags" | "dmenu.flags" => {
            ValidationResult::Correct
        }
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
        _ => ValidationResult::KeyError,
    }
}

pub fn get_value(config: &HashMap<String, String>, key: &str) -> String {
    let defaults: HashMap<&str, &str> = [
        ("sudo", "false"),
        ("dmenu.use", "false"),
        ("dmenu.command", "dmenu"),
        ("dmenu.flags", ""),
        ("dmenu.password_dialog.program", "rofi"),
        ("dmenu.password_dialog.rofi.flags", ""),
        ("mount.flags", ""),
    ]
    .iter()
    .cloned()
    .collect();

    config.get(key).cloned().unwrap_or_else(|| {
        defaults
            .get(key)
            .map(|&v| v.to_string())
            .expect("Invalid key provided!")
    })
}
