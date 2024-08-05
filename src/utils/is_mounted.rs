use std::fs;

pub fn is_mounted(address: &str) -> bool {
    let mounts = fs::read_to_string("/proc/mounts").expect("Failed to read /proc/mounts");

    mounts.lines().any(|line| line.starts_with(address))
}
