# Mounter

Simple Rust program for mounting drives on Linux.

Mounter can mount all of your external hard drives or even drives on a network. You can save your mount points for example your NAS and then you just simply mount it with mounter.

# Dependencies/Programs needed

- `mount` For mounting drives
- `umount` For unmounting drives
- `dmenu/rofi` If you want to use dmenu
- `sh` For executing scripts
- `lsblk` For --all command
- `jq` Used in --all command for parsing output from lsblk
- `pkexec` If you use sudo with dmenu enabled

# Installation

Download binary form Github releases and add it to PATH.

# How to use

## Adding a new mount point to the config

```bash
mounter -a NAME
```

For example

```bash
mounter -a nas
```

## Removing a mount point form the config

```bash
mounter -r NAME
```

For example

```bash
mounter -r nas
```

## Mounting a drive from the config

```bash
mounter
```

## Specifying config path

Optionally you can change where the config file will be located.

```bash
mounter --config ./preferences.json
```

## Mount any local disk

You can mount any connected disk on your linux machine using `--all` flag.

```bash
mounter --all
```

### No filter

`--all` flag filters drives that are currently mounted on `/`, `/boot` or `/home`. If you want to show these drives too use the `--no-filter` flag.

```bash
mounter --all --no-filter
```

# Configuration

See [Configuration](./CONFIGURATION.md) for configuration.

# Notice

If you use ask_for_password and use dmenu there is a change that your password will be shown in the polkit dialog.

It really depends on how long the command is because the password is inserted directly to the command and then pkexec shows part of the command in the dialog.

# TODO

<details>
  <summary>Better error handling</summary>
  
  Just generaly better error handling. Better explained error descriptions.
</details>

<details>
  <summary>Better ask_for_password implementation</summary>
  
  Better ask_for_password implementation for injecting the password to mount command.
  
  Right now the password from ask_for_password is injected to the mount command as a flag like this:

```rust
Flag {
    name: String::from("-o"),
    value: Some(format!("password={}", password)),
}
```

This is not very good because the command is then shown in the polkit dialog by pkexec. So the part of the password or the entire password is shown there.

I thought about saving the password to a file. and then linking the file to the mount command like this.

```rust
Flag {
    name: String::from("-o"),
    value: Some(format!("credentials={}", file_path)),
}
```

But there are also some downsides. The main one is that the file could be left over on the file system for example on crash. Also the file would be readable by other programs and people easily. We could theoretically make the file owner root but then we whould have to create more password dialogs or running mounter fully as a root. (I dont want that.)
</details>
