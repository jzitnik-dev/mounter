use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::process::{exit, Command};
use std::str;

use crate::preferences::config::get_value;
use crate::preferences::mount_point::MountPoint;
use crate::preferences::preferences::Preferences;
use crate::utils::dmenu::{run_dmenu_global, run_dmenu_list};
use crate::utils::logging::{console_error, console_log};
use crate::utils::mounting::{is_mounted, mount, umount_partition};

#[derive(Debug, Serialize, Deserialize)]
pub struct Partition {
    pub name: String,
    pub size: String,
    pub fstype: Option<String>,
    pub mountpoint: Option<String>,
    pub r#type: String,
    pub children: Option<Vec<Box<Partition>>>,
}

fn filter(partition: &Partition, no_filter: bool) -> bool {
    if no_filter {
        return true;
    }

    if let Some(mount_point) = &partition.mountpoint {
        if mount_point.trim() == "/" || mount_point == "/boot" || mount_point == "/home" {
            return false;
        }
        return true;
    } else {
        if let Some(ref children) = partition.children {
            return children.iter().any(|child| filter(child, no_filter));
        } else {
            return true;
        }
    }
}

pub fn all(no_filter: bool, prefs: Preferences) {
    let use_dmenu = match get_value(&prefs.config, "dmenu.use").as_str() {
        "true" => true,
        _ => false,
    };

    // Idk better way to get all the drives
    let output = Command::new("sh")
        .arg("-c")
        .arg(
            "lsblk -o NAME,SIZE,FSTYPE,MOUNTPOINT,TYPE -J | jq '[
            .blockdevices[] | 
            select(.type == \"disk\") |
            .children[]? 
        ]'",
        )
        .output()
        .expect("Failed to execute command");

    let mut log = String::new();
    log.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data"),
    });

    // All partitions
    let partitions: Vec<Partition> = serde_json::from_str(&log).unwrap();

    // Partitions filtered and fomatted
    let options: Vec<String> = partitions
        .iter()
        .filter(|part| filter(part, no_filter))
        .map(|part| {
            format!(
                "Name: {}, Size: {}  {}",
                part.name,
                part.size,
                if is_mounted(part) { "*" } else { "" }
            )
        })
        .collect();

    if options.len() == 0 {
        console_log(&prefs.config, "No drives were found!");
        exit(1);
    }

    let selection = if use_dmenu {
        let value = run_dmenu_list(&prefs, &options, "Select a mount point");

        match options.iter().position(|x| x.trim() == &value) {
            Some(index) => index,
            None => {
                console_error(&prefs.config, "Selected mount point is not in the list!");
                exit(1);
            }
        }
    } else {
        Select::new()
            .with_prompt("Choose a mount point")
            .items(&options)
            .default(0)
            .interact()
            .unwrap()
    };

    // Selected partition
    let partition = partitions.get(selection).unwrap();

    // If selected partition is mounted unmount it
    if is_mounted(partition) {
        umount_partition(partition, &prefs);
        return;
    }

    let mount_location: String = if use_dmenu {
        run_dmenu_global(
            // This is very wacky
            &prefs,
            String::from("echo \"\""),
            "Enter mount location (for example /mnt)",
        )
    } else {
        Input::new()
            .with_prompt("Enter mount location (for example /mnt)")
            .interact_text()
            .expect("Failed to read line")
    };

    let address = format!("/dev/{}", partition.name);

    let mount_point = MountPoint {
        name: "".to_string(),
        address,
        mount_location,
        flags: "".to_string(),
        ask_for_password: None,
    };

    mount(&mount_point, &prefs);
}
