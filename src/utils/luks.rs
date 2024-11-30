use std::process::Command;

pub fn get_luks_name(mount_address: &String) -> String {
    mount_address.replace("/", "_")
}

pub fn check_lusk(mount_address: &String, user_password: &Option<String>) -> bool {
    let mut lukscommand = if let Some(password) = user_password {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        cmd.arg(format!(
            "echo {} | sudo -S cryptsetup isLuks {}",
            password, mount_address
        ));
        cmd
    } else {
        let mut cmd = Command::new("cryptsetup");
        cmd.arg("isLuks");
        cmd.arg(mount_address);
        cmd
    };

    lukscommand
        .output()
        .expect("Error while running LUKS check")
        .status
        .success()
}

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
