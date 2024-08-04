# Mounter

Simple Rust program for mounting drives on Linux.

# Installtion

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

If you want to run the mount command as a sudo run

```bash
mounter --sudo
```

## Specifying config path

Optionally you can change where the config file will be located.

```bash
mounter --config ./preferences.json
```

## TODO

- Ability to mount any connected disk
- Better error handling
