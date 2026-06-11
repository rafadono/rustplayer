# RPlayer architecture

## Overview

RPlayer is organized into two well-separated layers: **backend** (playback logic, persistence, integrations) and **UI** (all egui panels). Communication between the two flows through `app.rs`, which acts as an orchestrator.

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

## Backend modules

### `player.rs`
Wrapper thread-safe over `libmpv2::Mpv`. The state (`PlayerState`) lives in a `Arc<Mutex<PlayerState>>` shared between the player thread and the UI thread.

**Public API Key:**
- `open(path)` — Load and play a file
- `toggle_pause()`, `stop()`, `seek_relative()`, `seek_absolute()`
- `set_volume()`, `set_speed()`, `set_aspect_ratio()`
- `frame_step()`, `frame_back_step()`
- `set_audio_delay()`, `set_sub_delay()`
- `apply_image_controls()` — Apply all of `ImageControls` at once
- `drain_events()` — Drains mpv events to process them in the main loop

**Events (`PlayerEvent`):**
- `FileLoaded { title, duration }` — File ready to play
- `EndOfFile` — End of track
- `PositionChanged(f64)` — Position updated (every ~250ms)
- `Error(String)`

### `renderer.rs`
Integrates the libmpv Render API with the eframe glow (OpenGL) backend. To avoid overlaps with the egui panels and the lower control bar, perform an offscreen rendering: generate an intermediate FBO and texture where `libmpv` draws the video, and then blit (`glBlitFramebuffer`) to the main destination FBO by relocating the rectangle in the physical space of the central panel. Generates a `egui::PaintCallback` that is inserted into the egui painter. The `destroy_gl_resources` method cleans up these OpenGL resources when you close the application.

### `config.rs`
Persistent configuration in JSON. Includes: volume, speed, aspect ratio, repeat mode, shuffle, audio/sub delays, `ImageControls`, `ThemeColors`, `Equalizer`, Last.fm and remote server settings.

**Path:** `~/.config/rplayer/config.json`

### `playlist.rs`
Track list with M3U/M3U8/PLS support. Methods: `add`, `remove`, `move_up`, `move_down`, `clear`, `next`, `prev`, `set_current_by_path`, `import_m3u`, `export_m3u`.

### `history.rs`
Playback history. Saves position, duration and playback date per file. Method `should_resume()` returns true if the saved position is > 5s and < 97% of the duration.

**Path:** `~/.local/share/rplayer/history.json`

### `bookmarks.rs`
Bookmarks with timestamp and editable label per file. Persistent as `HashMap<String, Vec<Bookmark>>`.

### `notes.rs`
Free text notes with timestamp per file. Exportable as `.txt`.

### `chapters.rs`
mpv JSON `chapter-list` Parser. Function `current_chapter(chapters, position)` returns the index of the active chapter.

### `image_controls.rs`
Groups all image properties into a single structure. The `apply(&self, mpv)` method sends all properties to libmpv at once: `brightness`, `contrast`, `saturation`, `hue`, `gamma`, `video-zoom`, `video-pan-x/y`, `video-rotate`, `deinterlace`, `vf` (for flip).

### `equalizer.rs`
EQ 10 ISO bands (31Hz–16kHz). Generate the `af=equalizer=f=...` filter for mpv. Presets: Flat, Bass, Vocal, Cinema, Rock.

### `ab_repeat.rs`
A-B repetition. Cycles between three states (A unchecked → A checked → A+B checked → clear) using the `ab-loop-a` and `ab-loop-b` properties of mpv.

### `sleep_timer.rs`
Timer countdown. Tick ​​in each frame. Triggers one of three actions: `Pause`, `Stop`, or `Quit`.

### `media_info.rs`
Extracts metadata from the uploaded file by parsing `track-list` (mpv JSON) and reading properties like `width`, `height`, `fps`, `video-bitrate`, `audio-bitrate`.

### `thumbnail.rs`
Generate 20 frames with `ffmpeg` in a background thread. Stores them in `/tmp/rplayer_thumbs/`. Method `nearest(time)` returns the path of the frame closest to a given position.

### `streaming.rs`
Classify URLs: `DirectStream` (http/rtmp/m3u8), `YtDlp` (YouTube/Vimeo/Twitch), `Unknown`. Detects yt-dlp and ffmpeg in PATH.

### `trim.rs`
Trim without re-encoding. Run `ffmpeg -ss START -i INPUT -t DURATION -c copy OUTPUT` in a separate thread. Status communicated via `crossbeam-channel`.

