//! bookmarks.rs — Timestamped bookmarks per file.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: u64,
    pub position: f64,
    pub label: String,
    pub created: DateTime<Utc>,
}

impl Bookmark {
    pub fn new(position: f64, label: impl Into<String>) -> Self {
        Self {
            id: Utc::now().timestamp_millis() as u64,
            position,
            label: label.into(),
            created: Utc::now(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BookmarkStore {
    map: HashMap<String, Vec<Bookmark>>,
}

impl BookmarkStore {
    pub fn load() -> Self {
        let path = Self::path();
        let Ok(data) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&data).unwrap_or_default()
    }

    pub fn save(&self) {
        let path = Self::path();
        if let Some(p) = path.parent() {
            let _ = std::fs::create_dir_all(p);
        }
        if let Ok(data) = serde_json::to_string(self) {
            let _ = std::fs::write(path, data);
        }
    }

    pub fn add(&mut self, file: &Path, bookmark: Bookmark) {
        let key = file.to_string_lossy().to_string();
        let bms = self.map.entry(key).or_default();
        bms.push(bookmark);
        bms.sort_by(|a, b| a.position.total_cmp(&b.position));
    }

    pub fn remove(&mut self, file: &Path, id: u64) {
        let key = file.to_string_lossy().to_string();
        if let Some(bms) = self.map.get_mut(&key) {
            bms.retain(|b| b.id != id);
        }
    }

    pub fn get(&self, file: &Path) -> &[Bookmark] {
        let key = file.to_string_lossy().to_string();
        self.map.get(&key).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn update_label(&mut self, file: &Path, id: u64, label: String) {
        let key = file.to_string_lossy().to_string();
        if let Some(bms) = self.map.get_mut(&key) {
            if let Some(b) = bms.iter_mut().find(|b| b.id == id) {
                b.label = label;
            }
        }
    }

    fn path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rplayer")
            .join("bookmarks.json")
    }
}

#[cfg(test)]
mod tests {
    use super::{Bookmark, BookmarkStore};
    use std::path::PathBuf;

    #[test]
    fn add_with_nan_position_does_not_panic() {
        let mut store = BookmarkStore::default();
        let p = PathBuf::from("demo.mp4");
        store.add(&p, Bookmark::new(10.0, "a"));
        store.add(&p, Bookmark::new(f64::NAN, "nan"));
        assert_eq!(store.get(&p).len(), 2);
    }

    #[test]
    fn bookmarks_are_sorted_by_position() {
        let mut store = BookmarkStore::default();
        let p = PathBuf::from("demo.mp4");
        store.add(&p, Bookmark::new(20.0, "later"));
        store.add(&p, Bookmark::new(10.0, "earlier"));

        let bookmarks = store.get(&p);
        assert_eq!(bookmarks.len(), 2);
        assert_eq!(bookmarks[0].position, 10.0);
        assert_eq!(bookmarks[1].position, 20.0);
    }

    #[test]
    fn update_label_changes_bookmark_text() {
        let mut store = BookmarkStore::default();
        let p = PathBuf::from("demo.mp4");
        let bookmark = Bookmark::new(15.0, "initial");
        let id = bookmark.id;
        store.add(&p, bookmark);

        store.update_label(&p, id, "updated".to_string());
        let bookmarks = store.get(&p);
        assert_eq!(bookmarks[0].label, "updated");
    }

    #[test]
    fn remove_bookmark_deletes_matching_id() {
        let mut store = BookmarkStore::default();
        let p = PathBuf::from("demo.mp4");
        let bookmark = Bookmark::new(10.0, "keep");
        let id = bookmark.id;
        store.add(&p, bookmark);
        store.remove(&p, id);
        assert!(store.get(&p).is_empty());
    }
}
