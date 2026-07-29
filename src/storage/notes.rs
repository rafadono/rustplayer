//! notes.rs — Timestamp notes per video/audio file.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: u64,
    pub position: f64,
    pub text: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Note {
    pub fn new(position: f64, text: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: now.timestamp_millis() as u64,
            position,
            text: text.into(),
            created: now,
            updated: now,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NoteStore {
    map: HashMap<String, Vec<Note>>,
}

impl NoteStore {
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

    pub fn add(&mut self, file: &Path, note: Note) {
        let key = file.to_string_lossy().to_string();
        let notes = self.map.entry(key).or_default();
        notes.push(note);
        notes.sort_by(|a, b| a.position.total_cmp(&b.position));
    }

    pub fn remove(&mut self, file: &Path, id: u64) {
        let key = file.to_string_lossy().to_string();
        if let Some(notes) = self.map.get_mut(&key) {
            notes.retain(|n| n.id != id);
        }
    }

    pub fn get(&self, file: &Path) -> &[Note] {
        let key = file.to_string_lossy().to_string();
        self.map.get(&key).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Export notes as plain text to share
    pub fn export_text(&self, file: &Path) -> String {
        let notes = self.get(file);
        let mut lines = vec![format!(
            "Notas: {}\n",
            file.file_name().unwrap_or_default().to_string_lossy()
        )];
        for n in notes {
            let h = (n.position as u64) / 3600;
            let m = ((n.position as u64) % 3600) / 60;
            let s = (n.position as u64) % 60;
            lines.push(format!("[{:02}:{:02}:{:02}] {}", h, m, s, n.text));
        }
        lines.join("\n")
    }

    fn path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rplayer")
            .join("notes.json")
    }
}

#[cfg(test)]
mod tests {
    use super::{Note, NoteStore};
    use std::path::PathBuf;

    #[test]
    fn add_with_nan_position_does_not_panic() {
        let mut store = NoteStore::default();
        let p = PathBuf::from("demo.mp4");
        store.add(&p, Note::new(10.0, "hola"));
        store.add(&p, Note::new(f64::NAN, "nan"));
        assert_eq!(store.get(&p).len(), 2);
    }

    #[test]
    fn export_text_formats_time_codes() {
        let mut store = NoteStore::default();
        let p = PathBuf::from("demo.mp4");
        store.add(&p, Note::new(65.0, "First note"));
        store.add(&p, Note::new(3602.0, "Second note"));

        let text = store.export_text(&p);
        assert!(text.contains("[00:01:05] First note"));
        assert!(text.contains("[01:00:02] Second note"));
    }

    #[test]
    fn notes_remain_ordered_by_position() {
        let mut store = NoteStore::default();
        let p = PathBuf::from("demo.mp4");
        store.add(&p, Note::new(30.0, "later"));
        store.add(&p, Note::new(10.0, "earlier"));

        let notes = store.get(&p);
        assert_eq!(notes[0].position, 10.0);
        assert_eq!(notes[1].position, 30.0);
    }
}
