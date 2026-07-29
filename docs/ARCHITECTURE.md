# Arquitectura Modular del Backend en Rust (`src/`)

El backend de **RPlayer** se encuentra organizado en **5 paquetes de dominio altamente modulares y desacoplados**:

```
src/
├── playback/   ── Lógica de reproducción libmpv, ecualizador PEQ, karaoke y capítulos
├── storage/    ── Persistencia JSON: configuración, historial, playlists, marcadores y notas
├── tools/      ── Procesamiento multimedia con FFmpeg: recortes (trim), conversor y thumbnails
├── services/   ── Integraciones y servicios de red: OpenSubtitles, Last.fm, HTTP Remote Server
└── theme/      ── Temas de color e internacionalización (i18n)
```

---

## 📦 Dominio 1: `playback/` (Reproducción & Audio/Video)
- **[src/playback/player.rs](file:///home/rafael/rustplayer/src/playback/player.rs)**: Wrapper thread-safe sobre `libmpv2` con superficie de ventana forzada (`force-window=yes`).
- **[src/playback/equalizer.rs](file:///home/rafael/rustplayer/src/playback/equalizer.rs)**: Ecualizador paramétrico de 6 bandas gráficos con preamp y presets.
- **[src/playback/image_controls.rs](file:///home/rafael/rustplayer/src/playback/image_controls.rs)**: Controles de imagen: brillo, contraste, saturación, tono y gamma.
- **[src/playback/karaoke.rs](file:///home/rafael/rustplayer/src/playback/karaoke.rs)**: Supresión vocal y ajuste de tono (*pitch shift*).
- **[src/playback/ab_repeat.rs](file:///home/rafael/rustplayer/src/playback/ab_repeat.rs)**: Bucle de repetición de puntos A-B.
- **[src/playback/chapters.rs](file:///home/rafael/rustplayer/src/playback/chapters.rs)**: Parser y navegación por capítulos de mpv.
- **[src/playback/up_next.rs](file:///home/rafael/rustplayer/src/playback/up_next.rs)**: Autoplay del siguiente episodio.
- **[src/playback/sleep_timer.rs](file:///home/rafael/rustplayer/src/playback/sleep_timer.rs)**: Temporizador de apagado.

---

## 💾 Dominio 2: `storage/` (Persistencia & Estado)
- **[src/storage/config.rs](file:///home/rafael/rustplayer/src/storage/config.rs)**: Configuración JSON en `~/.config/rplayer/config.json`.
- **[src/storage/playlist.rs](file:///home/rafael/rustplayer/src/storage/playlist.rs)**: Gestor de listas M3U / PLS.
- **[src/storage/history.rs](file:///home/rafael/rustplayer/src/storage/history.rs)**: Historial de reproducción y auto-resume.
- **[src/storage/bookmarks.rs](file:///home/rafael/rustplayer/src/storage/bookmarks.rs)**: Marcadores de tiempo persistentes.
- **[src/storage/notes.rs](file:///home/rafael/rustplayer/src/storage/notes.rs)**: Notas por segundo exportables a `.txt`.

---

## ✂️ Dominio 3: `tools/` (Procesamiento FFmpeg)
- **[src/tools/trim.rs](file:///home/rafael/rustplayer/src/tools/trim.rs)**: Recorte de video sin pérdida (*Lossless Video Trim*).
- **[src/tools/converter.rs](file:///home/rafael/rustplayer/src/tools/converter.rs)**: Conversor de formatos (MP4, MP3, MKV, FLAC).
- **[src/tools/media_info.rs](file:///home/rafael/rustplayer/src/tools/media_info.rs)**: Ficha técnica completa de medios.
- **[src/tools/thumbnail.rs](file:///home/rafael/rustplayer/src/tools/thumbnail.rs)**: Generador de vistas previas de fotogramas.

---

## 🌐 Dominio 4: `services/` (Servicios & Red)
- **[src/services/opensubtitles.rs](file:///home/rafael/rustplayer/src/services/opensubtitles.rs)**: Buscador de subtítulos en OpenSubtitles REST API.
- **[src/services/lastfm.rs](file:///home/rafael/rustplayer/src/services/lastfm.rs)**: Scrobbler de canciones en Last.fm.
- **[src/services/remote.rs](file:///home/rafael/rustplayer/src/services/remote.rs)**: Servidor de Control Remoto HTTP (Puerto 8080).
- **[src/services/streaming.rs](file:///home/rafael/rustplayer/src/services/streaming.rs)**: Clasificador de streams y YouTube (`yt-dlp`).
- **[src/services/updater.rs](file:///home/rafael/rustplayer/src/services/updater.rs)**: Verificador de actualizaciones.

---

## 🎨 Dominio 5: `theme/` (Temas & Localización)
- **[src/theme/theme_manager.rs](file:///home/rafael/rustplayer/src/theme/theme_manager.rs)**: Paletas de color y cálculo de contraste.
- **[src/theme/i18n.rs](file:///home/rafael/rustplayer/src/theme/i18n.rs)**: Localización internacional de cadenas.
