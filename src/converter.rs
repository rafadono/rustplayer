//! converter.rs — Video/audio conversion via ffmpeg.
//!
//! Supports common presets: MP4 H.264, WebM, MP3, FLAC, AAC.

use crossbeam_channel::{bounded, Receiver};
use log::{debug, error};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

#[derive(Debug, Clone, PartialEq)]
pub enum ConvertPreset {
    Mp4H264,
    Mp4H265,
    WebmVp9,
    Mp3_320,
    Mp3_192,
    FlacLossless,
    AacM4a,
    OggVorbis,
}

impl ConvertPreset {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Mp4H264 => "MP4 / H.264 (compatible)",
            Self::Mp4H265 => "MP4 / H.265 (eficiente)",
            Self::WebmVp9 => "WebM / VP9",
            Self::Mp3_320 => "MP3 320kbps",
            Self::Mp3_192 => "MP3 192kbps",
            Self::FlacLossless => "FLAC (sin pérdida)",
            Self::AacM4a => "AAC / M4A",
            Self::OggVorbis => "OGG Vorbis",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::Mp4H264 | Self::Mp4H265 => "mp4",
            Self::WebmVp9 => "webm",
            Self::Mp3_320 | Self::Mp3_192 => "mp3",
            Self::FlacLossless => "flac",
            Self::AacM4a => "m4a",
            Self::OggVorbis => "ogg",
        }
    }

    pub fn ffmpeg_args(&self) -> Vec<&'static str> {
        match self {
            Self::Mp4H264 => vec![
                "-c:v", "libx264", "-crf", "23", "-c:a", "aac", "-b:a", "192k",
            ],
            Self::Mp4H265 => vec![
                "-c:v", "libx265", "-crf", "28", "-c:a", "aac", "-b:a", "192k",
            ],
            Self::WebmVp9 => vec![
                "-c:v",
                "libvpx-vp9",
                "-crf",
                "30",
                "-b:v",
                "0",
                "-c:a",
                "libopus",
            ],
            Self::Mp3_320 => vec!["-vn", "-c:a", "libmp3lame", "-b:a", "320k"],
            Self::Mp3_192 => vec!["-vn", "-c:a", "libmp3lame", "-b:a", "192k"],
            Self::FlacLossless => vec!["-vn", "-c:a", "flac"],
            Self::AacM4a => vec!["-vn", "-c:a", "aac", "-b:a", "256k"],
            Self::OggVorbis => vec!["-vn", "-c:a", "libvorbis", "-q:a", "6"],
        }
    }

    pub fn all() -> &'static [ConvertPreset] {
        &[
            ConvertPreset::Mp4H264,
            ConvertPreset::Mp4H265,
            ConvertPreset::WebmVp9,
            ConvertPreset::Mp3_320,
            ConvertPreset::Mp3_192,
            ConvertPreset::FlacLossless,
            ConvertPreset::AacM4a,
            ConvertPreset::OggVorbis,
        ]
    }
}

#[derive(Debug, Clone)]
pub enum ConvertStatus {
    Done(PathBuf),
    Error(String),
}

pub struct ConvertJob {
    pub status_rx: Receiver<ConvertStatus>,
}

impl ConvertJob {
    pub fn start(input: &Path, output: PathBuf, preset: &ConvertPreset) -> Option<Self> {
        let ffmpeg = which::which("ffmpeg").ok()?;
        let input = input.to_path_buf();
        let extra_args: Vec<String> = preset.ffmpeg_args().iter().map(|s| s.to_string()).collect();
        let (tx, rx) = bounded(4);

        thread::Builder::new()
            .name("convert-job".into())
            .spawn(move || {
                debug!(
                    "ffmpeg convert: {} -> {}",
                    input.display(),
                    output.display()
                );

                let mut cmd = Command::new(&ffmpeg);
                cmd.arg("-y")
                    .arg("-i")
                    .arg(&input)
                    .args(&extra_args)
                    .arg(&output)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null());

                match cmd.status() {
                    Ok(s) if s.success() => {
                        let _ = tx.send(ConvertStatus::Done(output));
                    }
                    Ok(s) => {
                        let _ = tx.send(ConvertStatus::Error(format!(
                            "ffmpeg salió con código {}",
                            s
                        )));
                    }
                    Err(e) => {
                        error!("convert error: {}", e);
                        let _ = tx.send(ConvertStatus::Error(e.to_string()));
                    }
                }
            })
            .ok()?;

        Some(ConvertJob { status_rx: rx })
    }

    pub fn default_output(input: &Path, preset: &ConvertPreset) -> PathBuf {
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        let parent = input.parent().unwrap_or(Path::new("."));
        parent.join(format!("{}_converted.{}", stem, preset.extension()))
    }
}
