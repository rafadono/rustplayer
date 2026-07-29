//! chapters.rs — Navigation of chapters embedded in MKV/MP4.
//!
//! mpv exposes the list of chapters as JSON via the "chapter-list" property.

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Chapter {
    pub index: usize,
    pub title: String,
    pub time: f64,
}

/// Parse the JSON returned by mpv for "chapter-list"
pub fn parse_chapter_list(json: &str) -> Vec<Chapter> {
    #[derive(Deserialize)]
    struct MpvChapter {
        title: Option<String>,
        time: f64,
    }

    let raw: Vec<MpvChapter> = serde_json::from_str(json).unwrap_or_default();
    raw.into_iter()
        .enumerate()
        .map(|(i, c)| Chapter {
            index: i,
            title: c
                .title
                .filter(|t| !t.is_empty())
                .unwrap_or_else(|| format!("Capítulo {}", i + 1)),
            time: c.time,
        })
        .collect()
}

/// Returns the index of the active chapter given a timestamp
pub fn current_chapter(chapters: &[Chapter], position: f64) -> Option<usize> {
    chapters
        .iter()
        .rev()
        .find(|c| c.time <= position)
        .map(|c| c.index)
}
