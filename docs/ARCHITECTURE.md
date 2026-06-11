# Arquitectura de RPlayer

## Visión general

RPlayer está organizado en dos capas bien separadas: **backend** (lógica de reproducción, persistencia, integraciones) y **UI** (todos los paneles de egui). La comunicación entre ambas fluye a través de `app.rs`, que actúa como orquestador.

```
┌─────────────────────────────────────────────────┐
│                    app.rs                        │
│           (orquestador principal)                │
│  ┌──────────────┐    ┌─────────────────────────┐ │
│  │  Backend     │    │       UI (egui)         │ │
│  │  modules     │◄──►│       panels            │ │
│  └──────────────┘    └─────────────────────────┘ │
└─────────────────────────────────────────────────┘
                    │
         ┌──────────▼───────────┐
         │       libmpv         │
         │  (reproducción,      │
         │   render OpenGL)     │
         └──────────────────────┘
```

---

## Módulos backend

### `player.rs`
Wrapper thread-safe sobre `libmpv2::Mpv`. El estado (`PlayerState`) vive en un `Arc<Mutex<PlayerState>>` compartido entre el thread del player y el thread de UI.

**API pública clave:**
- `open(path)` — Carga y reproduce un archivo
- `toggle_pause()`, `stop()`, `seek_relative()`, `seek_absolute()`
- `set_volume()`, `set_speed()`, `set_aspect_ratio()`
- `frame_step()`, `frame_back_step()`
- `set_audio_delay()`, `set_sub_delay()`
- `apply_image_controls()` — Aplica todo el `ImageControls` de una vez
- `drain_events()` — Drena eventos mpv para procesarlos en el loop principal

**Eventos (`PlayerEvent`):**
- `FileLoaded { title, duration }` — Archivo listo para reproducir
- `EndOfFile` — Fin de pista
- `PositionChanged(f64)` — Posición actualizada (cada ~250ms)
- `Error(String)`

### `renderer.rs`
Integra la libmpv Render API con el backend glow (OpenGL) de eframe. Para evitar superposiciones con los paneles de egui y la barra de controles inferior, realiza un renderizado offscreen: genera un FBO y una textura intermedios donde `libmpv` dibuja el video, y luego realiza un blitting (`glBlitFramebuffer`) al FBO destino principal reubicando el rectángulo en el espacio físico del panel central. Genera un `egui::PaintCallback` que se inserta en el painter de egui. El método `destroy_gl_resources` limpia estos recursos OpenGL al cerrar la aplicación.

### `config.rs`
Configuración persistente en JSON. Incluye: volumen, velocidad, aspect ratio, modo de repetición, shuffle, delays de audio/sub, `ImageControls`, `ThemeColors`, `Equalizer`, configuración de Last.fm y del servidor remoto.

**Ruta:** `~/.config/rplayer/config.json`

### `playlist.rs`
Lista de pistas con soporte de M3U/M3U8/PLS. Métodos: `add`, `remove`, `move_up`, `move_down`, `clear`, `next`, `prev`, `set_current_by_path`, `import_m3u`, `export_m3u`.

### `history.rs`
Historial de reproducción. Guarda posición, duración y fecha de reproducción por archivo. Método `should_resume()` retorna true si la posición guardada es > 5s y < 97% de la duración.

**Ruta:** `~/.local/share/rplayer/history.json`

### `bookmarks.rs`
Marcadores con timestamp y etiqueta editable por archivo. Persistidos como `HashMap<String, Vec<Bookmark>>`.

### `notes.rs`
Notas de texto libre con timestamp por archivo. Exportables como `.txt`.

### `chapters.rs`
Parser del JSON `chapter-list` de mpv. Función `current_chapter(chapters, position)` retorna el índice del capítulo activo.

### `image_controls.rs`
Agrupa todas las propiedades de imagen en una sola estructura. El método `apply(&self, mpv)` envía todas las propiedades a libmpv de una vez: `brightness`, `contrast`, `saturation`, `hue`, `gamma`, `video-zoom`, `video-pan-x/y`, `video-rotate`, `deinterlace`, `vf` (para flip).

### `equalizer.rs`
EQ 10 bandas ISO (31Hz–16kHz). Genera el filtro `af=equalizer=f=...` para mpv. Presets: Plano, Bass, Vocal, Cine, Rock.

### `ab_repeat.rs`
Repetición A-B. Cicla entre tres estados (A sin marcar → A marcado → A+B marcados → limpiar) usando las propiedades `ab-loop-a` y `ab-loop-b` de mpv.

### `sleep_timer.rs`
Timer countdown. Tick en cada frame. Dispara una de tres acciones: `Pause`, `Stop`, o `Quit`.

### `media_info.rs`
Extrae metadatos del archivo cargado parseando `track-list` (JSON de mpv) y leyendo propiedades como `width`, `height`, `fps`, `video-bitrate`, `audio-bitrate`.

### `thumbnail.rs`
Genera 20 frames con `ffmpeg` en un thread background. Los almacena en `/tmp/rplayer_thumbs/`. Método `nearest(time)` retorna el path del frame más cercano a una posición dada.

### `streaming.rs`
Clasifica URLs: `DirectStream` (http/rtmp/m3u8), `YtDlp` (YouTube/Vimeo/Twitch), `Unknown`. Detecta yt-dlp y ffmpeg en PATH.

### `trim.rs`
Recorte sin re-encodear. Ejecuta `ffmpeg -ss START -i INPUT -t DURATION -c copy OUTPUT` en un thread separado. Estado comunicado via `crossbeam-channel`.

### `converter.rs`
Conversión de formatos con 8 presets (H.264, H.265, VP9, MP3×2, FLAC, AAC, OGG). Cada preset define los args de ffmpeg y la extensión de salida.

