use std::{
    collections::HashMap,
    io::Write,
    process::{Command, Stdio},
};

use crate::utils::logging::console_error;

use super::logging::console_log;

pub fn get_luks_name(mount_address: &String) -> String {
    mount_address.replace("/", "_")
}

pub fn check_luks(mount_address: &String, user_password: &Option<String>) -> bool {
    let mut command = if let Some(_password) = user_password {
        let mut cmd = Command::new("sudo");
        cmd.arg("-S")
            .arg("cryptsetup")
            .arg("isLuks")
            .arg(mount_address);
        cmd.stdin(Stdio::piped());
        cmd
    } else {
        let mut cmd = Command::new("cryptsetup");
        cmd.arg("isLuks").arg(mount_address);
        cmd
    };

    let mut child = command.spawn().expect("Failed to spawn cryptsetup command");

    if let Some(password) = user_password {
        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(format!("{}\n", password).as_bytes())
                .expect("Failed to write sudo password to stdin");
        }
    }

    let output = child.wait_with_output().expect("Failed to execute command");

    output.status.success()
}

// This will need a rewrite to not use echo
pub fn unlock(user_password: &Option<String>, address: &String, passphrase: String) {
    let mut command = if let Some(password) = user_password {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        cmd.arg(format!(
            "echo \"{}\" | sudo -S sh -c 'echo -n \"{}\" | cryptsetup luksOpen \"{}\" \"{}\" --key-file=-'",
            password,
            passphrase,
            address,
            get_luks_name(address)
        ));
        cmd
    } else {
        let mut cmd = Command::new("cryptsetup");
        cmd.arg("luksOpen");
        cmd.arg("--key-file=-");
        cmd.arg(address);
        cmd.arg(get_luks_name(address));
        cmd
    };

    command.output().expect("Error while decryping drive.");
}

pub fn lock(user_password: &Option<String>, address: &String, config: &HashMap<String, String>) {
    let luks_name = get_luks_name(address);

    let mut command = if user_password.is_some() {
        let mut cmd = Command::new("sudo");
        cmd.arg("-S")
            .arg("cryptsetup")
            .arg("luksClose")
            .arg(&luks_name);
        cmd.stdin(Stdio::piped());
        cmd
    } else {
        let mut cmd = Command::new("cryptsetup");
        cmd.arg("luksClose").arg(&luks_name);
        cmd
    };

    let mut child = command.spawn().expect("Failed to spawn cryptsetup command");

    if let Some(password) = user_password {
        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(format!("{}\n", password).as_bytes())
                .expect("Failed to write sudo password to stdin");
        }
    }

    let output = child
        .wait_with_output()
        .expect("Error while executing lock command");

    if !output.status.success() {
        console_error(config, "Failed to lock device!");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        console_log(config, "Device locked successfully.");
    }
}
