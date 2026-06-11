//! playlist.rs — Playlist with M3U/PLS support.

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Track {
    pub path: PathBuf,
    pub title: String,
}

impl Track {
    pub fn from_path(path: PathBuf) -> Self {
        let title = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        Self { path, title }
    }

    pub fn from_url(url: String, title: Option<String>) -> Self {
        let title = title.unwrap_or_else(|| url.clone());
        Self {
            path: PathBuf::from(&url),
            title,
        }
    }
}

#[derive(Default)]
pub struct Playlist {
    pub tracks: Vec<Track>,
    pub current: Option<usize>,
}

impl Playlist {
    pub fn add(&mut self, path: PathBuf) {
        if !self.tracks.iter().any(|t| t.path == path) {
            self.tracks.push(Track::from_path(path));
        }
    }

    pub fn add_url(&mut self, url: String, title: Option<String>) {
        let track = Track::from_url(url, title);
        if !self.tracks.iter().any(|t| t.path == track.path) {
            self.tracks.push(track);
        }
    }

    pub fn add_many(&mut self, paths: Vec<PathBuf>) {
        for p in paths {
            self.add(p);
        }
    }

    pub fn set_current_by_path(&mut self, path: &PathBuf) {
        self.current = self.tracks.iter().position(|t| &t.path == path);
    }

    pub fn next(&self) -> Option<&Track> {
        self.tracks.get(self.current? + 1)
    }

    pub fn prev(&self) -> Option<&Track> {
        let idx = self.current?.checked_sub(1)?;
        self.tracks.get(idx)
    }

    pub fn remove(&mut self, idx: usize) {
        if idx < self.tracks.len() {
            self.tracks.remove(idx);
            self.current = self.current.and_then(|c| {
                if c == idx {
                    None
                } else if c > idx {
                    Some(c - 1)
                } else {
                    Some(c)
                }
            });
        }
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
        self.current = None;
    }

    /// Import an M3U or M3U8 file
    pub fn import_m3u(&mut self, path: &Path) {
        let Ok(content) = fs::read_to_string(path) else {
            return;
        };
        let dir = path.parent().unwrap_or(Path::new("."));
        let mut title_hint: Option<String> = None;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line == "#EXTM3U" {
                continue;
            }
            if let Some(rest) = line.strip_prefix("#EXTINF:") {
                // #EXTINF: duration, Artist - Title
                title_hint = rest.splitn(2, ',').nth(1).map(|t| t.to_string());
            } else if !line.starts_with('#') {
                if line.starts_with("http://") || line.starts_with("https://") {
                    self.add_url(line.to_string(), title_hint.take());
                } else {
                    let p = if Path::new(line).is_absolute() {
                        PathBuf::from(line)
                    } else {
                        dir.join(line)
                    };
                    self.add(p);
                    title_hint = None;
                }
            }
        }
    }

    /// Import to PLS file
    pub fn import_pls(&mut self, path: &Path) {
        let Ok(content) = fs::read_to_string(path) else {
            return;
        };
        let mut files: std::collections::HashMap<u32, String> = Default::default();
        let mut titles: std::collections::HashMap<u32, String> = Default::default();

        for line in content.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("File") {
                if let Some((num_str, val)) = rest.split_once('=') {
                    if let Ok(n) = num_str.trim().parse::<u32>() {
                        files.insert(n, val.trim().to_string());
                    }
                }
            } else if let Some(rest) = line.strip_prefix("Title") {
                if let Some((num_str, val)) = rest.split_once('=') {
                    if let Ok(n) = num_str.trim().parse::<u32>() {
                        titles.insert(n, val.trim().to_string());
                    }
                }
            }
        }

        let mut indices: Vec<u32> = files.keys().cloned().collect();
        indices.sort();
        for idx in indices {
            if let Some(file) = files.get(&idx) {
                let title = titles.get(&idx).cloned();
                if file.starts_with("http") {
                    self.add_url(file.clone(), title);
                } else {
                    self.add(PathBuf::from(file));
                }
            }
        }
    }

    /// Export the playlist as M3U
    pub fn export_m3u(&self, path: &Path) -> std::io::Result<()> {
        let mut lines = vec!["#EXTM3U".to_string()];
        for t in &self.tracks {
            lines.push(format!("#EXTINF:-1,{}", t.title));
            lines.push(t.path.to_string_lossy().to_string());
        }
        fs::write(path, lines.join("\n"))
    }
}

impl Playlist {
    /// Import an M3U/PLS and return the added paths.
    pub fn load_m3u(&mut self, path: &std::path::Path) -> Vec<PathBuf> {
        let before = self.tracks.len();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if ext == "pls" {
            self.import_pls(path);
        } else {
            self.import_m3u(path);
        }
        self.tracks[before..]
            .iter()
            .map(|t| t.path.clone())
            .collect()
    }

