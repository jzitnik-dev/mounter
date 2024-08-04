use std::process;
use tabled::Table;
use crate::preferences::preferences::Preferences;

pub fn list(prefs: Preferences) {
    if prefs.saved_mount_points.len() == 0 {
        eprintln!("No mount points are saved!");
        process::exit(1);
    } else {
        let table = Table::new(&prefs.saved_mount_points).to_string();
        println!("Saved mount points:\n{}", table);
    }
}