### `lastfm.rs`
Scrobbling a Last.fm via Mobile Session API. Autenticación con MD5. `ScrobbleTracker` acumula tiempo reproducido excluyendo pausas y seeks grandes.

### `remote.rs`
Servidor HTTP `tiny_http` en `127.0.0.1:7890`. Endpoints REST para control de reproducción. Página HTML embebida con botones y slider de volumen.

### `opensubtitles.rs`
Búsqueda y descarga de subtítulos via API REST de OpenSubtitles v2. Ejecución en threads separados con canal de estado.

### `karaoke.rs`
Soporte de archivos .CDG. Detecta si existe un `.cdg` junto al `.mp3` con el mismo nombre base. libmpv los reproduce automáticamente cuando están juntos.

### `theme_manager.rs`
9 paletas predefinidas + editor custom. `ThemeColors::apply(ctx)` sobreescribe los visuals de egui (`panel_fill`, `widgets.inactive.bg_fill`, etc.) y actualiza la estructura global mutable `ACTIVE_THEME` en `ui/theme`. Esto permite que todos los componentes y textos que usan constantes de tema estáticos (mediante el wrapper proxy `DynamicColor`) cambien y se adapten de forma dinámica y con el contraste correcto según el tema elegido.

### `donation.rs`
Banner de donación con enlace a Patreon. Dismissable y el estado persiste en config.

---

## UI (src/ui/)

Todos los paneles son structs simples que reciben referencias mutables y retornan acciones o valores.

### Convención de retorno

Los paneles retornan un valor semántico o `Option<T>`:
- `playlist_panel::draw_playlist` → `(Option<usize>, ContextAction)`
- `HistoryPanel::show` → `(Option<PathBuf>, ContextAction)`
- `BookmarksPanel::show` → `(Option<f64>, ContextAction)`
- `ImageControlsPanel::show` → `bool` (changed)
- `ThemePanel::show` → `bool` (changed)
- `KaraokePanel::show` → `Option<PathBuf>` (archivo a abrir)
- `OpenSubtitlesPanel::show` → `Option<PathBuf>` (subtítulo descargado)

### `context_menu.rs`
Menús contextuales de clic derecho usando `response.context_menu(|ui| {...})` de egui.

- `video_context_menu` — sobre el área de video
- `playlist_item_context_menu` — sobre cada ítem de la lista
- `history_item_context_menu` — sobre cada ítem del historial
- `bookmark_context_menu` — sobre cada marcador

Todos retornan `ContextAction`, manejado en `app.rs::handle_context_action()`.

### `menu.rs`
Menú superior organizado por área temática. Usa `MenuState` para pasar todos los flags de visibilidad y estado activo. Retorna `MenuAction`, manejado en `app.rs::handle_menu_action()`.

**Estructura del menú:**
1. **Archivo** — abrir, importar/exportar lista, salir
2. **Reproducción** — play/pausa, siguiente/anterior, saltar, frame a frame, repetición, velocidad
3. **Audio** — silenciar, volumen, normalización, paneles de audio
4. **Video** — captura, PiP, aspecto, rotación, flip, deinterlace
5. **Subtítulos** — pistas, descarga
6. **Herramientas** — recortar, convertir, sleep timer, control remoto
7. **Vista** — todos los paneles laterales
8. **?** — acerca de

### `controls.rs`
Barra inferior de controles con diseño estético premium.
- **Seekbar Dinámica:** Se expande responsivamente (de 4px a 6px) en hover, ocultando el círculo indicador (thumb) en estado inactivo. Dibuja el rango A-B con `egui::Painter::rect_filled` y marcadores visuales. Admite interacción directa (clic y arrastre) para buscar posiciones.
- **Botón de Play Destacado:** Se renderiza como un círculo del color de acento del tema activo, con contraste inteligente automático (icono blanco/negro) según la luminancia del color.
- **Botones Sin Bordes (Borderless):** Utiliza botones vectoriales transparentes que muestran un fondo circular sutil al pasar el cursor.
- **Flujo de Acciones:** Retorna un enum `ControlsAction` para propagar los eventos de interacción (prev, next, toggle repeat/shuffle) hacia `app.rs`, logrando que la interfaz sea totalmente interactiva.

---

## Flujo de datos en `app.rs`

Cada frame de UI:

```
update() {
    1. process_player_events()    // eventos mpv → acciones (FileLoaded, EndOfFile)
    2. process_remote_commands()  // comandos HTTP → player
    3. poll_jobs()                // trim/convert jobs → success/error msg
    4. tick_scrobble()            // acumula segundos reproducidos
    5. tick_sleep_timer()         // dispara si llegó el tiempo
    6. handle_keyboard()          // teclas → acciones
    7. Dibuja menú superior (con indicador A-B condicional) → handle_menu_action()
    8. Dibuja controles inferiores
    10. draw_side_panel()          // panel lateral activo
    11. draw_central()             // video + context_menu
    12. draw_floating_windows()    // ventanas modales
    13. request_repaint_after(33ms) si reproduciendo
}
```

---

## Threads en ejecución

| Thread | Responsabilidad |
|--------|----------------|
| Thread principal (UI) | egui + input + lógica de app |
| libmpv internal | Decodificación de video/audio |
| OpenGL render | Render de frames via mpv render API |
| `thumbnail-gen` | Genera miniaturas con ffmpeg |
| `trim-job` | Ejecuta ffmpeg para recortar |
| `convert-job` | Ejecuta ffmpeg para convertir |
| `opensubs-search` | Busca subtítulos en OpenSubtitles |
| `opensubs-download` | Descarga el subtítulo |
| `lastfm-*` | Peticiones HTTP a Last.fm |
| `remote-server` | Servidor HTTP tiny_http |