    pub fn move_up(&mut self, idx: usize) {
        if idx > 0 && idx < self.tracks.len() {
            self.tracks.swap(idx - 1, idx);
            if self.current == Some(idx) {
                self.current = Some(idx - 1);
            } else if self.current == Some(idx - 1) {
                self.current = Some(idx);
            }
        }
    }

    pub fn move_down(&mut self, idx: usize) {
        if idx + 1 < self.tracks.len() {
            self.tracks.swap(idx, idx + 1);
            if self.current == Some(idx) {
                self.current = Some(idx + 1);
            } else if self.current == Some(idx + 1) {
                self.current = Some(idx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn sandbox_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join("rplayer_test").join(name);
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn track_from_path_uses_filename_as_title() {
        let t = Track::from_path(PathBuf::from("/tmp/example.mp4"));
        assert_eq!(t.title, "example.mp4");
    }

    #[test]
    fn track_from_url_uses_provided_title_or_url() {
        let t = Track::from_url(
            "http://example.com/stream".to_string(),
            Some("Live".to_string()),
        );
        assert_eq!(t.title, "Live");
        let t2 = Track::from_url("http://example.com/stream".to_string(), None);
        assert_eq!(t2.title, "http://example.com/stream");
    }

    #[test]
    fn playlist_add_and_next_prev_work() {
        let mut pl = Playlist::default();
        pl.add(PathBuf::from("a.mp3"));
        pl.add(PathBuf::from("b.mp3"));
        pl.current = Some(0);
        assert_eq!(pl.next().unwrap().path, PathBuf::from("b.mp3"));
        assert!(pl.prev().is_none());
        pl.current = Some(1);
        assert_eq!(pl.prev().unwrap().path, PathBuf::from("a.mp3"));
    }

    #[test]
    fn playlist_remove_updates_current_index() {
        let mut pl = Playlist::default();
        pl.add(PathBuf::from("a.mp3"));
        pl.add(PathBuf::from("b.mp3"));
        pl.current = Some(1);
        pl.remove(0);
        assert_eq!(pl.tracks.len(), 1);
        assert_eq!(pl.current, Some(0));
    }

    #[test]
    fn playlist_move_up_down_adjusts_selection() {
        let mut pl = Playlist::default();
        pl.add(PathBuf::from("a.mp3"));
        pl.add(PathBuf::from("b.mp3"));
        pl.current = Some(1);
        pl.move_up(1);
        assert_eq!(pl.tracks[0].path, PathBuf::from("b.mp3"));
        assert_eq!(pl.current, Some(0));
        pl.move_down(0);
        assert_eq!(pl.tracks[0].path, PathBuf::from("a.mp3"));
        assert_eq!(pl.current, Some(1));
    }

    #[test]
    fn import_m3u_handles_relative_paths_and_url_entries() {
        let dir = sandbox_dir("playlist_m3u");
        let m3u = dir.join("test.m3u");
        let content =
            "#EXTM3U\n#EXTINF:-1,Artist - Track\ntrack.mp3\nhttp://example.com/stream.mp3\n";
        fs::write(&m3u, content).unwrap();

        let mut pl = Playlist::default();
        pl.import_m3u(&m3u);

        assert_eq!(pl.tracks.len(), 2);
        assert_eq!(pl.tracks[0].title, "track.mp3");
        assert_eq!(pl.tracks[0].path, dir.join("track.mp3"));
        assert_eq!(pl.tracks[1].title, "http://example.com/stream.mp3");
        assert!(pl.tracks[1]
            .path
            .to_string_lossy()
            .starts_with("http://example.com/stream.mp3"));
    }

    #[test]
    fn import_pls_parses_file_and_title_entries() {
        let dir = sandbox_dir("playlist_pls");
        let pls = dir.join("test.pls");
        let content = "[playlist]\nFile1=http://example.com/audio.mp3\nTitle1=Remote Audio\nFile2=local.mp3\nTitle2=Local Audio\n";
        fs::write(&pls, content).unwrap();

        let mut pl = Playlist::default();
        pl.import_pls(&pls);

        assert_eq!(pl.tracks.len(), 2);
        assert_eq!(pl.tracks[0].title, "Remote Audio");
        assert_eq!(pl.tracks[1].title, "local.mp3");
    }

    #[test]
    fn export_m3u_writes_expected_playlist_file() {
        let dir = sandbox_dir("playlist_export");
        let out = dir.join("out.m3u");

        let mut pl = Playlist::default();
        pl.add(PathBuf::from("song.mp3"));
        pl.tracks[0].title = "Song Title".to_string();
        pl.export_m3u(&out).unwrap();

        let content = fs::read_to_string(out).unwrap();
        assert!(content.contains("#EXTINF:-1,Song Title"));
        assert!(content.contains("song.mp3"));
    }
}
