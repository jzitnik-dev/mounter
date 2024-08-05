use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::process::{exit, Command};
use std::str;

use crate::preferences::mount_point::MountPoint;
use crate::utils::mounting::{mount, umount_addr};

#[derive(Debug, Serialize, Deserialize)]
struct Partition {
    name: String,
    size: String,
    fstype: String,
    mountpoint: String,
}

pub fn all(no_filter: bool, sudo: bool) {
    // Idk better way to get all the drives
    let output = Command::new("sh")
        .arg("-c")
        .arg(
            "lsblk -o NAME,SIZE,FSTYPE,MOUNTPOINT,TYPE -J | jq '[
            .blockdevices[] | 
            select(.type == \"disk\") | 
            .children[]? | 
            { 
                name: .name, 
                size: .size, 
                fstype: (if .fstype == null then \"N/A\" else .fstype end), 
                mountpoint: (if .mountpoint == null then \"N/A\" else .mountpoint end) 
            }
        ]'",
        )
        .output()
        .expect("Failed to execute command");

    let mut log = String::new();
    log.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data"),
    });

    let partitions: Vec<Partition> = serde_json::from_str(&log).unwrap();

    let options: Vec<String> = partitions
        .iter()
        .filter(|part| {
            no_filter
                || part.mountpoint.trim() != "/"
                    && part.mountpoint != "/boot"
                    && part.mountpoint != "/home"
        })
        .map(|part| {
            format!(
                "{} {}",
                part.name,
                if part.mountpoint != "N/A" { "*" } else { "" }
            )
        })
        .collect();

    if options.len() == 0 {
        println!("No drives were found!");
        exit(1);
    }

    let selection = Select::new()
        .with_prompt("Choose a mount point")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let partition = partitions.get(selection).unwrap();

    if partition.mountpoint != "N/A" {
        umount_addr(&partition.mountpoint, sudo);
        return;
    }

    let mount_location: String = Input::new()
        .with_prompt("Enter mount location (for example /mnt)")
        .interact_text()
        .expect("Failed to read line");

    let address = format!("/dev/{}", partition.name);

    let mount_point = MountPoint {
        name: "".to_string(),
        address,
        mount_location,
        flags: "".to_string()
    };

    mount(&mount_point, sudo);
}