### `converter.rs`
Format conversion with 8 presets (H.264, H.265, VP9, ​​MP3×2, FLAC, AAC, OGG). Each preset defines the ffmpeg args and output extension.

### `lastfm.rs`
Scrobbling to Last.fm via Mobile Session API. Authentication with MD5. `ScrobbleTracker` accumulates played time excluding pauses and large searches.

### `remote.rs`
HTTP server `tiny_http` on `127.0.0.1:7890`. REST endpoints for playback control. Embedded HTML page with buttons and volume slider.

### `opensubtitles.rs`
Search and download subtitles via OpenSubtitles v2 REST API. Execution in separate threads with state channel.

### `karaoke.rs`
.CDG file support. Detects if there is a `.cdg` next to `.mp3` with the same base name. libmpv automatically plays them when they are together.

### `theme_manager.rs`
9 predefined palettes + custom editor. `ThemeColors::apply(ctx)` overrides the egui visuals (`panel_fill`, `widgets.inactive.bg_fill`, etc.) and updates the mutable global structure `ACTIVE_THEME` to `ui/theme`. This allows all components and texts that use static theme constants (via the `DynamicColor` proxy wrapper) to change and adapt dynamically and with the correct contrast according to the chosen theme.

### `donation.rs`
Donation banner with link to Patreon. Dismissable and the state persists in config.

---

## UI (src/ui/)

All panels are simple structs that receive mutable references and return actions or values.

### Return convention

The panels return a semantic value or `Option<T>`:
- `playlist_panel::draw_playlist` → `(Option<usize>, ContextAction)`
- `HistoryPanel::show` → `(Option<PathBuf>, ContextAction)`
- `BookmarksPanel::show` → `(Option<f64>, ContextAction)`
- `ImageControlsPanel::show` → `bool` (changed)
- `ThemePanel::show` → `bool` (changed)
- `KaraokePanel::show` → `Option<PathBuf>` (file to open)
- `OpenSubtitlesPanel::show` → `Option<PathBuf>` (downloaded subtitle)

### `context_menu.rs`
Right click context menus using egui's `response.context_menu(|ui| {...})`.

- `video_context_menu` — about the video area
- `playlist_item_context_menu` — about each item in the list
- `history_item_context_menu` — about each history item
- `bookmark_context_menu` — about each marker

They all return `ContextAction`, handled in `app.rs::handle_context_action()`.

### `menu.rs`
Top menu organized by topic area. Use `MenuState` to pass all visibility and active status flags. Returns `MenuAction`, handled in `app.rs::handle_menu_action()`.

**Menu structure:**
1. **File** — open, import/export list, exit
2. **Playback** — play/pause, next/previous, skip, frame by frame, repeat, speed
3. **Audio** — mute, volume, normalization, audio panels
4. **Video** — capture, PiP, aspect, rotate, flip, deinterlace
5. **Subtitles** — tracks, download
6. **Tools** — crop, convert, sleep timer, remote control
7. **View** — all side panels
8. **?** - about

### `controls.rs`
Lower control bar with premium aesthetic design.
- **Dynamic Seekbar:** Expands responsively (from 4px to 6px) on hover, hiding the thumb circle in the inactive state. Draw the range A-B with `egui::Painter::rect_filled` and visual markers. Supports direct interaction (click and drag) to search for positions.
- **Featured Play Button:** Renders as a circle of the active theme's accent color, with automatic smart contrast (black/white icon) based on color luminance.
- **Borderless Buttons:** Uses transparent vector buttons that display a subtle circular background on hover.
- **Action Flow:** Returns an enum `ControlsAction` to propagate the interaction events (prev, next, toggle repeat/shuffle) towards `app.rs`, making the interface fully interactive.

---

## Data flow in `app.rs`

Each UI frame:

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

## Threads running

| Thread | Responsibility |
|--------|----------------|
| Main thread (UI) | egui + input + app logic |
| libmpv internal | Video/Audio Decoding |
| OpenGL rendering | Render frames via mpv render API |
| `thumbnail-gen` | Generate thumbnails with ffmpeg |
| `trim-job` | Run ffmpeg to trim |
| `convert-job` | Run ffmpeg to convert |
| `opensubs-search` | Search for subtitles on OpenSubtitles |
| `opensubs-download` | Download the subtitle |
| `lastfm-*` | HTTP requests to Last.fm |
| `remote-server` | HTTP Server tiny_http |
