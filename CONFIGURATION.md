# Configuration

You can configure mounter from cli.

Example:

```bash
mounter --config-set dmenu.use true
```

## Configuration list

### sudo

Use sudo (without dmenu) or polkit (with dmenu) while mounting/unmounting a drive.

**Values**: "true", "false"
**Default**: "false"

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

### dmenu.password_dialog.program

Choose what program should be used to show password dialog when using dmenu.

dmenu does not support password dialog as far as I know.

**Values**: "yanity" "yad" "kdialog" "rofi"
**Default**: "rofi"

### dmenu.password_dialog.rofi.flags

Custom flags for rofi when dmenu.password_dialog.program is set to "rofi". Flags are seperated by `;`. Same format as in dmenu.flags.

### mount.flags

Custom flags for mount command that mounts the drive. They are merged with flags property in the mount point.

##### Use case:

I personally use this like this:

`-o uid=kuba,gid=kuba`

So all of the drives that I mount in my Linux machine is owned by my user.

Flags are seperated by `;`. Same format as in dmenu.flags.
