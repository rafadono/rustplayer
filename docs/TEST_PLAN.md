# 🧪 Manual Test Plan — RPlayer v0.5.0

This document provides a detailed step-by-step verification guide to manually test **100% of the features** of the **RPlayer** player on Linux (Fedora/Ubuntu/Debian), Windows, and macOS.

---

## 📋 Test Index

1. [🎬 Module 1: Loading and Playback Control](#-module-1-loading-and-playback-control)
2. [🎛️ Module 2: Audio and Sound Center](#️-module-2-audio-and-sound-center)
3. [🎬 Module 3: Video and Subtitles Center](#-module-3-video-and-subtitles-center)
4. [⚙️ Module 4: Tools and Settings Center](#️-module-4-tools-and-settings-center)
5. [📋 Module 5: Playlist](#-module-5-playlist)
6. [🌗 Module 6: Theme System and Visual Readability](#-module-6-theme-system-and-visual-readability)
7. [🆕 Module 7: VLC-Parity Pass 1 (History, Notes, Chapters, Overlay, Playlist, Thumbnails)](#-module-7-vlc-parity-pass-1-history-notes-chapters-overlay-playlist-thumbnails)

---

## 🎬 Module 1: Loading and Playback Control

| ID | Feature | Verification Steps | Expected Result |
| :-: | :--- | :--- | :--- |
| **TC-01** | **File Opening** | 1. Click `📂 Open` in the top bar.<br>2. Select a media file (`.mkv`, `.mp4`, `.mp3`, `.flac`). | The file loads immediately, the title appears in the header, and video playback starts with no black screen. |
| **TC-02** | **Play / Pause** | 1. Click `⏸ Pause` or press `Space`.<br>2. Click `▶ Play`. | The video freezes immediately on the current frame and resumes when pressed again. |
| **TC-03** | **Seeking via Progress Bar** | 1. Click any point on the bottom progress bar.<br>2. Drag the slider from left to right. | The video jumps instantly (*keyframe seek*) to the indicated position without the screen going black or white. |
| **TC-04** | **Quick ±10s Skip** | 1. Click `⏪ -10s`.<br>2. Click `⏩ +10s`. | The time counter moves back or forward exactly 10 seconds per click, smoothly. |
| **TC-05** | **Volume Control and Mute** | 1. Drag the volume slider.<br>2. Click the `🔊` / `🔇` button. | The audio level changes progressively. Muting switches to `🔇` and silences audio output. |
| **TC-06** | **Playback Speed** | 1. Open the speed selector in the bottom bar.<br>2. Select `0.5x`, `1.5x`, or `2.0x`. | Audio and video speed up or slow down at the exact selected rate without pitch loss. |
| **TC-07** | **A-B Repeat Loop** | 1. Press `🔂 A-B` at the starting position (Point A).<br>2. Press it again further along (Point B). | The player automatically loops between Point A and Point B without stopping. An additional click clears the loop. |

---

## 🎛️ Module 2: Audio and Sound Center

> **Access**: Click the `🎛️ Audio` button in the top bar.

| ID | Feature | Verification Steps | Expected Result |
| :-: | :--- | :--- | :--- |
| **TC-08** | **PEQ Equalizer (Filters)** | 1. Check the *"Enable parametric equalizer"* checkbox.<br>2. Move the frequency sliders (`60Hz` to `16kHz`). | The acoustic change in bass, mid, and treble tones is noticeable in real time. |
| **TC-09** | **Equalizer Presets** | 1. Open the presets list.<br>2. Select `Bass Boost`, `Rock`, or `Vocal`. | The dropdown is 100% readable (white text on dark background in dark mode) and the sliders adopt the preset values. |
| **TC-10** | **Alternate Audio Tracks** | 1. Go to the `🎧 Audio Tracks` tab.<br>2. Select a secondary track on a multi-language file. | The audio track switches immediately to the selected language/track. |
| **TC-11** | **Audio Offset** | 1. Move the *"Audio Offset"* slider between `-5.0s` and `+5.0s`. | The sound is delayed or advanced relative to the characters' lips on screen. |
| **TC-12** | **Vocal Suppression (Karaoke)** | 1. Go to the `🎤 Karaoke & Pitch` tab.<br>2. Enable *"Vocal Suppression"*. | The song's center-panned frequencies (vocals) are significantly attenuated. |
| **TC-13** | **Pitch Shift** | 1. Move the *"Pitch Shift"* slider from `-6.0` to `+6.0` semitones. | The musical pitch shifts up or down while keeping the same playback speed. |

---

## 🎬 Module 3: Video and Subtitles Center

> **Access**: Click the `🎬 Video` button in the top bar.

| ID | Feature | Verification Steps | Expected Result |
| :-: | :--- | :--- | :--- |
| **TC-14** | **Embedded Subtitles** | 1. Open a video with embedded subtitles.<br>2. Go to the `💬 Subtitles` tab and select a track. | Subtitles appear at the bottom of the video image. |
| **TC-15** | **External Subtitles** | 1. Click `📁 Load File (.srt / .vtt)`.<br>2. Choose a local subtitle file. | The `.srt` or `.vtt` file is linked and rendered over the active playback. |
| **TC-16** | **Subtitle Offset** | 1. Move the *"Subtitle Offset"* slider between `-5.0s` and `+5.0s`. | Subtitle text appears before or after the spoken dialogue. |
| **TC-17** | **OpenSubtitles Search** | 1. Type a movie title into the OpenSubtitles search box.<br>2. Press `🔍 Search`. | The API is queried in the background without freezing the UI. |
| **TC-18** | **Image Settings (Brightness/Contrast)** | 1. Go to the `🎨 Image Settings` tab.<br>2. Move the *Brightness*, *Contrast*, *Saturation*, *Hue*, and *Gamma* sliders. | The video image responds in real time to the color changes. |
| **TC-19** | **Reset Image** | 1. Press `🔄 Reset to Defaults`. | All image sliders return to `0` and the video regains its original colors. |

---

## ⚙️ Module 4: Tools and Settings Center

> **Access**: Click the `⚙️ Tools` button in the top bar.

| ID | Feature | Verification Steps | Expected Result |
| :-: | :--- | :--- | :--- |
| **TC-20** | **Add Bookmark** | 1. Go to the `🔖 Bookmarks & Notes` tab.<br>2. Click `➕ Add Current Bookmark`. | The exact timestamp is recorded with an editable label. |
| **TC-21** | **Jump to Bookmark** | 1. Click `▶ Jump` on any saved bookmark. | Playback jumps immediately to that timestamp in the file. |
| **TC-22** | **Delete Bookmark** | 1. Click the `🗑️` icon on a bookmark row. | The entry is removed from the list and from JSON persistence. |
| **TC-23** | **Video Trim** | 1. Go to the `✂️ Lossless Trim` tab.<br>2. Set start and end points, then press `✂️ Export Trimmed Clip`. | `ffmpeg` is invoked to extract the segment without quality loss or re-encoding. |
| **TC-24** | **Media Info Sheet** | 1. Go to the `ℹ️ Media Info` tab. | The exact resolution, video/audio codecs, bitrate, container, and FPS of the file are shown. |
| **TC-25** | **HTTP Remote Control Server** | 1. Go to the `⚙️ Network & Preferences` tab.<br>2. Click `▶ Start Remote Server (Port 7890)`.<br>3. Open `http://localhost:7890` in a browser. | The web remote-control panel appears to pause, play, and control volume from a phone/browser. |
| **TC-26** | **Sleep Timer** | 1. Select a timer (`15 min`, `30 min`, `45 min`, `60 min`). | The countdown starts and stops playback once time is up. |

---

## 📋 Module 5: Playlist

| ID | Feature | Verification Steps | Expected Result |
| :-: | :--- | :--- | :--- |
| **TC-27** | **Toggle Side Panel** | 1. Click `📋 Playlist` in the top bar. | The right-side panel slides open with animation without altering the video's aspect ratio. |
| **TC-28** | **Track Selection** | 1. Click any file in the playlist. | The selected file loads and starts playing immediately. |

---

## 🌗 Module 6: Theme System and Visual Readability

| ID | Feature | Verification Steps | Expected Result |
| :-: | :--- | :--- | :--- |
| **TC-29** | **Dark / Light Mode Toggle** | 1. Click `☀️ Light` / `🌙 Dark` in the top-right corner. | The entire interface instantly switches between the dark charcoal theme and the bright light theme. |
| **TC-30** | **Dropdown Readability (`<select>`)** | 1. Open any dropdown list (e.g. equalizer presets or speed) in Dark Mode.<br>2. Hover over the options. | **Box and menu**: the select box has a dark `#1a1d24` background, white `#f3f4f6` text, and a vector SVG arrow. The popup options read with perfect contrast. |
| **TC-31** | **Clean Modal Close (1 Click)** | 1. Open any modal (`Audio`, `Video`, `Tools`).<br>2. Click `✕` or the backdrop layer. | The modal closes instantly with **a single click**, with no duplicated backdrop layers or getting "stuck". |
| **TC-32** | **Language Toggle (ES/EN)** | 1. Click the globe icon (🌐) in the header bar.<br>2. Observe UI labels, tooltips, and buttons throughout the app.<br>3. Restart the app. | The interface switches instantly between Spanish and English, and the chosen language persists after restart (saved in `Config.language`). |

---

## 🆕 Module 7: VLC-Parity Pass 1 (History, Notes, Chapters, Overlay, Playlist, Thumbnails)

| ID | Feature | Verification Steps | Expected Result |
| :-: | :--- | :--- | :--- |
| **TC-33** | **Playback History & Resume** | 1. Play a file partway through, then close it or switch to another file.<br>2. Reopen the same file and check the `⏱️ History` tab in the Tools modal. | The file appears in the history list with its last playback position, and reopening it offers to resume from that position. |
| **TC-34** | **Timestamped Notes** | 1. During playback, go to the `📝 Notes` tab in the Tools modal.<br>2. Click `➕ Add Note` at the current position, then check the list. | A note is created at the current timestamp, appears in the list, and can be removed with the `🗑️` icon. |
| **TC-35** | **Notes Export** | 1. In the `📝 Notes` tab, click `⬇ Export Notes`.<br>2. Choose a destination in the save dialog. | All notes for the current file are written to a `.txt` file at the chosen location. |
| **TC-36** | **Real A-B Loop** | 1. Click `🔂 A-B` to mark point A, click again to mark point B.<br>2. Let playback run past point B. | Playback jumps back to point A automatically, driven by mpv's real `ab-loop-a`/`ab-loop-b` properties (not just a UI toggle). A third click clears the loop. |
| **TC-37** | **Chapter Navigation** | 1. Open a file with embedded chapters (e.g. `.mkv`).<br>2. Go to the `📑 Chapters` tab in the Tools modal and click a chapter entry. | The tab lists the chapters embedded in the file with their titles/timestamps, and clicking one jumps playback to that chapter. |
| **TC-38** | **Performance/Metrics Overlay** | 1. Go to the `⚙️ Settings` tab in the Tools modal and enable the metrics overlay checkbox. | A small overlay appears in the top-right corner of the video canvas showing live FPS, dropped frames, hardware-decode status, and buffer seconds. |
| **TC-39** | **Playlist Import (M3U/M3U8/PLS)** | 1. Open the playlist panel.<br>2. Click `⬆ Import` and select a `.m3u`, `.m3u8`, or `.pls` file. | All entries from the playlist file, including network stream URLs, are loaded into the playlist panel. |
| **TC-40** | **Playlist Export** | 1. With items in the playlist, click `⬇ Export`.<br>2. Choose a destination in the save dialog. | A valid `.m3u`/`.m3u8`/`.pls` file is written containing the current playlist entries. |
| **TC-41** | **Seekbar Thumbnail Preview** | 1. Press and hold the seek slider handle and drag it without releasing. | A small preview thumbnail pops up above the slider showing the frame at that position, and disappears when dragging stops. It does not appear on passive hover (mouse-over without dragging). |
