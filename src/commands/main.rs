use crate::preferences::preferences::Preferences;
use crate::utils::is_mounted::is_mounted;
use crate::utils::mounting::{mount, umount};
use dialoguer::Select;
use std::process;

pub fn main(prefs: Preferences, sudo: bool) {
    if prefs.saved_mount_points.len() == 0 {
        eprint!("No mount points are saved!");
        process::exit(1);
    }

    let options: Vec<String> = prefs
        .saved_mount_points
        .iter()
        .map(|mount_point| {
            let mut name = mount_point.name.clone();
            if is_mounted(&mount_point.address) {
                name.push_str(" *");
            }
            name
        })
        .collect();

    let selection = Select::new()
        .with_prompt("Choose a mount point")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let mount_point = prefs.saved_mount_points.get(selection).unwrap();
    let mounted = is_mounted(&mount_point.address);

    if mounted {
        umount(mount_point, sudo);
    } else {
        mount(mount_point, sudo);
    }
}
