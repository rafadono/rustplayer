//! karaoke.rs — Karaoke support with .CDG files.
//!
//! A CDG (CD+Graphics) file contains graphics synchronous with the music.
//! libmpv can play it directly if the .cdg file is next to the .mp3
//! with the same base name.
//!
//! Example: "cancion.mp3" + "cancion.cdg" in the same directory.
//! mpv recognizes the CDG automatically if the .mp3 is opened.
//! You can also pass the .cdg directly and mpv will play both.

use std::path::{Path, PathBuf};

/// Checks if an audio file has an associated .CDG in the same directory.
pub fn find_cdg(audio_path: &Path) -> Option<PathBuf> {
    let stem = audio_path.file_stem()?;
    let dir = audio_path.parent()?;
    let cdg = dir.join(format!("{}.cdg", stem.to_string_lossy()));
    let cdg_upper = dir.join(format!("{}.CDG", stem.to_string_lossy()));

    if cdg.exists() {
        return Some(cdg);
    }
    if cdg_upper.exists() {
        return Some(cdg_upper);
    }
    None
}

/// Returns the list of .CDG files in a directory.
pub fn list_cdg_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };
    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .map(|e| e.to_ascii_lowercase() == "cdg")
                .unwrap_or(false)
        })
        .collect()
}

/// File type: Detects whether it is CDG or karaoke audio.
#[derive(Debug, Clone, PartialEq)]
pub enum KaraokeFileType {
    Cdg,
    AudioWithCdg(PathBuf), // .mp3 audio with associated .cdg
    Audio,
}

pub fn classify(path: &Path) -> KaraokeFileType {
    let ext = path
        .extension()
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_default();

    if ext == "cdg" {
        return KaraokeFileType::Cdg;
    }

    // Audio with associated CDG
    if matches!(
        ext.to_str().unwrap_or(""),
        "mp3" | "ogg" | "flac" | "wav" | "aac" | "m4a"
    ) {
        if let Some(cdg) = find_cdg(path) {
            return KaraokeFileType::AudioWithCdg(cdg);
        }
    }

    KaraokeFileType::Audio
}

///Instructions for use to display in the UI.
pub const USAGE_HINT: &str =
    "Para karaoke: coloca el archivo .cdg junto al .mp3 con el mismo nombre.\n\
     Ejemplo: 'cancion.mp3' + 'cancion.cdg'\n\
     RPlayer los reproducirá automáticamente en sincronía.";
