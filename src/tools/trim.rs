//! trim.rs — Trim video/audio without re-encoding via ffmpeg -c copy.
//!
//! Spawn ffmpeg in a separate thread. Return progress via channel.

use crossbeam_channel::{bounded, Receiver};
use log::{debug, error};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

#[derive(Debug, Clone)]
pub enum TrimStatus {
    Done(PathBuf),
    Error(String),
}

pub struct TrimJob {
    pub status_rx: Receiver<TrimStatus>,
}

impl TrimJob {
    /// Starts clipping in the background. Returns None if ffmpeg is not in PATH.
    pub fn start(input: &Path, start: f64, end: f64, output: PathBuf) -> Option<Self> {
        let ffmpeg = which::which("ffmpeg").ok()?;
        let input = input.to_path_buf();
        let (tx, rx) = bounded(4);

        thread::Builder::new()
            .name("trim-job".into())
            .spawn(move || {
                let start_str = format_time(start);
                let duration_str = format_time(end - start);

                debug!(
                    "ffmpeg trim: {} -> {} (from {} dur {})",
                    input.display(),
                    output.display(),
                    start_str,
                    duration_str
                );

                let result = Command::new(&ffmpeg)
                    .args([
                        "-y",
                        "-ss",
                        &start_str,
                        "-i",
                        &input.to_string_lossy(),
                        "-t",
                        &duration_str,
                        "-c",
                        "copy", // without re-encoding
                        "-avoid_negative_ts",
                        "make_zero",
                        &output.to_string_lossy(),
                    ])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();

                match result {
                    Ok(s) if s.success() => {
                        let _ = tx.send(TrimStatus::Done(output));
                    }
                    Ok(s) => {
                        let _ =
                            tx.send(TrimStatus::Error(format!("ffmpeg salió con código {}", s)));
                    }
                    Err(e) => {
                        error!("trim error: {}", e);
                        let _ = tx.send(TrimStatus::Error(e.to_string()));
                    }
                }
            })
            .ok()?;

        Some(TrimJob { status_rx: rx })
    }
}

/// Generate output path with _trim suffix
pub fn default_output(input: &Path, start: f64, end: f64) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    let ext = input.extension().unwrap_or_default().to_string_lossy();
    let parent = input.parent().unwrap_or(Path::new("."));
    parent.join(format!("{}_trim_{:.0}-{:.0}.{}", stem, start, end, ext))
}

fn format_time(secs: f64) -> String {
    let s = secs as u64;
    let h = s / 3600;
    let m = (s % 3600) / 60;
    let s = s % 60;
    let ms = ((secs - secs.floor()) * 1000.0) as u64;
    format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
}
