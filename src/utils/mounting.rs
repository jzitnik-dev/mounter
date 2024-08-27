use crate::{
    preferences::{config::get_value, mount_point::MountPoint, preferences::Preferences},
    utils::flag_merge::Flag,
};
use dialoguer::Password;
use std::process::{exit, Command};

use super::{
    dmenu::run_gui_password_dialog,
    flag_merge::{add_flags, flag_merge, parse_flags},
};

pub fn mount(mount_point: &MountPoint, preferences: &Preferences, sudo: bool, use_dmenu: bool) {
    let global_flags_config = get_value(&preferences.config, "mount.flags");

    let mut command = if sudo {
        if use_dmenu {
            let mut cmd = Command::new("pkexec");
            cmd.arg("mount");
            cmd
        } else {
            let mut cmd = Command::new("sudo");
            cmd.arg("mount");
            cmd
        }
    } else {
        Command::new("mount")
    };

    let default_flags = match mount_point.ask_for_password == Some(true) {
        true => {
            let password = match use_dmenu {
                true => run_gui_password_dialog(&preferences).unwrap_or_else(|| {
                    eprintln!("Password dialog canceled!");
                    exit(1);
                }),
                false => Password::new()
                    .with_prompt("Enter password for your mount point")
                    .interact()
                    .expect("Failed to read password"),
            };

            vec![Flag {
                name: String::from("-o"),
                value: Some(format!("password={}", password)),
            }]
        }
        false => vec![],
    };

    let mount_point_flags = parse_flags(mount_point.flags.clone()).unwrap_or_else(|e| {
        eprintln!("Error while parsing mount point flags: {}", e);
        exit(1);
    });
    let global_flags = parse_flags(global_flags_config).unwrap_or_else(|e| {
        eprintln!("Error while parsing mount flags: {}", e);
        exit(1);
    });

    let merge_ignore = vec!["-o".to_owned()];
    let flags1 = flag_merge(&default_flags, &global_flags, &merge_ignore);
    let flags2 = flag_merge(&flags1, &mount_point_flags, &merge_ignore);

    add_flags(&mut command, flags2);

    command.arg(&mount_point.address);
    command.arg(&mount_point.mount_location);

    let output = command.output().expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Mount failed with status: {}", output.status);
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }
}

pub fn umount(mount_point: &MountPoint, sudo: bool, use_dmenu: bool) {
    umount_addr(&mount_point.mount_location, sudo, use_dmenu)
}

pub fn umount_addr(mount_location: &str, sudo: bool, use_dmenu: bool) {
    let mut command = if sudo {
        if use_dmenu {
            let mut cmd = Command::new("pkexec");
            cmd.arg("umount");
            cmd
        } else {
            let mut cmd = Command::new("sudo");
            cmd.arg("umount");
            cmd
        }
    } else {
        Command::new("umount")
    };

    command.arg(mount_location);

    let output = command.output().expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Umount failed with status: {}", output.status);
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }
}
