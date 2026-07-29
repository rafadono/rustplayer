# Compilación e Instalación de RPlayer

## Requisitos comunes

- **Rust** 1.75 o superior (instalar desde https://rustup.rs)
- **Cargo** (incluido con Rust)

---

## 🐧 Linux — Instalación de dependencias por distribución

### Fedora / RHEL / CentOS Stream
```bash
sudo dnf install -y https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
sudo dnf install -y --allowerasing ffmpeg libavcodec-freeworld mpv gcc pkg-config mpv-libs mpv-libs-devel openssl-devel yt-dlp javascriptcoregtk4.1-devel webkit2gtk4.1-devel libsoup3-devel libxdo-devel
```

### Ubuntu / Debian / Linux Mint / Pop!_OS
```bash
sudo apt update
sudo apt install -y build-essential libmpv-dev pkg-config libssl-dev ffmpeg yt-dlp dpkg-deb
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

## ⚙️ Compilación e Instalación en Linux

### Compilación rápida y empaquetado:
```bash
# 1. Clonar y compilar en modo release
./scripts/build-release.sh --package
```

### Métodos de instalación:

#### Opción A: Script instalador universal (`install.sh`)
```bash
# Instalación local en el usuario actual (~/.local/bin y menú de apps):
./installer/linux/install.sh

# O instalación para todo el sistema (/usr/local/bin):
./installer/linux/install.sh --system
```

#### Opción B: Paquete `.deb` (Ubuntu / Debian / Mint)
Si ejecutas `./scripts/build-release.sh --package` en Debian/Ubuntu, se generará el paquete `.deb` en `artifacts/`:
```bash
sudo dpkg -i artifacts/rplayer_0.5.0-alpha_amd64.deb
```

#### Opción C: Tarball distribuible (`.tar.gz`)
El tarball generado en `artifacts/rplayer-0.5.0-alpha-linux-x86_64.tar.gz` contiene el binario, icono, lanzador `.desktop` y script de instalación listo para desplegar en cualquier equipo Linux.

---

## 🪟 Windows — Instalación de dependencias y compilación

### Opción 1: MSVC nativo con PowerShell (Recomendado)

1. **Instalar Herramientas de compilación de Visual Studio (MSVC)**:
   - Descarga [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) e instala el workload *"Desarrollo para el escritorio con C++"*.
2. **Instalar Rust para Windows**:
   - Descarga e instala [rustup-init.exe](https://rustup.rs).
3. **Descargar libmpv para Windows**:
   - Descarga `mpv-dev-x86_64.7z` desde [SourceForge libmpv](https://sourceforge.net/projects/mpv-player-windows/files/libmpv/).
   - Extrae el contenido en una carpeta (ej: `C:\libs\mpv`).
4. **Instalar `ffmpeg` y `yt-dlp`**:
   - Instala mediante `winget`:
     ```powershell
     winget install Gyan.FFmpeg
     winget install yt-dlp.yt-dlp
     ```
5. **Configurar variables y compilar**:
   - Usa el script auxiliar en PowerShell:
     ```powershell
     .\scripts\build-release.ps1 -Package
     ```
   - *Si tienes Inno Setup instalado, generará el instalador exe en `artifacts/`*.

---

### Opción 2: Cross-compilar para Windows desde Linux (con MinGW)

```bash
rustup target add x86_64-pc-windows-gnu

# En Fedora:
sudo dnf install -y mingw64-gcc

# En Ubuntu/Debian:
sudo apt install -y mingw-w64

# Compilar ejecutable .exe
cargo build --release --target x86_64-pc-windows-gnu
```

---

## Recommended build scripts

The most direct path is `cargo build --release`.

- `./scripts/build-release.sh [--package]` — release build on Linux/WSL and optional packaging.
- `./scripts/build-release.ps1 [-Package]` — release build on Windows and generates Inno Setup installer if available.
- `./scripts/setup-mpv-windows.ps1 -MpvDllPath <ruta>` — generates `mpv.lib` for MSVC builds with Windows.

---

## Useful environment variables

```bash
RUST_LOG=debug     # Activa logs de debug
RUST_LOG=warn      # Solo warnings (default)
RUST_BACKTRACE=1   # Backtrace en panics
MPV_VERBOSE=1      # Logs de libmpv
```

---

## In-app updates (auto/manual + rollback)

- RPlayer allows:
  - automatic check on startup (configurable),
  - manual check from Settings,
  - update installation from the UI.
- During installation, a backup of the current executable (`.bak`) is created.
- The new version is validated with `--self-check`; If it fails, the backup is automatically restored (fallback).

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
├── rplayer (o rplayer.exe)
├── mpv-2.dll           (solo Windows)
├── ffmpeg              (en PATH del sistema)
└── yt-dlp              (en PATH del sistema)
```

---

## Compilation with Docker

See [docker-compose.yml](../docker-compose.yml) in the project root.

```bash
# Build Linux release dentro de Docker
docker compose run --rm build-linux

# El binario queda en ./artifacts/
```

---

## CI/GitHub Actions

The `.github/workflows/ci.yml` file automatically executes the following tasks when you upload code or create a pull request:

- `cargo fmt --all -- --check` — Check the code format and style
- `cargo clippy --all-targets --all-features -- -D warnings` — Run the Rust static linter
- `cargo test --all` — Runs unit and integrated tests
- `cargo audit` — Checks for known security vulnerabilities in dependencies
- `cargo build --release` — Generate the production executable (only in push to branch `main`)

### Local verification prior to uploading to GitHub

To ensure that your code passes continuous integration checks, you can configure and run the checks locally:

1. **Pre-commit hooks**:
   ```bash
   pre-commit install
   ```
This will automatically run `cargo fmt` and `cargo clippy` before each change commit. You can run them manually at any time:
   ```bash
   pre-commit run --all-files
   ```

2. **Manual test execution**:
   ```bash
   cargo test --all
   ```

---

## Common Problem Solving

**Error: `libmpv.so not found`**

```bash
sudo ldconfig
# O en Fedora:
sudo dnf install mpv-libs
```

**Error: `mpv-2.dll not found` (Windows)**
Make sure `libmpv-2.dll` is in the same directory as `.exe` (or in `vendor/mpv` and with `RPLAYER_MPV_LIB_DIR` configured).

**Video not displayed (black screen)**
Verify that the system has OpenGL support:

```bash
glxinfo | grep "OpenGL version"
```

**ffmpeg not found**

```bash
which ffmpeg
# Si no está: sudo dnf install ffmpeg
```

---

## Binary protection

RPlayer applies three layers of protection in the release build:

### 1. Hardened release profile (`Cargo.toml`)

```toml
[profile.release]
strip = true          # Elimina nombres de funciones y paths de código fuente
lto = true            # Link-time optimization: binario más opaco
codegen-units = 1     # Un solo chunk de código
panic = "abort"       # Sin mensajes de panic con nombres de archivos .rs
```

### 2. API keys obfuscated with `obfstr`

The keys **do not appear in plain text** in the binary. They cannot be extracted with `strings binario`.

To compile with your real keys:

```bash
# Linux / macOS
export RUSTPLAYER_LASTFM_KEY="tu_key"
export RUSTPLAYER_OPENSUBS_KEY="tu_key"
cargo build --release

# Windows PowerShell
$env:RUSTPLAYER_LASTFM_KEY="tu_key"
$env:RUSTPLAYER_OPENSUBS_KEY="tu_key"
cargo build --release
```

### 3. Compressed Windows Installer

The installer generated with Inno Setup uses `lzma2/ultra64 + SolidCompression`.
The internal `.exe` cannot be extracted directly with 7-Zip or similar tools.

### What protects and what doesn't

| Threat                           | State                                                              |
| --------------------------------- | ------------------------------------------------------------------- |
| Extract source code from binary | Not possible — the source code never exists in the binary         |
| Read API keys with `strings`       | Protected — obfstr encrypts them at compile-time                     |
| View .rs file paths         | Protected — `strip=true` + `panic=abort`                            |
| Disassemble the general logic    | Partial — LTO and `codegen-units=1` make it difficult, but do not prevent it |
| Redistribute the binary           | Not protected in this version                                        |
