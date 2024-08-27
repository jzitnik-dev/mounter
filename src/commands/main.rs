use crate::preferences::config::get_value;
use crate::preferences::preferences::Preferences;
use crate::utils::dmenu::run_dmenu_list;
use crate::utils::is_mounted::is_mounted;
use crate::utils::mounting::{mount, umount};
use dialoguer::Select;
use std::process::exit;

pub fn main(prefs: Preferences) {
    if prefs.saved_mount_points.len() == 0 {
        eprint!("No mount points are saved!");
        exit(1);
    }

    let use_dmenu = match get_value(&prefs.config, "dmenu.use").as_str() {
        "true" => true,
        _ => false,
    };

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

    let selection = match use_dmenu {
        true => {
            let value = run_dmenu_list(&prefs, &options, "Select a mount point");

            match options.iter().position(|x| x == &value) {
                Some(index) => index,
                None => {
                    eprintln!("Selected mount point is not in the list!");
                    exit(1);
                },
            }
        }
        false => Select::new()
            .with_prompt("Choose a mount point")
            .items(&options)
            .default(0)
            .interact()
            .unwrap(),
    };

    let mount_point = prefs.saved_mount_points.get(selection).unwrap();
    let mounted = is_mounted(&mount_point.address);

    let use_sudo = match get_value(&prefs.config, "sudo").as_str() {
        "true" => true,
        _ => false,
    };

    if mounted {
        umount(mount_point, use_sudo, use_dmenu);
    } else {
        mount(mount_point, &prefs, use_sudo, use_dmenu);
    }
}
