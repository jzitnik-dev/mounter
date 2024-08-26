use std::process;

use crate::preferences::{config::{is_valid, ValidationResult}, preferences::Preferences};

pub async fn set_config(mut prefs: Preferences, options: Vec<String>, config_file: &Option<String>) {
    let key = options.get(0).unwrap();
    let value = options.get(1).unwrap().replace("*", "-");

    match is_valid(key, &value) {
        ValidationResult::ValueError => {
            println!("Invalid value in \"{}\" reading \"{}\".", key, value);
            process::exit(1);
        }
        ValidationResult::KeyError => {
            println!("Invalid key \"{}\".", key);
            process::exit(1);
        }
        ValidationResult::Correct => {}
    }

    prefs.update_config(key, &value, config_file).await.unwrap_or_else(|err| {
        eprint!("Error saving config: {}", err);
        process::exit(1);
    });
}
