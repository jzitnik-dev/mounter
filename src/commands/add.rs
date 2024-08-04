use std::process;
use dialoguer::Input;
use crate::preferences::{mount_point::MountPoint, preferences::Preferences};

pub async fn add(name: String, mut prefs: Preferences, config_file: &Option<String>) {
    let address: String = Input::new()
        .with_prompt("Enter address for the mount point (for example /dev/sda1)")
        .interact_text()
        .expect("Failed to read line");

    let mount_location: String = Input::new()
        .with_prompt("Enter mount location (for example /mnt)")
        .interact_text()
        .expect("Failed to read line");

    let flags: String = Input::new()
        .with_prompt("(optional) Enter custom flags separated by ; (for example \"-t cifs;-o credentials=/etc/smbcredentials)\")")
        .default(String::new())
        .interact_text()
        .expect("Failed to read input");

    let mount_point = MountPoint {
        name,
        address: address.trim().to_owned(),
        mount_location: mount_location.trim().to_owned(),
        flags: flags.trim().to_owned(),
    };

    prefs.add_mount_point(mount_point, config_file).await.unwrap_or_else(|err| {
        eprint!("Error saving mount point: {}", err);
        process::exit(1);
    });

    println!("Mount point saved!");
}
