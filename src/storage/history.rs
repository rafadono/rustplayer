//! history.rs — Playback history and resume position.
//!
//! Saves the last position of each file to summarize where you left off.
//! It also maintains a history of recent files.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const MAX_HISTORY: usize = 200;
/// Do not save position if there is less than this percentage left to finish
const NEAR_END_THRESHOLD: f64 = 0.97;
/// Do not save position if it is less than this time (the first seconds do not count)
const MIN_SAVE_POSITION: f64 = 5.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub path: PathBuf,
    pub title: String,
    pub last_position: f64,
    pub duration: f64,
    pub last_watched: DateTime<Utc>,
    pub play_count: u32,
}

impl HistoryEntry {
    pub fn should_resume(&self) -> bool {
        self.last_position > MIN_SAVE_POSITION
            && (self.duration == 0.0 || self.last_position / self.duration < NEAR_END_THRESHOLD)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct History {
    /// Mapping by canonical path for O(1) lookup
    entries: HashMap<String, HistoryEntry>,
    /// Access order (most recent first)
    order: Vec<String>,
}

impl History {
    pub fn load() -> Self {
        let path = Self::path();
        let Ok(data) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&data).unwrap_or_default()
    }

    pub fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(data) = serde_json::to_string(self) {
            let _ = std::fs::write(path, data);
        }
    }

    /// Save playback progress (position/duration) without counting a new playback.
    pub fn update(&mut self, path: &Path, title: &str, position: f64, duration: f64) {
        let key = path.to_string_lossy().to_string();

        let entry = self
            .entries
            .entry(key.clone())
            .or_insert_with(|| HistoryEntry {
                path: path.to_path_buf(),
                title: title.to_string(),
                last_position: 0.0,
                duration,
                last_watched: Utc::now(),
                play_count: 0,
            });

        entry.last_position = position;
        entry.duration = duration;
        entry.last_watched = Utc::now();
        entry.title = title.to_string();

        self.bump_order(key);
    }

    /// Marks the start of a play and increments the plays counter once.
    pub fn mark_play_start(&mut self, path: &Path, title: &str, duration: f64) {
        let key = path.to_string_lossy().to_string();
        let entry = self
            .entries
            .entry(key.clone())
            .or_insert_with(|| HistoryEntry {
                path: path.to_path_buf(),
                title: title.to_string(),
                last_position: 0.0,
                duration,
                last_watched: Utc::now(),
                play_count: 0,
            });
        entry.last_watched = Utc::now();
        entry.duration = duration;
        entry.title = title.to_string();
        entry.play_count = entry.play_count.saturating_add(1);

        self.bump_order(key);
    }

    fn bump_order(&mut self, key: String) {
        self.order.retain(|k| k != &key);
        self.order.insert(0, key);

        if self.order.len() > MAX_HISTORY {
            if let Some(old_key) = self.order.pop() {
                self.entries.remove(&old_key);
            }
        }
    }

    pub fn get(&self, path: &Path) -> Option<&HistoryEntry> {
        let key = path.to_string_lossy().to_string();
        self.entries.get(&key)
    }

    pub fn recent(&self) -> impl Iterator<Item = &HistoryEntry> {
        self.order.iter().filter_map(|k| self.entries.get(k))
    }

    pub fn all_entries(&self) -> Vec<HistoryEntry> {
        self.order
            .iter()
            .filter_map(|k| self.entries.get(k).cloned())
            .collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
    }

    pub fn remove(&mut self, path: &Path) {
        let key = path.to_string_lossy().to_string();
        self.entries.remove(&key);
        self.order.retain(|k| k != &key);
    }

    pub fn remove_many(&mut self, paths: &[PathBuf]) {
        for p in paths {
            self.remove(p);
        }
    }

    fn path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rplayer")
            .join("history.json")
    }
}

#[cfg(test)]
mod tests {
    use super::{History, HistoryEntry};
    use std::path::PathBuf;

    #[test]
    fn play_count_only_changes_on_mark_play_start() {
        let mut h = History::default();
        let p = PathBuf::from("demo.mp4");
        h.mark_play_start(&p, "demo", 120.0);
        h.update(&p, "demo", 10.0, 120.0);
        h.update(&p, "demo", 20.0, 120.0);
        let e = h.get(&p).expect("missing history entry");
        assert_eq!(e.play_count, 1);
    }

    #[test]
    fn should_resume_only_when_position_is_valid() {
        let entry = HistoryEntry {
            path: PathBuf::from("demo.mp4"),
            title: "demo".to_string(),
            last_position: 6.0,
            duration: 100.0,
            last_watched: chrono::Utc::now(),
            play_count: 1,
        };
        assert!(entry.should_resume());

        let entry_end = HistoryEntry {
            last_position: 98.0,
            duration: 100.0,
            ..entry.clone()
        };
        assert!(!entry_end.should_resume());

        let entry_short = HistoryEntry {
            last_position: 4.0,
            duration: 100.0,
            ..entry.clone()
        };
        assert!(!entry_short.should_resume());
    }

    #[test]
    fn recent_returns_entries_in_most_recent_order() {
        let mut h = History::default();
        let p1 = PathBuf::from("first.mp4");
        let p2 = PathBuf::from("second.mp4");

        h.mark_play_start(&p1, "first", 120.0);
        h.mark_play_start(&p2, "second", 100.0);

        let recent: Vec<_> = h.recent().map(|e| e.path.clone()).collect();
        assert_eq!(recent, vec![p2.clone(), p1.clone()]);
    }

    #[test]
    fn clear_and_remove_many_work() {
        let mut h = History::default();
        let p1 = PathBuf::from("first.mp4");
        let p2 = PathBuf::from("second.mp4");

        h.mark_play_start(&p1, "first", 120.0);
        h.mark_play_start(&p2, "second", 100.0);
        h.remove_many(&[p1.clone(), p2.clone()]);
        assert!(h.get(&p1).is_none());
        assert!(h.get(&p2).is_none());

        h.mark_play_start(&p1, "first", 120.0);
        h.clear();
        assert!(h.get(&p1).is_none());
    }
}
