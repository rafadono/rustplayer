# ▶ RPlayer — Free Video and Audio Media Player

**RPlayer** is a modern, lightweight, cross-platform media player built **100% in Rust**, using the native **`libmpv`** engine and the reactive **`Dioxus Desktop`** interface.

![RPlayer UI](docs/screenshot.png)

---

## 🌟 Main Features

### 🎬 Video and Audio Playback
- Hardware acceleration via GPU (`vo=gpu,auto` and `hwdec=auto-safe`).
- Support for MKV, MP4, AVI, WEBM, MP3, FLAC, OGG, WAV, and HLS/M3U8 streams.
- Precise playback speed control (0.25x to 4.0x) and frame-by-frame stepping (*frame step*).
- A-B repeat loop (`🔂 A-B`).

### 🎛️ Audio and Equalization
- **6-band graphic PEQ equalizer** with preamp and presets (Rock, Pop, Bass Boost, Vocal, etc.).
- Audio track selector and audio time offset (+/- ms).
- **Karaoke** mode (vocal suppression) and pitch shift (+/- 6 semitones).
- **Last.fm Scrobbler** integration.

### 🎨 Video and Subtitles
- Real-time image controls: Brightness, Contrast, Saturation, Hue, and Gamma.
- Management of embedded and external subtitles (`.srt`, `.vtt`, `.ass`) with time offset.
- Automatic online subtitle search and download via **OpenSubtitles**.
- Seekbar thumbnail preview: a small preview thumbnail pops up while dragging the seek slider, generated in the background via `ffmpeg` when the file loads.
- Optional on-video performance overlay (FPS, dropped frames, hardware-decode status, buffer seconds), toggled from the Tools modal's Settings tab.

### 🔖 Bookmarks, Notes, History, and Chapters
- Persistent timestamp bookmarks with custom labels per file.
- Timestamped notes per file (**Notes** tab in the Tools modal): add a note at the current position, list/delete notes, and export all notes for a file to `.txt`.
- Chapter navigation with click-to-jump (**Chapters** tab in the Tools modal) for files with embedded chapters (e.g. MKV).
- Playback history with automatic resume point, browsable in the **History** tab in the Tools modal.

### ✂️ Media Tools
- **Lossless video trim** via `ffmpeg` processes.
- File format converter.
- Full technical sheet (**MediaInfo**): video/audio codecs, fps, resolution, bitrate, and container.

### 🌐 Network, Remote Control, and Customization
- **HTTP Remote Control Server** (configurable port, 7890 by default) to control playback from a phone or another browser on the local network.
- **Full Dark Mode and Light Mode support** with real-time switching.
- **Bilingual interface (Spanish/English)**, switchable at runtime via the globe-icon button in the header bar and persisted in the app configuration.
- Real playlist manager with **M3U / M3U8 / PLS import and export**, supporting local files and network stream URLs alike.
- Sleep timer and automatic next-episode playback.

---

## 🛠️ Installation and Build

### System Dependencies

#### Fedora / RHEL:
```bash
sudo dnf install -y https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
sudo dnf install -y --allowerasing ffmpeg libavcodec-freeworld mpv gcc pkg-config mpv-libs mpv-libs-devel openssl-devel yt-dlp javascriptcoregtk4.1-devel webkit2gtk4.1-devel libsoup3-devel libxdo-devel
```

#### Ubuntu / Debian / Linux Mint:
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libmpv-dev libssl-dev \
  libx11-dev libxcursor-dev libxrandr-dev libxi-dev libgl1-mesa-dev \
  libgtk-3-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev libgdk-pixbuf-2.0-dev \
  libxdo-dev ffmpeg yt-dlp
```

### Build and Run

```bash
# Run in development mode
cargo run -- "path/to/your/video.mkv"

# Build an optimized executable
cargo build --release
```

---

## 📖 Full Documentation

- 📐 **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)**: Detailed project architecture and Rust backend modules.
- 🛠️ **[docs/BUILDING.md](docs/BUILDING.md)**: Detailed guide for building on Linux, Windows, and macOS.
- 🧪 **[docs/TEST_PLAN.md](docs/TEST_PLAN.md)**: Master step-by-step manual test plan for every feature.
- ⌨️ **[docs/SHORTCUTS.md](docs/SHORTCUTS.md)**: Full list of player keyboard shortcuts.
- 🔑 **[docs/API_KEYS.md](docs/API_KEYS.md)**: API key configuration for OpenSubtitles and Last.fm.

---

## 📄 License

This project is licensed under the MIT License.
