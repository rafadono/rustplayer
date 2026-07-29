//! thumbnail.rs — Generation of previews for the seekbar.
//!
//! Extract N frames from the video using ffmpeg in a background thread.
//! Frames are stored in /tmp/rplayer_thumbs/ as PNG.
//! The UI can query the closest texture at the time you hover the cursor.

use log::debug;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

const THUMB_COUNT: u32 = 20;
const THUMB_WIDTH: u32 = 160;

#[derive(Clone)]
pub struct Thumbnail {
    pub time: f64,
    pub path: PathBuf,
}

pub struct ThumbnailCache {
    pub thumbs: Arc<Mutex<Vec<Thumbnail>>>,
    pub ready: bool,
}

impl ThumbnailCache {
    pub fn new() -> Self {
        Self {
            thumbs: Arc::new(Mutex::new(Vec::new())),
            ready: false,
        }
    }

    /// Starts the background generation. Only if ffmpeg is available.
    pub fn generate(&mut self, file: &Path, duration: f64) -> bool {
        if duration <= 0.0 || duration.is_nan() || duration.is_infinite() {
            return false;
        }

        let ffmpeg = match which::which("ffmpeg") {
            Ok(p) => p,
            Err(_) => return false,
        };

        let thumbs_arc = Arc::clone(&self.thumbs);
        let file = file.to_path_buf();

        // Clear previous thumbs
        {
            let mut t = thumbs_arc.lock().unwrap();
            t.clear();
        }
        self.ready = false;

        let pid = std::process::id();
        let out_dir = std::env::temp_dir().join(format!("rplayer_thumbs_{}", pid));
        let _ = std::fs::remove_dir_all(&out_dir);
        let _ = std::fs::create_dir_all(&out_dir);

        thread::Builder::new()
            .name("thumb-gen".into())
            .spawn(move || {
                let fps = THUMB_COUNT as f64 / duration;
                let fps_str = format!("{:.6}", fps);
                let out_pattern = out_dir.join("thumb_%03d.png");

                let status = std::process::Command::new(&ffmpeg)
                    .args([
                        "-y",
                        "-i",
                        &file.to_string_lossy(),
                        "-vf",
                        &format!("fps={},scale={}:-1", fps_str, THUMB_WIDTH),
                        &out_pattern.to_string_lossy(),
                    ])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status();

                let interval = duration / THUMB_COUNT as f64;
                let mut results = Vec::new();

                if status.map(|s| s.success()).unwrap_or(false) {
                    for i in 1..=THUMB_COUNT {
                        let out = out_dir.join(format!("thumb_{:03}.png", i));
                        if out.is_file() {
                            let t = (i as f64 - 1.0) * interval;
                            results.push(Thumbnail { time: t, path: out });
                        }
                    }
                }

                debug!("thumbnails generados: {}/{}", results.len(), THUMB_COUNT);
                let mut guard = thumbs_arc.lock().unwrap();
                *guard = results;
            })
            .ok();

        true
    }

    /// Returns the path of the thumbnail closest to time `t`.
    pub fn nearest(&self, t: f64) -> Option<PathBuf> {
        let thumbs = self.thumbs.lock().unwrap();
        if thumbs.is_empty() {
            return None;
        }
        thumbs
            .iter()
            .min_by(|a, b| (a.time - t).abs().total_cmp(&(b.time - t).abs()))
            .map(|th| th.path.clone())
    }

    pub fn is_ready(&self) -> bool {
        !self.thumbs.lock().unwrap().is_empty()
    }

    ///Clean temporary files
    pub fn clear_temp() {
        let pid = std::process::id();
        let dir = std::env::temp_dir().join(format!("rplayer_thumbs_{}", pid));
        let _ = std::fs::remove_dir_all(dir);
    }
}

#[cfg(test)]
mod tests {
    use super::{Thumbnail, ThumbnailCache};
    use std::path::PathBuf;

    #[test]
    fn nearest_with_nan_time_does_not_panic() {
        let cache = ThumbnailCache::new();
        {
            let mut guard = cache.thumbs.lock().expect("thumb lock");
            guard.push(Thumbnail {
                time: 1.0,
                path: PathBuf::from("a.png"),
            });
            guard.push(Thumbnail {
                time: f64::NAN,
                path: PathBuf::from("b.png"),
            });
        }
        let _ = cache.nearest(0.5);
    }

    #[test]
    fn nearest_returns_closest_thumbnail() {
        let cache = ThumbnailCache::new();
        {
            let mut guard = cache.thumbs.lock().unwrap();
            guard.push(Thumbnail {
                time: 1.0,
                path: PathBuf::from("a.png"),
            });
            guard.push(Thumbnail {
                time: 3.0,
                path: PathBuf::from("b.png"),
            });
        }

        let nearest = cache.nearest(2.4).unwrap();
        assert_eq!(nearest, PathBuf::from("b.png"));
    }

    #[test]
    fn is_ready_reports_true_when_thumbnails_exist() {
        let cache = ThumbnailCache::new();
        {
            let mut guard = cache.thumbs.lock().unwrap();
            guard.push(Thumbnail {
                time: 1.0,
                path: PathBuf::from("a.png"),
            });
        }
        assert!(cache.is_ready());
    }
}
