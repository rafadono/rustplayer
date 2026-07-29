# Building and Installing RPlayer

## Common Requirements

- **Rust** 1.75 or higher (install from https://rustup.rs)
- **Cargo** (included with Rust)

---

## 🐧 Linux — Installing dependencies per distribution

### Fedora / RHEL / CentOS Stream
```bash
sudo dnf install -y https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
sudo dnf install -y --allowerasing ffmpeg libavcodec-freeworld mpv gcc pkg-config mpv-libs mpv-libs-devel openssl-devel yt-dlp javascriptcoregtk4.1-devel webkit2gtk4.1-devel libsoup3-devel libxdo-devel
```

### Ubuntu / Debian / Linux Mint / Pop!_OS
```bash
sudo apt update
sudo apt install -y build-essential libmpv-dev pkg-config libssl-dev \
  libx11-dev libxcursor-dev libxrandr-dev libxi-dev libgl1-mesa-dev \
  libgtk-3-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev libgdk-pixbuf-2.0-dev \
  libxdo-dev ffmpeg yt-dlp dpkg-deb
```

### Arch Linux / Manjaro / EndeavourOS
```bash
sudo pacman -S --needed base-devel mpv ffmpeg yt-dlp pkgconf openssl
```

### openSUSE (Tumbleweed / Leap)
```bash
sudo zypper install -y gcc pkg-config mpv-devel libopenssl-devel ffmpeg yt-dlp
```

### Alpine Linux
```bash
sudo apk add build-base pkgconf mpv-dev openssl-dev ffmpeg yt-dlp
```

---

## ⚙️ Building and Installing on Linux

### Quick build and packaging:
```bash
# 1. Clone and build in release mode
./scripts/build-release.sh --package
```

### Installation methods:

#### Option A: Universal installer script (`install.sh`)
```bash
# Local install for the current user (~/.local/bin and app menu):
./installer/linux/install.sh

# Or system-wide install (/usr/local/bin):
./installer/linux/install.sh --system
```

#### Option B: `.deb` package (Ubuntu / Debian / Mint)
Running `./scripts/build-release.sh --package` on Debian/Ubuntu generates the `.deb` package in `artifacts/`:
```bash
sudo dpkg -i artifacts/rplayer_0.5.0-alpha_amd64.deb
```

#### Option C: Distributable tarball (`.tar.gz`)
The tarball generated at `artifacts/rplayer-0.5.0-alpha-linux-x86_64.tar.gz` contains the binary, icon, `.desktop` launcher, and an install script ready to deploy on any Linux machine.

---

## 🪟 Windows — Installing dependencies and building

### Option 1: Native MSVC with PowerShell (Recommended)

1. **Install Visual Studio Build Tools (MSVC)**:
   - Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) and install the *"Desktop development with C++"* workload.
