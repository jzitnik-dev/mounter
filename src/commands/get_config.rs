use std::process::exit;

use crate::preferences::{config::{is_present, is_valid, IsPresentResponse, ValidationResult}, preferences::Preferences};

pub async fn get_config(prefs: Preferences, config: String) {
    match is_valid(&config, "") {
        ValidationResult::KeyError => {
            eprintln!("Invalid key \"{}\".", config);
            exit(1);
        }
        _ => ()
    }
    let present = is_present(&prefs.config, &config);

    match present {
        IsPresentResponse::Present(value) => {
            println!("Config \"{}\" has value \"{}\".", config, value);
        }
        IsPresentResponse::NotPresent(value) => {
            println!("Config \"{}\" has no selected value.\n\nUsing default: \"{}\"", config, value);
        }
    };
}
