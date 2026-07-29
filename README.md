# ▶ RPlayer — Reproductor de Video y Audio Multimedia Libre

**RPlayer** es un reproductor multimedia moderno, liviano y multiplataforma desarrollado al **100% en Rust** utilizando el motor nativo de **`libmpv`** y la interfaz reactiva de **`Dioxus Desktop`**.

![RPlayer UI](docs/screenshot.png)

---

## 🌟 Características Principales

### 🎬 Reproducción de Video y Audio
- Aceleración por hardware mediante GPU (`vo=gpu,auto` y `hwdec=auto-safe`).
- Soporte para formatos MKV, MP4, AVI, WEBM, MP3, FLAC, OGG, WAV y streams HLS/M3U8.
- Control preciso de velocidad de reproducción (0.25x a 4.0x) y salto por cuadros (*frame step*).
- Bucle de repetición de puntos A-B (`🔂 A-B`).

### 🎛️ Audio y Ecualización
- **Ecualizador PEQ de 6 bandas gráfico** con amplificador (preamp) y presets (Rock, Pop, Bass Boost, Vocal, etc.).
- Selector de pistas de audio y desfase temporal de audio (+/- ms).
- Modo **Karaoke** (supresión vocal de canciones) y ajuste de tono (*pitch shift* +/- 6 semitonos).
- Integración con **Last.fm Scrobbler**.

### 🎨 Video y Subtítulos
- Controles de imagen en tiempo real: Brillo, Contraste, Saturación, Tono (Hue) y Gamma.
- Gestión de subtítulos integrados y externos (`.srt`, `.vtt`, `.ass`) con desfase de tiempo.
- Buscador y descargador automático de subtítulos en línea con **OpenSubtitles**.

### 🔖 Marcadores, Notas y Capítulos
- Marcadores de tiempo persistentes con etiquetas personalizadas por archivo.
- Notas anotadas por segundo y navegación por capítulos nativos de mpv.
- Historial de reproducción con punto de reanudación automática.

### ✂️ Herramientas Multimedia
- **Recorte sin pérdida (*Lossless Video Trim*)** mediante procesos de `ffmpeg`.
- Conversor de formatos de archivo.
- Ficha técnica completa (**MediaInfo**): códecs de video/audio, fps, resolución, bitrate y contenedor.

### 🌐 Red, Remote Control y Personalización
- **Servidor de Control Remoto HTTP** (Puerto 8080) para controlar la reproducción desde el teléfono u otro navegador en la red local.
- **Soporte completo para Modo Oscuro y Modo Claro** con alternancia en tiempo real.
- Temporizador de apagado (*Sleep Timer*) y reproducción automática del siguiente episodio.

---

## 🛠️ Instalación y Compilación

### Dependencias del Sistema

#### Fedora / RHEL:
```bash
sudo dnf install -y https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
sudo dnf install -y --allowerasing ffmpeg libavcodec-freeworld mpv gcc pkg-config mpv-libs mpv-libs-devel openssl-devel yt-dlp javascriptcoregtk4.1-devel webkit2gtk4.1-devel libsoup3-devel libxdo-devel
```

#### Ubuntu / Debian / Linux Mint:
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libmpv-dev libssl-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev libxdo-dev ffmpeg yt-dlp
```

### Compilar y Ejecutar

```bash
# Ejecutar en modo desarrollo
cargo run -- "ruta/a/tu/video.mkv"

# Compilar ejecutable optimizado
cargo build --release
```

---

## 📖 Documentación Completa

- 📐 **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)**: Arquitectura detallada del proyecto y módulos del backend en Rust.
- 🛠️ **[docs/BUILDING.md](docs/BUILDING.md)**: Guía detallada para compilar en Linux, Windows y macOS.
- 🧪 **[docs/TEST_PLAN.md](docs/TEST_PLAN.md)**: Plan maestro de pruebas manuales paso a paso de cada funcionalidad.
- ⌨️ **[docs/SHORTCUTS.md](docs/SHORTCUTS.md)**: Lista completa de atajos de teclado del reproductor.
- 🔑 **[docs/API_KEYS.md](docs/API_KEYS.md)**: Configuración de claves API para OpenSubtitles y Last.fm.

---

## 📄 Licencia

Este proyecto está bajo la Licencia MIT.
