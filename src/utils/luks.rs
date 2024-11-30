use std::{io::Write, process::{Command, Stdio}};

pub fn get_luks_name(mount_address: &String) -> String {
    mount_address.replace("/", "_")
}

pub fn check_luks(mount_address: &String, user_password: &Option<String>) -> bool {
    let mut command = if let Some(_password) = user_password {
        let mut cmd = Command::new("sudo");
        cmd.arg("-S").arg("cryptsetup").arg("isLuks").arg(mount_address);
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

pub fn lock(user_password: &Option<String>, address: &String) {
    let mut command = if let Some(password) = user_password {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        cmd.arg(format!(
            "echo \"{}\" | sudo -S cryptsetup luksClose \"{}\"",
            password,
            get_luks_name(address)
        ));
        cmd
    } else {
        let mut cmd = Command::new("cryptsetup");
        cmd.arg("luksClose");
        cmd.arg(get_luks_name(address));
        cmd
    };

    let output = command.output().expect("Error while locking drive.");

    if !output.status.success() {
        eprintln!("Failed to lock the device. Error: {:?}", output);
    } else {
        println!("Device locked successfully.");
    }
}
