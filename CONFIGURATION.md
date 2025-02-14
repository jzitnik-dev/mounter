# Configuration

You can configure mounter from cli.

Example:

```bash
mounter --config-set dmenu.use true
```

You can also read the configuration.

Example

```bash
mounter --config-get dmenu.use
```

## Configuration list

### sudo

Run mount and other commands that may require root as a root.

**Values**: "true", "false"
**Default**: "true"

### dmenu.use

Use dmenu

**Values**: "true", "false"
**Default**: "false"

### dmenu.command

Specify what command should be ran. (for rofi users)

**Values**: any string
**Defualt**: "dmenu"

### dmenu.flags

Custom flags when running dmenu. Flags are seperated by `;`.

Example for rofi:

dmenu.command: "rofi"
dmenu.flags: "-dmenu"

**Default**: ""

### dmenu.password_dialog.program

Choose what program should be used to show password dialog when using dmenu.

dmenu does not support password dialog as far as I know.

**Values**: "yanity" "yad" "kdialog" "rofi"
**Default**: "rofi"

### dmenu.password_dialog.rofi.flags

Custom flags for rofi when dmenu.password_dialog.program is set to "rofi".

Flags are seperated by `;`. Same format as in dmenu.flags.

**Default**: ""

### mount.flags

Custom flags for mount command that mounts the drive. They are merged with flags property in the mount point.

Flags are seperated by `;`. Same format as in dmenu.flags.

**Default**: ""

##### Use case:

I personally use this like this:

`-o uid=1000,gid=1000`

So all of the drives that I mount in my Linux machine is owned by my user.

Keep in mind that not all filesystems support setting uid and gid.

### logging.program

What should be used for messages and logging.

**Values**: "cli", "notify"
**Default**: "cli"

Notify will send notify-send for the message but cli will just log it in stdout.

### logging.program.notify.flags

Custom flags when running notify-send when logging.program is set to notify.

Flags are seperated by `;`.

**Default**: ""

### luks

Enable or disable a LUKS support. This is only for users that do not have `cryptsetup` installed on their system. It is recommended to keep enabled even when you do not use LUKS.

**Values**: "true", "false"
**Default**: "true"
