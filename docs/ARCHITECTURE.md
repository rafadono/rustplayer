# Modular Rust Backend Architecture (`src/`)

**RPlayer**'s backend is organized into **5 highly modular, decoupled domain packages**:

```
src/
├── playback/   ── libmpv playback logic, PEQ equalizer, karaoke, and chapters
├── storage/    ── JSON persistence: config, history, playlists, bookmarks, and notes
├── tools/      ── FFmpeg-based media processing: trim, converter, and thumbnails
├── services/   ── Integrations and network services: OpenSubtitles, Last.fm, HTTP Remote Server
└── theme/      ── Color themes and internationalization (i18n)
```

---

## 📦 Domain 1: `playback/` (Playback & Audio/Video)
- **[src/playback/player.rs](../src/playback/player.rs)**: Thread-safe wrapper around `libmpv2` with a forced window surface (`force-window=yes`).
- **[src/playback/equalizer.rs](../src/playback/equalizer.rs)**: 6-band graphic parametric equalizer with preamp and presets.
- **[src/playback/image_controls.rs](../src/playback/image_controls.rs)**: Image controls: brightness, contrast, saturation, hue, and gamma.
- **[src/playback/karaoke.rs](../src/playback/karaoke.rs)**: Vocal suppression (channel phase-cancellation `pan` filter) and pitch shift (asetrate/atempo trick, ±12 semitones) applied to mpv's `af` chain alongside the PEQ equalizer; wired to the checkbox/slider in the Audio modal's Karaoke tab.
- **[src/playback/ab_repeat.rs](../src/playback/ab_repeat.rs)**: A-B point repeat loop, wired to mpv's real `ab-loop-a`/`ab-loop-b` properties. The A-B button in the player controls cycles set-A → set-B → clear.
- **[src/playback/chapters.rs](../src/playback/chapters.rs)**: mpv chapter parser and navigation, exposed in the UI via the Tools modal's **Chapters** tab with click-to-jump.
- **[src/playback/sleep_timer.rs](../src/playback/sleep_timer.rs)**: Sleep timer.

---

## 💾 Domain 2: `storage/` (Persistence & State)
- **[src/storage/config.rs](../src/storage/config.rs)**: JSON configuration at `~/.config/rplayer/config.json`.
- **[src/storage/playlist.rs](../src/storage/playlist.rs)**: M3U / M3U8 / PLS playlist manager with UI-driven import/export (⬆/⬇ buttons in the playlist panel); supports network stream URLs alongside local files.
- **[src/storage/history.rs](../src/storage/history.rs)**: Playback history and auto-resume, exposed in the Tools modal's **History** tab with a resume prompt on file load.
- **[src/storage/bookmarks.rs](../src/storage/bookmarks.rs)**: Persistent timestamp bookmarks.
- **[src/storage/notes.rs](../src/storage/notes.rs)**: Timestamped per-file notes, exposed in the Tools modal's **Notes** tab (add/list/delete) with export to `.txt` via a save dialog.

---

## ✂️ Domain 3: `tools/` (FFmpeg Processing)
- **[src/tools/trim.rs](../src/tools/trim.rs)**: Lossless video trim.
- **[src/tools/converter.rs](../src/tools/converter.rs)**: Format converter (MP4, MP3, MKV, FLAC), exposed in the Tools modal's **Convert** tab.
- **[src/tools/media_info.rs](../src/tools/media_info.rs)**: Full media technical sheet.
- **[src/tools/thumbnail.rs](../src/tools/thumbnail.rs)**: Background `ffmpeg`-based thumbnail generator; powers the seekbar drag preview (thumbnails inlined as base64 data URIs — shown only while actively dragging, not on passive hover).

---

## 🌐 Domain 4: `services/` (Services & Network)
- **[src/services/opensubtitles.rs](../src/services/opensubtitles.rs)**: Subtitle search via the OpenSubtitles REST API.
- **[src/services/lastfm.rs](../src/services/lastfm.rs)**: Last.fm song scrobbler.
- **[src/services/remote.rs](../src/services/remote.rs)**: HTTP Remote Control Server (configurable port, 7890 by default).
- **[src/services/streaming.rs](../src/services/streaming.rs)**: Stream/podcast URL classifier (`yt-dlp`) and URL validation, exposed via the header's **Open URL** modal (text entry + quick-pick default radio stations); mpv's own `ytdl_hook` handles resolving YouTube/Twitch/etc. links when `yt-dlp` is on `PATH`.
- **[src/services/updater.rs](../src/services/updater.rs)**: Update checker, exposed in the Tools modal's Settings tab (channel selector, manifest URL fields, check/install buttons). This project does not run its own release-manifest server, so the manifest URLs are blank by default — point them at your own JSON endpoint(s) to activate it *(see Known Gaps)*.

---

## 🎨 Domain 5: `theme/` (Themes & Localization)
- **[src/theme/theme_manager.rs](../src/theme/theme_manager.rs)**: Color palettes and contrast calculation.
- **[src/theme/i18n.rs](../src/theme/i18n.rs)**: Bilingual (Spanish/English) string dictionary via `tr(language, key)`. The GUI is fully bilingual; language is runtime-switchable from the globe-icon button in the header bar and persisted in `Config.language`.

---

## ⚠️ Known Gaps

- **`services/updater.rs`**: fully wired end-to-end (check, show result, download-and-install-with-rollback), but there's no manifest to point it at — this project doesn't publish a release-manifest JSON anywhere. The Settings tab lets you fill in your own Stable/Beta manifest URL; until one is set, "Check for updates" reports it isn't configured instead of hitting a fake endpoint.
- **`playback/up_next.rs`** was removed as dead code (unused since the real playlist manager in `storage/playlist.rs` replaced it).
