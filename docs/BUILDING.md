# Compilación de RPlayer

## Requisitos comunes

- **Rust** 1.75 o superior
- **Cargo** (incluido con Rust)

Instalar Rust: https://rustup.rs

---

## Linux

### Fedora

```bash
sudo dnf install mpv-libs mpv-libs-devel pkg-config ffmpeg yt-dlp gcc openssl-devel
cargo build --release
./target/release/rplayer
```

### Ubuntu / Debian

```bash
sudo apt update
sudo apt install libmpv-dev pkg-config build-essential ffmpeg yt-dlp libssl-dev
cargo build --release
./target/release/rplayer
```

### Arch Linux

```bash
sudo pacman -S mpv ffmpeg yt-dlp base-devel pkg-config
cargo build --release
./target/release/rplayer
```

---

## Windows

### Con MinGW (cross-compile desde Linux)

```bash
rustup target add x86_64-pc-windows-gnu
sudo dnf install mingw64-gcc   # Fedora
sudo apt install mingw-w64      # Ubuntu
cargo build --release --target x86_64-pc-windows-gnu
```

### Con MSVC (Windows nativo)

1. Instalar Visual Studio Build Tools: https://visualstudio.microsoft.com/downloads/
2. Instalar Rust para Windows: https://rustup.rs
3. Descargar libmpv: https://sourceforge.net/projects/mpv-player-windows/files/libmpv/
4. Configurar `MPV_LIB_PATH` y `PKG_CONFIG_PATH`, o usar `scripts/setup-mpv-windows.ps1`.

Si estás usando `vendor/mpv`, el build script copiará automáticamente `libmpv-2.dll` al directorio `target/debug` o `target/release`.

```powershell
$env:MPV_LIB_PATH = "C:\libs\mpv"
$env:PKG_CONFIG_PATH = "C:\libs\mpv\lib\pkgconfig"
cargo build --release
```

---

## Scripts de build recomendados

El camino más directo es `cargo build --release`.

- `./scripts/build-release.sh [--package]` — build de release en Linux/WSL y empaqueta opcionalmente.
- `./scripts/build-release.ps1 [-Package]` — build de release en Windows y genera instalador Inno Setup si está disponible.
- `./scripts/setup-mpv-windows.ps1 -MpvDllPath <ruta>` — genera `mpv.lib` para builds MSVC con Windows.

---

## Variables de entorno útiles

```bash
RUST_LOG=debug     # Activa logs de debug
RUST_LOG=warn      # Solo warnings (default)
RUST_BACKTRACE=1   # Backtrace en panics
MPV_VERBOSE=1      # Logs de libmpv
```

---

## Actualizaciones in-app (auto/manual + rollback)

- RPlayer permite:
  - chequeo automático al iniciar (configurable),
  - chequeo manual desde Configuración,
  - instalación de update desde la UI.
- Durante instalación, se crea backup del ejecutable actual (`.bak`).
- La versión nueva se valida con `--self-check`; si falla, se restaura automáticamente el backup (fallback).

### Self-check interno

El flag `--self-check` es usado por el flujo de updater para validar que el binario arranca correctamente sin abrir la UI principal:

```bash
rplayer --self-check
```

Si devuelve código de salida distinto de 0, la instalación se considera fallida y se ejecuta rollback.

---

## Estructura del artefacto de distribución

```
rplayer/
├── rplayer (o rplayer.exe)
├── mpv-2.dll           (solo Windows)
├── ffmpeg              (en PATH del sistema)
└── yt-dlp              (en PATH del sistema)
```

---

## Compilación con Docker

Ver [docker-compose.yml](../docker-compose.yml) en la raíz del proyecto.

```bash
# Build Linux release dentro de Docker
docker compose run --rm build-linux

# El binario queda en ./artifacts/
```

---

## CI / GitHub Actions

El archivo `.github/workflows/ci.yml` ejecuta automáticamente las siguientes tareas al subir código o crear un pull request:

- `cargo fmt --all -- --check` — Verifica el formato y estilo del código
- `cargo clippy --all-targets --all-features -- -D warnings` — Ejecuta el linter estático de Rust
- `cargo test --all` — Ejecuta las pruebas unitarias e integradas
- `cargo audit` — Verifica vulnerabilidades conocidas de seguridad en dependencias
- `cargo build --release` — Genera el ejecutable de producción (solo en push a la rama `main`)

### Verificación local previa a subir a GitHub

Para asegurar que tu código pasa las comprobaciones de integración continua, puedes configurar y ejecutar las revisiones de forma local:

1. **Ganchos de pre-commit**:
   ```bash
   pre-commit install
   ```
   Esto ejecutará automáticamente `cargo fmt` y `cargo clippy` antes de cada confirmación de cambios. Puedes correrlos manualmente en cualquier momento:
   ```bash
   pre-commit run --all-files
   ```

2. **Ejecución manual de pruebas**:
   ```bash
   cargo test --all
   ```

---

## Solución de problemas comunes

**Error: `libmpv.so not found`**

```bash
sudo ldconfig
# O en Fedora:
sudo dnf install mpv-libs
```

**Error: `mpv-2.dll not found` (Windows)**
Asegurarse de que `libmpv-2.dll` está en el mismo directorio que el `.exe` (o en `vendor/mpv` y con `RPLAYER_MPV_LIB_DIR` configurado).

**Video no se muestra (pantalla negra)**
Verificar que el sistema tiene soporte OpenGL:

```bash
glxinfo | grep "OpenGL version"
```

**ffmpeg no encontrado**

```bash
which ffmpeg
# Si no está: sudo dnf install ffmpeg
```

---

## Protección del binario

RPlayer aplica tres capas de protección en el build de release:

### 1. Perfil de release endurecido (`Cargo.toml`)

```toml
[profile.release]
strip = true          # Elimina nombres de funciones y paths de código fuente
lto = true            # Link-time optimization: binario más opaco
codegen-units = 1     # Un solo chunk de código
panic = "abort"       # Sin mensajes de panic con nombres de archivos .rs
```

### 2. API keys ofuscadas con `obfstr`

Las keys **no aparecen en texto plano** en el binario. No se pueden extraer con `strings binario`.

Para compilar con tus keys reales:

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

### 3. Instalador Windows comprimido

El instalador generado con Inno Setup usa `lzma2/ultra64 + SolidCompression`.
El `.exe` interno no se puede extraer directamente con 7-Zip ni herramientas similares.

### Qué protege y qué no

| Amenaza                           | Estado                                                              |
| --------------------------------- | ------------------------------------------------------------------- |
| Extraer código fuente del binario | No es posible — el código fuente nunca existe en el binario         |
| Leer API keys con `strings`       | Protegido — obfstr las encripta en compile-time                     |
| Ver paths de archivos .rs         | Protegido — `strip=true` + `panic=abort`                            |
| Desensamblar la lógica general    | Parcial — LTO y `codegen-units=1` lo dificultan, pero no lo impiden |
| Redistribuir el binario           | No protegido en esta versión                                        |
