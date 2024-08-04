use std::process;
use crate::preferences::preferences::Preferences;

pub async fn remove(name: String, mut prefs: Preferences, config_file: &Option<String>) {
    let exists = prefs.saved_mount_points.iter().any(|mount_point| mount_point.name == name);

    if !exists {
        eprint!("Mount point with that name doesn't exist.");
        process::exit(1);
    }

    prefs.remove_mount_point(name, config_file).await.unwrap_or_else(|err| {
        eprint!("Error removing mount point: {}", err);
        process::exit(1);
    });

    println!("Mount point removed!");
}
