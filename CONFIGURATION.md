# Configuration

You can configure mounter from cli.

Example:

```bash
mounter --config-set rofi.use true
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
