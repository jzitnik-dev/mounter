use crate::{
    commands::all::Partition,
    preferences::{config::get_value, mount_point::MountPoint, preferences::Preferences},
    utils::{flag_merge::Flag, logging::console_error},
};
use dialoguer::Password;
use std::{io::Write, process::{exit, Command, Stdio}};

use super::{
    dmenu::run_gui_password_dialog,
    flag_merge::{add_flags, flag_merge, parse_flags},
    logging::console_log,
    luks::{check_luks, get_luks_name, lock, unlock},
    sudo::ask_for_sudo,
};

pub fn is_mounted(partition: &Partition) -> bool {
    if partition.fstype == Some("crypto_LUKS".to_owned()) {
        if let Some(value) = &partition.children {
            if let Some(first_child) = value.get(0) {
                return first_child.to_owned().mountpoint.is_some();
            }
        }
    }

    partition.mountpoint.is_some()
}

// I fucking hate this code
pub fn get_mountpoint(partition: &Partition) -> Option<String> {
    if partition.fstype == Some("crypto_LUKS".to_owned()) {
        if let Some(value) = &partition.children {
            if let Some(first_child) = value.get(0) {
                if let Some(mount_point) = &first_child.mountpoint {
                    return Some(mount_point.to_string());
                }
            }
        }
    }

    partition.mountpoint.clone()
}

// TODO: Fix mounting
// The | in the command.arg does not work
pub fn mount(mount_point: &MountPoint, preferences: &Preferences) {
    let global_flags_config = get_value(&preferences.config, "mount.flags");
    let sudo = match get_value(&preferences.config, "sudo").as_str() {
        "true" => true,
        _ => false,
    };
    let use_dmenu = match get_value(&preferences.config, "dmenu.use").as_str() {
        "true" => true,
        _ => false,
    };

    let user_password = if sudo {
        Some(ask_for_sudo(use_dmenu, preferences))
    } else {
        None
    };

    let encrypted = check_luks(&mount_point.address, &user_password);
    if encrypted {
        let passphrase = run_gui_password_dialog(&preferences, "Enter passphrase for the volume:")
            .unwrap_or_else(|| {
                console_error(&preferences.config, "Password dialog canceled!");
                exit(1);
            });
        unlock(&user_password, &mount_point.address, passphrase);
    }

    let default_flags = if mount_point.ask_for_password == Some(true) {
        let password = if use_dmenu {
            run_gui_password_dialog(&preferences, "Enter password for your mount point")
                .unwrap_or_else(|| {
                    console_error(&preferences.config, "Password dialog canceled!");
                    exit(1);
                })
        } else {
            Password::new()
                .with_prompt("Enter password for your mount point")
                .interact()
                .expect("Failed to read password")
        };

        vec![Flag {
            name: String::from("-o"),
            value: Some(format!("password={}", password)),
        }]
    } else {
        vec![]
    };

    let mount_point_flags = parse_flags(mount_point.flags.clone()).unwrap_or_else(|e| {
        console_error(
            &preferences.config,
            format!("Error while parsing mount point flags: {}", e).as_str(),
        );
        exit(1);
    });
    let global_flags = parse_flags(global_flags_config).unwrap_or_else(|e| {
        console_error(
            &preferences.config,
            format!("Error while parsing mount flags: {}", e).as_str(),
        );
        exit(1);
    });

    let merge_ignore = vec!["-o".to_owned()];
    let flags1 = flag_merge(&default_flags, &global_flags, &merge_ignore);
    let flags2 = flag_merge(&flags1, &mount_point_flags, &merge_ignore);

    let mut command = Command::new("sudo");
    command.arg("-S").arg("mount");

    add_flags(&mut command, flags2);

    if encrypted {
        command.arg(format!(
            "/dev/mapper/{}",
            get_luks_name(&mount_point.address)
        ));
    } else {
        command.arg(&mount_point.address);
    }
    command.arg(&mount_point.mount_location);

    let mut child = command.spawn().expect("Failed to spawn mount command");

    if sudo {
        if let Some(password) = user_password {
            if let Some(stdin) = child.stdin.as_mut() {
                stdin
                    .write_all(format!("{}\n", password).as_bytes())
                    .expect("Failed to write to stdin");
            }
        }
    }

    let output = child.wait_with_output().expect("Failed to execute command");

    if !output.status.success() {
        console_error(
            &preferences.config,
            format!("Mount failed with status: {}", output.status).as_str(),
        );
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    console_log(
        &preferences.config,
        format!("Drive {} was mounted successfully!", mount_point.address).as_str(),
    );
}


pub fn umount_partition(partition: &Partition, preferences: &Preferences) {
    umount_addr(
        &get_mountpoint(&partition).unwrap(),
        &format!("/dev/{}", partition.name),
        preferences,
    )
}

pub fn umount(mount_point: &MountPoint, preferences: &Preferences) {
    umount_addr(
        &mount_point.mount_location,
        &mount_point.address,
        preferences,
    )
}

pub fn umount_addr(mount_location: &str, mount_address: &String, preferences: &Preferences) {
    let sudo = match get_value(&preferences.config, "sudo").as_str() {
        "true" => true,
        _ => false,
    };
    let use_dmenu = match get_value(&preferences.config, "dmenu.use").as_str() {
        "true" => true,
        _ => false,
    };
    let user_password = if sudo {
        Some(ask_for_sudo(use_dmenu, preferences))
    } else {
        None
    };

    let mut command = if sudo {
        let mut cmd = Command::new("sudo");
        cmd.arg("-S").arg("umount").arg(mount_location);
        if let Some(_password) = user_password.clone() {
            cmd.stdin(Stdio::piped());
        }
        cmd
    } else {
        let mut cmd = Command::new("umount");
        cmd.arg(mount_location);
        cmd
    };

    let mut child = command.spawn().expect("Failed to spawn command");

    if sudo {
        if let Some(password) = &user_password {
            if let Some(stdin) = child.stdin.as_mut() {
                stdin
                    .write_all(format!("{}\n", password).as_bytes())
                    .expect("Failed to write to stdin");
            }
        }
    }

    let output = child.wait_with_output().expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Umount failed with status: {}", output.status);
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    let encrypted = check_luks(mount_address, &user_password);
    if encrypted {
        lock(&user_password, mount_address);
    }

    console_log(
        &preferences.config,
        format!(
            "Drive that was mounted on {} was unmounted!",
            mount_location
        )
        .as_str(),
    )
}
