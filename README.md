# Mounter

Simple Rust program for mounting drives on Linux.

Mounter can mount all of your external hard drives or even drives on a network. You can save your mount points for example your NAS and then you just simply mount it with mounter.

It even supports disks encrypted by LUKS.

# Dependencies/Programs needed

- `mount` For mounting drives
- `umount` For unmounting drives
- `dmenu/rofi` If you want to use dmenu
- `sh` For executing scripts
- `lsblk` For --all command
- `jq` Used in --all command for parsing output from lsblk
- `cryptsetup` For LUKS support, needs to be installed even when not using LUKS.

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

# TODO

<details>
  <summary>Better error handling</summary>
  
  Just generaly better error handling. Better explained error descriptions.
</details>
