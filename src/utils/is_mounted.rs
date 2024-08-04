use crate::preferences::mount_point::MountPoint;
use std::fs;

pub fn is_mounted(mount_point: &MountPoint) -> bool {
    let mounts = fs::read_to_string("/proc/mounts").expect("Failed to read /proc/mounts");

    mounts
        .lines()
        .any(|line| line.starts_with(&mount_point.address))
}