2. **Install Rust for Windows**:
   - Download and run [rustup-init.exe](https://rustup.rs).
3. **Download libmpv for Windows**:
   - Download `mpv-dev-x86_64.7z` from [SourceForge libmpv](https://sourceforge.net/projects/mpv-player-windows/files/libmpv/).
   - Extract its contents into a folder (e.g. `C:\libs\mpv`).
   - Generate `mpv.lib` for MSVC linking and set the required environment variables:
     ```powershell
     .\scripts\build-release.ps1 -SetupMpv "C:\libs\mpv\mpv-2.dll"
     ```
4. **Install `ffmpeg` and `yt-dlp`**:
   - Install via `winget`:
     ```powershell
     winget install Gyan.FFmpeg
     winget install yt-dlp.yt-dlp
     ```
5. **Set variables and build**:
   - Use the PowerShell helper script:
     ```powershell
     .\scripts\build-release.ps1 -Package
     ```
   - *If Inno Setup is installed, it will generate the exe installer in `artifacts/`*.

---

### Option 2: Cross-compile for Windows from Linux (with MinGW)

```bash
rustup target add x86_64-pc-windows-gnu

# On Fedora:
sudo dnf install -y mingw64-gcc

# On Ubuntu/Debian:
sudo apt install -y mingw-w64

# Build the .exe
cargo build --release --target x86_64-pc-windows-gnu
```

---

## Recommended build scripts

The most direct path is `cargo build --release`.

- `./scripts/build-release.sh [--package]` — release build on Linux/WSL and optional packaging.
- `./scripts/build-release.ps1 [-Package]` — release build on Windows and generates Inno Setup installer if available.
- `./scripts/build-release.ps1 -SetupMpv <path-to-mpv-2.dll>` — one-time dev setup: generates `mpv.lib` for MSVC builds with Windows, then exits without building.

---

## Useful environment variables

```bash
RUST_LOG=debug     # Enable debug logs
RUST_LOG=warn      # Warnings only (default)
RUST_BACKTRACE=1   # Backtrace on panics
MPV_VERBOSE=1      # libmpv logs
```

---

## In-app updates (auto/manual + rollback)

- RPlayer allows:
  - automatic check on startup (configurable),
  - manual check from Settings,
  - update installation from the UI.
- During installation, a backup of the current executable (`.bak`) is created.
- The new version is validated with `--self-check`; if it fails, the backup is automatically restored (fallback).

### Internal self-check

The `--self-check` flag is used by the updater flow to validate that the binary boots correctly without opening the main UI:

```bash
rplayer --self-check
```

If it returns an exit code other than 0, the installation is considered failed and a rollback is executed.

---

## Distribution artifact structure

```
rplayer/
├── rplayer (or rplayer.exe)
├── mpv-2.dll           (Windows only)
├── ffmpeg              (in system PATH)
└── yt-dlp              (in system PATH)
```

---

## Compilation with Docker

See [docker-compose.yml](../docker-compose.yml) in the project root.

```bash
# Build the Linux release inside Docker
docker compose run --rm build-linux

# The binary ends up in ./artifacts/
```

---

## CI/GitHub Actions

The `.github/workflows/ci.yml` file automatically runs the following tasks on push or pull request:

- `cargo fmt --all -- --check` — Check the code format and style
- `cargo clippy --all-targets --all-features -- -D warnings` — Run the Rust static linter
- `cargo test --all` — Runs unit and integration tests
- `cargo audit` — Checks for known security vulnerabilities in dependencies
- `cargo build --release` — Generate the production executable (only on push to the `main` branch)

### Local verification before pushing to GitHub

To make sure your code passes continuous integration checks, you can set up and run the checks locally:

1. **Pre-commit hooks**:
   ```bash
   pre-commit install
   ```
This will automatically run `cargo fmt` and `cargo clippy` before each commit. You can also run them manually at any time:
   ```bash
   pre-commit run --all-files
   ```

2. **Manual test run**:
   ```bash
   cargo test --all
   ```

---

## Common Problem Solving

**Error: `libmpv.so not found`**

```bash
sudo ldconfig
# Or on Fedora:
sudo dnf install mpv-libs
```

**Error: `mpv-2.dll not found` (Windows)**
Make sure `libmpv-2.dll` is in the same directory as the `.exe` (or in `vendor/mpv` with `RPLAYER_MPV_LIB_DIR` configured).

**Video not displayed (black screen)**
Verify that the system has OpenGL support:

```bash
glxinfo | grep "OpenGL version"
```

**ffmpeg not found**

```bash
which ffmpeg
# If missing: sudo dnf install ffmpeg
```

**Video freezes or disappears when using the progress bar (Wayland)**
mpv's video embedding uses X11's `wid` API (via `gdkx11`); there is no native
Wayland support. On Wayland sessions (e.g. Fedora Workstation with GNOME,
which defaults to Wayland), RPlayer forces `GDK_BACKEND=x11` on startup so
GTK uses XWayland (present by default on Fedora, Ubuntu, and Debian) and
always gets a valid `wid`. If you need to force a different GDK backend for
some reason, set the `GDK_BACKEND` environment variable before running
`rplayer` — RPlayer respects the value if it's already set.

---

## Binary protection

RPlayer applies three layers of protection in the release build:

### 1. Hardened release profile (`Cargo.toml`)

```toml
[profile.release]
strip = true          # Removes function names and source file paths
lto = true            # Link-time optimization: a more opaque binary
codegen-units = 1     # A single code chunk
panic = "abort"       # No panic messages with .rs file names
```

### 2. API keys obfuscated with `obfstr`

The keys **do not appear in plain text** in the binary. They cannot be extracted with `strings binary`.

To build with your real keys:

```bash
# Linux / macOS
export RUSTPLAYER_LASTFM_KEY="your_key"
export RUSTPLAYER_OPENSUBS_KEY="your_key"
cargo build --release

# Windows PowerShell
$env:RUSTPLAYER_LASTFM_KEY="your_key"
$env:RUSTPLAYER_OPENSUBS_KEY="your_key"
cargo build --release
```

### 3. Compressed Windows Installer

The installer generated with Inno Setup uses `lzma2/ultra64 + SolidCompression`.
The internal `.exe` cannot be extracted directly with 7-Zip or similar tools.

### What is protected and what isn't

| Threat                           | State                                                              |
| --------------------------------- | ------------------------------------------------------------------- |
| Extract source code from binary | Not possible — the source code never exists in the binary         |
| Read API keys with `strings`       | Protected — obfstr encrypts them at compile-time                     |
| View .rs file paths         | Protected — `strip=true` + `panic=abort`                            |
| Disassemble the general logic    | Partial — LTO and `codegen-units=1` make it difficult, but do not prevent it |
| Redistribute the binary           | Not protected in this version                                        |
