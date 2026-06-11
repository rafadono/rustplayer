# RPlayer

Reproductor de video y audio libre para Linux y Windows. Sin publicidad. Sin telemetría.

**v0.5.0-alpha** — Rust · egui · libmpv · OpenGL

---

## Inicio rápido

### Camino más simple

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

Ubuntu / Debian:

```bash
sudo apt install libmpv-dev ffmpeg yt-dlp
cargo build --release
```

### Windows

1. Descargar `mpv-dev-x86_64.7z` de https://sourceforge.net/projects/mpv-player-windows/files/libmpv/
2. Copiar `libmpv-2.dll` al mismo directorio que el `.exe` (o a `vendor/mpv` en desarrollo)
3. Instalar `ffmpeg` y `yt-dlp` y agregarlos al `PATH`
4. `cargo build --release`

---

## Build opcional

Los scripts de `scripts/` son auxiliares.

- `./scripts/build-release.sh [--package]` — build de release en Linux/WSL y empaqueta opcionalmente.
- `./scripts/build-release.ps1 [-Package]` — build de release en Windows y genera instalador Inno Setup si está disponible.
- `./scripts/setup-mpv-windows.ps1 -MpvDllPath <ruta>` — genera `mpv.lib` para builds MSVC cuando usas libmpv de Windows.

Para detalles completos, consulta `docs/BUILDING.md`.

---

## Variables de entorno opcionales

Para compilar con API keys reales, copia `.env.example` a `.env` y define las variables antes de compilar.

```bash
source .env && cargo build --release
```

> No comitees `.env` con valores reales. Usa `.env.example` como plantilla.

---

## Características

- **Formatos**: MP4, MKV, AVI, WebM, MP3, FLAC, OGG, AAC, OPUS, CDG y 100+ más
- **Playlist** con importación/exportación M3U/PLS
- **Repetición**: sin / una pista / toda la lista · **Shuffle** · **A-B Loop**
- **Frame a frame** con `.` y `,`
- **Ecualizador paramétrico (PEQ)** 6 filtros con presets, preamp y anti-clipping
- **Controles de imagen**: brillo, contraste, saturación, matiz, gamma, zoom, rotación, flip, deinterlace e integer scaling
- **Picture-in-Picture** always-on-top
- **Karaoke** .CDG (auto-detecta el archivo junto al MP3)
- **Subtítulos dobles** (dos idiomas simultáneos)
- **Descarga automática** de subtítulos via OpenSubtitles.org
- **Recorte sin re-encodear** via ffmpeg
- **Conversión de formatos** (H.264, H.265, VP9, MP3, FLAC, AAC, OGG)
- **Sleep timer**
- **Control remoto HTTP** en `http://localhost:7890`
- **Historial** con reanudación automática de posición
- **Marcadores**, **notas** y **capítulos** por archivo
- **9 temas de color** + editor custom de paleta
- **UI bilingüe** (Español / English)
- **Reporte de bugs** desde la app (copiar reporte técnico y abrir canal configurable)
- **Actualizaciones**: chequeo automático al iniciar (opcional), chequeo manual e instalación con fallback/rollback
- **Menús contextuales** en video, playlist, historial y marcadores
- **Scrobbling a Last.fm** y streaming via yt-dlp

---

## Atajos de teclado

| Tecla               | Acción                       |
| ------------------- | ---------------------------- |
| `Espacio`           | Play / Pausa                 |
| `→` `←`             | ±5s                          |
| `Shift+→` `Shift+←` | ±60s                         |
| `Ctrl+→` `Ctrl+←`   | Rotar ±90°                   |
| `↑` `↓`             | Volumen ±5%                  |
| `M`                 | Silenciar                    |
| `N` `P`             | Siguiente / Anterior         |
| `.` `,`             | Frame adelante / atrás       |
| `R`                 | A-B Loop (cicla A→B→limpiar) |
| `S`                 | Capturar frame PNG           |
| `B`                 | Añadir marcador              |
| `Ctrl+O`            | Abrir archivo                |

---

## Menús contextuales

**Clic derecho en el video** — Play/pausa · Saltar · Volumen · Audio/Subs · Aspecto · Imagen · Recortar · Convertir · Info de medios

**Clic derecho en playlist** — Reproducir · Mover arriba/abajo · Marcador · Copiar ruta · Abrir en explorador · Quitar · Limpiar

**Clic derecho en historial** — Abrir · Explorador · Copiar ruta · Quitar · Limpiar

**Clic derecho en marcador** — Ir · Renombrar · Eliminar

---

## Actualizaciones y fallback

- Puedes activar o desactivar `Buscar actualizaciones al iniciar` en **Configuración**.
- Puedes ejecutar `Buscar actualizaciones ahora` manualmente en cualquier momento.
- Si se instala una actualización y la nueva versión falla la validación interna, RPlayer restaura automáticamente la versión anterior (rollback).

---

## Reporte de bugs en producción

- Menú: **Ayuda → Reportar bug...**
- Genera un reporte con datos técnicos de runtime (versión, SO, estado de reproducción, etc.).
- Puedes copiar el reporte al portapapeles y abrir un canal de reporte (GitHub Issues, formulario o `mailto`) configurable en **Configuración → Reportes de bugs**.

---

## API Keys opcionales

| Servicio           | Archivo                | Link                                       |
| ------------------ | ---------------------- | ------------------------------------------ |
| Last.fm scrobbling | `src/lastfm.rs`        | https://www.last.fm/api/account/create     |
| OpenSubtitles      | `src/opensubtitles.rs` | https://www.opensubtitles.com/en/consumers |

Ver [docs/API_KEYS.md](docs/API_KEYS.md) para instrucciones detalladas.

---

## Pruebas y calidad de código

Antes de subir cambios a GitHub, es recomendable verificar el formato, la calidad del código y ejecutar las pruebas locales.

### Ganchos de pre-commit

El proyecto incluye ganchos (hooks) de pre-commit para automatizar estas revisiones. Para activarlos:

1. Asegúrate de tener instalado `pre-commit` (o mediante Python: `pip install pre-commit`).
2. Instala los ganchos en tu copia local:
   ```bash
   pre-commit install
   ```

Esto ejecutará comprobaciones de formato (`cargo fmt`) y análisis estático (`cargo clippy`) en cada commit. También puedes correr los ganchos manualmente sobre todos los archivos:

```bash
pre-commit run --all-files
```

### Ejecutar pruebas manualmente

Para compilar y ejecutar las pruebas locales:

```bash
cargo test --all
```

---

## Documentación

- [BUILDING.md](docs/BUILDING.md) — Compilación en Linux, Windows, CI
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) — Diseño y estructura del código
- [SHORTCUTS.md](docs/SHORTCUTS.md) — Referencia completa de atajos
- [API_KEYS.md](docs/API_KEYS.md) — Configuración de servicios externos
- [website/](website/README.md) — Landing web del proyecto (descargas, soporte, monetización)

---

## Licencia

MIT
