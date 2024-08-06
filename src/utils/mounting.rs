use crate::preferences::mount_point::MountPoint;
use std::process::{exit, Command};

pub fn mount(mount_point: &MountPoint, sudo: bool, use_dmenu: bool) {
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

    // Add flags to the command if they exist
    if !mount_point.flags.trim().is_empty() {
        let flags: Vec<&str> = mount_point.flags.split(';').collect();
        for flag in flags {
            for flagx in flag.split_whitespace() {
                command.arg(flagx);
            }
        }
    }

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
