# RPlayer

Free video and audio player for Linux and Windows. No advertising. No telemetry.

**v0.5.0-alpha** — Rust · egui · libmpv · OpenGL

---

## Quick start

### Simpler way

```bash
cargo build --release
./target/release/rplayer
```

### Linux

Fedora:

```bash
sudo dnf install mpv-libs mpv-libs-devel ffmpeg yt-dlp
cargo build --release
```

Ubuntu/Debian:

```bash
sudo apt install libmpv-dev ffmpeg yt-dlp
cargo build --release
```

### Windows

1. Download `mpv-dev-x86_64.7z` from https://sourceforge.net/projects/mpv-player-windows/files/libmpv/
2. Copy `libmpv-2.dll` to the same directory as `.exe` (or to `vendor/mpv` in development)
3. Install `ffmpeg` and `yt-dlp` and add them to `PATH`
4. `cargo build --release`

---

## Optional build

The `scripts/` scripts are auxiliary.

- `./scripts/build-release.sh [--package]` — release build on Linux/WSL and optional packaging.
- `./scripts/build-release.ps1 [-Package]` — release build on Windows and generates Inno Setup installer if available.
- `./scripts/setup-mpv-windows.ps1 -MpvDllPath <path>` — generates `mpv.lib` for MSVC builds when using Windows libmpv.

For complete details, see `docs/BUILDING.md`.

---

## Optional environment variables

To compile with real API keys, copy `.env.example` to `.env` and define the variables before compiling.

```bash
source .env && cargo build --release
```

> Don't commit `.env` with real values. Use `.env.example` as a template.

---

## Characteristics

- **Formats**: MP4, MKV, AVI, WebM, MP3, FLAC, OGG, AAC, OPUS, CDG and 100+ more
- **Playlist** with M3U/PLS import/export
- **Repeat**: without / one track / the entire list · **Shuffle** · **A-B Loop**
- **Frame by frame** with `.` and `,`
- **Parametric equalizer (PEQ)** 6 filters with presets, preamp and anti-clipping
- **Image controls**: brightness, contrast, saturation, hue, gamma, zoom, rotation, flip, deinterlace and integer scaling
- **Picture-in-Picture** always-on-top
- **Karaoke** .CDG (auto-detects the file along with the MP3)
- **Double subtitles** (two languages ​​simultaneously)
- **Automatic download** of subtitles via OpenSubtitles.org
- **Trim without re-encoding** via ffmpeg
- **Format conversion** (H.264, H.265, VP9, ​​MP3, FLAC, AAC, OGG)
- **Sleep timer**
- **HTTP remote control** in `http://localhost:7890`
- **History** with automatic position resumption
- **Bookmarks**, **notes** and **chapters** per file
- **9 color themes** + custom palette editor
- **Bilingual UI** (Spanish / English)
- **Bug report** from the app (copy technical report and open configurable channel)
- **Updates**: automatic check on startup (optional), manual check and installation with fallback/rollback
- **Contextual menus** in video, playlist, history and bookmarks
- **Scrobbling to Last.fm** and streaming via yt-dlp

---

## Keyboard shortcuts

| Key               | Action                       |
| ------------------- | ---------------------------- |
| `Space`             | Play/Pause                 |
| `→` `←`             | ±5s                          |
| `Shift+→` `Shift+←` | ±60s                         |
| `Ctrl+→` `Ctrl+←`   | Rotate ±90°                   |
| `↑` `↓`             | Volume ±5%                  |
| `M`                 | Mute                    |
| `N` `P`             | Next / Previous         |
| `.` `,`             | Frame forward/backward       |
| `R`                 | A-B Loop (cycle A→B→clear) |
| `S`                 | Capture frame PNG           |
| `B`                 | Add bookmark              |
| `Ctrl+O`            | Open file                |

---

## Context menus

**Right click on video** — Play/pause · Skip · Volume · Audio/Subs · Aspect · Image · Crop · Convert · Media info

**Right click on playlist** — Play · Move up/down · Bookmark · Copy path · Open in browser · Remove · Clear

**Right click on history** — Open · Explorer · Copy path · Remove · Clear

**Right click on bookmark** — Go · Rename · Delete

---

## Updates and fallback

- You can enable or disable "Check for updates on startup" in **Settings**.
- You can run "Check for updates now" manually at any time.
- If an update is installed and the new version fails internal validation, RPlayer automatically restores the previous version (rollback).

---

## Bug report in production

- Menu: **Help → Report bug...**
- Generates a report with technical runtime data (version, OS, playback status, etc.).
- You can copy the report to the clipboard and open a reporting channel (GitHub Issues, form or `mailto`) configurable in **Settings → Bug reports**.

---

## Optional API Keys

| Service           | File                   | Link                                       |
| ------------------ | ---------------------- | ------------------------------------------ |
| Last.fm scrobbling | `src/lastfm.rs`        | https://www.last.fm/api/account/create     |
| OpenSubtitles      | `src/opensubtitles.rs` | https://www.opensubtitles.com/en/consumers |

See [docs/API_KEYS.md](docs/API_KEYS.md) for detailed instructions.

---

## Testing and code quality

Before uploading changes to GitHub, it is advisable to check the formatting, code quality, and run local tests.

### Pre-commit hooks

The project includes pre-commit hooks to automate these reviews. To activate them:

1. Make sure you have `pre-commit` installed (or via Python: `pip install pre-commit`).
2. Install the hooks on your local copy:
   ```bash
   pre-commit install
   ```

This will run format checks (`cargo fmt`) and static analysis (`cargo clippy`) on each commit. You can also run the hooks manually on all files:

```bash
pre-commit run --all-files
```

### Run tests manually

To compile and run local tests:

```bash
cargo test --all
```

---

## Documentation

- [BUILDING.md](docs/BUILDING.md) — Build on Linux, Windows, CI
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) — Code design and structure
- [SHORTCUTS.md](docs/SHORTCUTS.md) — Complete Shortcut Reference
- [API_KEYS.md](docs/API_KEYS.md) — External services configuration
- [website/](website/README.md) — Project web landing (downloads, support, monetization)

---

## License

M.I.T.
