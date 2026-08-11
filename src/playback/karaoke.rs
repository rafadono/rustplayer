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
                .map(|e| e.eq_ignore_ascii_case("cdg"))
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

/// Builds the mpv `af` filter fragment for vocal suppression and pitch shift.
/// Returns an empty string when both are inactive.
///
/// Vocal suppression uses phase-cancellation between channels (removes audio
/// mixed dead-center, which is where most studio vocal tracks live). Pitch
/// shift uses the asetrate/atempo trick (resample to bend pitch, then
/// time-stretch back to the original tempo) so it works with a stock ffmpeg
/// build, without depending on librubberband being compiled into it.
pub fn to_mpv_af_chain(vocal_suppression: bool, pitch_semitones: f64) -> String {
    let mut chain: Vec<String> = Vec::new();

    if vocal_suppression {
        chain.push("pan=stereo|c0=c0-c1|c1=c1-c0".to_string());
    }

    if pitch_semitones.abs() > 0.01 {
        let ratio = 2f64.powf(pitch_semitones.clamp(-12.0, 12.0) / 12.0);
        chain.push(format!(
            "asetrate=48000*{ratio:.6},aresample=48000,atempo={:.6}",
            1.0 / ratio
        ));
    }

    chain.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_when_both_inactive() {
        assert_eq!(to_mpv_af_chain(false, 0.0), "");
    }

    #[test]
    fn vocal_suppression_only() {
        assert_eq!(to_mpv_af_chain(true, 0.0), "pan=stereo|c0=c0-c1|c1=c1-c0");
    }

    #[test]
    fn pitch_shift_only_contains_expected_filters() {
        let chain = to_mpv_af_chain(false, 6.0);
        assert!(chain.contains("asetrate=48000*"));
        assert!(chain.contains("aresample=48000"));
        assert!(chain.contains("atempo="));
    }

    #[test]
    fn combines_both_filters_with_comma() {
        let chain = to_mpv_af_chain(true, -3.0);
        assert!(chain.starts_with("pan=stereo|c0=c0-c1|c1=c1-c0,asetrate="));
    }
}
