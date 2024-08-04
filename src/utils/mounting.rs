use crate::preferences::mount_point::MountPoint;
use std::process::{exit, Command};

pub fn mount(mount_point: &MountPoint, sudo: bool) {
    let mut command = if sudo {
        let mut cmd = Command::new("sudo");
        cmd.arg("mount");
        cmd
    } else {
        Command::new("mount")
    };

    if mount_point.flags.trim().len() != 0 {
        let flags: Vec<&str> = mount_point.flags.split(';').collect();
        for flag in flags {
            for flagx in flag.split(' ').collect::<Vec<_>>() {
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

pub fn umount(mount_point: &MountPoint, sudo: bool) {
    let output = match sudo {
        true => Command::new("sudo")
            .arg("umount")
            .arg(&mount_point.mount_location)
            .output()
            .expect("Failed to execute command"),
        false => Command::new("umount")
            .arg(&mount_point.mount_location)
            .output()
            .expect("Failed to execute command"),
    };

    if !output.status.success() {
        eprintln!("Umount failed with status: {}", output.status);
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }
}
