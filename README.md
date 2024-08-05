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

## Mount any local disk

You can mount any connected disk on your linux machine using `--all` flag.

```bash
mounter --all
```

### No filter

`--all` flag filters drives that are currently mounted on `/`, `/boot` or `/home`. If you want to show these drives too use the `--no-filter flag`.

```bash
mounter --all --no-filter
```

# TODO

- Better error handling
