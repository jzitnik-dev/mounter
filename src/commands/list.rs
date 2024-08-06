use crate::{
    preferences::{config::get_value, preferences::Preferences},
    utils::dmenu::run_dmenu_list,
};
use std::process;
use tabled::Table;

pub fn list(prefs: Preferences) {
    let use_dmenu = match get_value(&prefs.config, "dmenu.use").as_str() {
        "true" => true,
        _ => false,
    };

    if use_dmenu {
        let options = prefs
            .saved_mount_points
            .iter()
            .map(|mount_point| mount_point.name.clone())
            .collect();

        run_dmenu_list(&prefs, &options, "List of mount points");
        return;
    }
    if prefs.saved_mount_points.len() == 0 {
        eprintln!("No mount points are saved!");
        process::exit(1);
    } else {
        let table = Table::new(&prefs.saved_mount_points).to_string();
        println!("Saved mount points:\n{}", table);
    }
}
