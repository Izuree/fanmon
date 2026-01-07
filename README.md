# ASCII Fan Monitor

A terminal-based animated ASCII fan that syncs with your system's actual fan speed using NBFC (NoteBook FanControl) Service listener.

## Screenshots

![ASCII Fan Normal](2026-01-07-194301_hyprshot.png)

![ASCII Fan Critical](2026-01-07-194314_hyprshot.png)


```bash
ascii-fan [OPTIONS]
```

**Options:**
- `-h, --help` - Show help message
- `-s, --speed <NUM>` - Speed scaling factor (default: 1.0)
- `-c, --color <COLOR>` - Set fan color (red, yellow, blue, green, cyan, purple)
- `--hide-text` - Hide speed and status text
- `-C, --center` - Center the fan animation
