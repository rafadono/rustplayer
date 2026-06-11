# Keyboard shortcuts — RPlayer

## Reproduction

| Key | Action |
|-------|--------|
| `Space` | Play/Pause |
| `N` | next track |
| `P` | Previous track |
| `→` | Go forward 5 seconds |
| `←` | Go back 5 seconds |
| `Shift+→` | Skip ahead 60 seconds |
| `Shift+←` | Go back 60 seconds |
| `.` | Frame forward (requires pause) |
| `,` | Frame back (requires pause) |
| `R` | Cycle A-B loop: mark A → mark B → clear |

## Audio

| Key | Action |
|-------|--------|
| `↑` | Increase volume 5% |
| `↓` | Reduce volume 5% |
| `M` | Mute / Unmute |
| `+` | Increase speed 0.25× |
| `-` | Reduce speed 0.25× |
| `=` | Reset speed to 1× |

## Video and capture

| Key | Action |
|-------|--------|
| `S` | Capture frame PNG in ~/Pictures/RPlayer/ |
| `Ctrl+→` | Rotate video +90° |
| `Ctrl+←` | Rotate video -90° |

## Organization

| Key | Action |
|-------|--------|
| `B` | Add marker at current position |

## Files

| Key | Action |
|-------|--------|
| `Ctrl+O` | Open file |
| `Ctrl+U` | Open URL/Stream |

## Drag & Drop

Drag video, audio or .CDG files directly onto the window to open them or add them to the playlist.

---

## Context menus (right click)

### Video area

| Action | Description |
|--------|-------------|
| Play/Pause | Playback toggle |
| Stop | Stop and reset position |
| Skip... | Submenu with +5s, +60s, -5s, -60s |
| Mute/Unmute | Toggle mute |
| Volume up/down | ±5% |
| Volume normalization | Activate/deactivate loudnorm |
| Audio and subtitles | Open submenu with audio panels, subs, EQ, sync |
| Capture frame | Save PNG |
| Picture-in-Picture | Always-on-top floating window |
| aspect ratio | Auto / 4:3 / 16:9 / 21:9 / 1:1 |
| Image and video | Detailed controls (brightness/contrast/zoom/rotation) |
| Trim video... | Open cropping panel |
| Convert format... | Open conversion panel |
| Media information | Shows codec, resolution, bitrate, etc. |

### Playlist item

| Action | Description |
|--------|-------------|
| Play | Play this file |
| Move up | Raise your position on the list |
| Move down | Lower your position on the list |
| Add bookmark | Create marker with current position |
| Copy path | Copy the path to the clipboard |
| Show in explorer | Open the file directory |
| Remove from list | Delete from playlist (not from disk) |
| Clear list | Empty the entire playlist |

### History item

| Action | Description |
|--------|-------------|
| Open | Play the file |
| Show in explorer | Open the directory |
| Copy path | Copy the path to the clipboard |
| Remove from history | Delete only this entry |
| Clear history | Delete all history |

### Marker

| Action | Description |
|--------|-------------|
| Go to this bookmark | Jump into position |
| Rename | Change the label |
| Delete bookmark | Delete the marker |

---

## Usage Notes

- **A-B Loop**: Press `R` once to mark point A, again to mark B, and a third time to clear both. Markers A and B appear as lines in the seekbar.

- **Frame by frame**: Only works with the video paused. Useful for motion analysis, sports, animation.

- **Frame capture**: Saved in `~/Pictures/RPlayer/` (Linux) or `%USERPROFILE%\Pictures\RPlayer\` (Windows). The name includes the timestamp.
