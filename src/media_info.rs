//! media_info.rs — Extract metadata from the file via libmpv.

use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct MediaInfo {
    pub filename: String,
    pub format: String,
    pub duration: f64,
    pub file_size: Option<u64>,

    pub video_codec: String,
    pub width: i64,
    pub height: i64,
    pub fps: f64,
    pub video_bitrate: i64,
    pub color_space: String,

    pub audio_codec: String,
    pub audio_channels: i64,
    pub sample_rate: i64,
    pub audio_bitrate: i64,

    pub sub_count: usize,
    pub track_count: usize,
}

impl MediaInfo {
    ///Load from mpv properties (call after FileLoaded).
    pub fn from_mpv(mpv: &libmpv2::Mpv, path: &std::path::Path) -> Self {
        let mut info = Self::default();

        info.filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        // Extension as format
        info.format = path
            .extension()
            .map(|e| e.to_string_lossy().to_uppercase())
            .unwrap_or_default();

        // Duration
        info.duration = mpv.get_property("duration").unwrap_or(0.0);

        // Video
        info.width = mpv.get_property("width").unwrap_or(0);
        info.height = mpv.get_property("height").unwrap_or(0);
        info.fps = mpv.get_property("fps").unwrap_or(0.0);
        info.video_bitrate = mpv.get_property("video-bitrate").unwrap_or(0);

        // Read video codec of the active track via track-list JSON
        if let Ok(json) = mpv.get_property::<String>("track-list") {
            #[derive(Deserialize)]
            struct T {
                #[serde(rename = "type")]
                kind: String,
                #[serde(default)]
                codec: String,
                #[serde(default)]
                selected: bool,
                #[serde(rename = "audio-channels", default)]
                channels: i64,
                #[serde(rename = "demux-samplerate", default)]
                samplerate: i64,
            }
            if let Ok(tracks) = serde_json::from_str::<Vec<T>>(&json) {
                info.track_count = tracks.len();
                info.sub_count = tracks.iter().filter(|t| t.kind == "sub").count();
                for t in &tracks {
                    if t.selected && t.kind == "video" {
                        info.video_codec = t.codec.clone();
                    }
                    if t.selected && t.kind == "audio" {
                        info.audio_codec = t.codec.clone();
                        info.audio_channels = t.channels;
                        info.sample_rate = t.samplerate;
                    }
                }
            }
        }

        info.audio_bitrate = mpv.get_property("audio-bitrate").unwrap_or(0);

        // Color space
        info.color_space = mpv
            .get_property::<String>("video-params/colormatrix")
            .unwrap_or_default();

        // File size
        info.file_size = std::fs::metadata(path).ok().map(|m| m.len());

        info
    }

    pub fn resolution(&self) -> String {
        if self.width > 0 && self.height > 0 {
            format!("{}×{}", self.width, self.height)
        } else {
            "—".into()
        }
    }

    pub fn size_str(&self) -> String {
        match self.file_size {
            Some(b) if b >= 1_073_741_824 => format!("{:.2} GB", b as f64 / 1_073_741_824.0),
            Some(b) if b >= 1_048_576 => format!("{:.1} MB", b as f64 / 1_048_576.0),
            Some(b) if b >= 1_024 => format!("{:.0} KB", b as f64 / 1_024.0),
            Some(b) => format!("{} B", b),
            None => "—".into(),
        }
    }

    pub fn video_bitrate_str(&self) -> String {
        if self.video_bitrate > 0 {
            format!("{} kbps", self.video_bitrate / 1000)
        } else {
            "—".into()
        }
    }

    pub fn audio_bitrate_str(&self) -> String {
        if self.audio_bitrate > 0 {
            format!("{} kbps", self.audio_bitrate / 1000)
        } else {
            "—".into()
        }
    }

    pub fn channel_str(&self) -> String {
        match self.audio_channels {
            1 => "Mono".into(),
            2 => "Estéreo".into(),
            6 => "5.1".into(),
            8 => "7.1".into(),
            n if n > 0 => format!("{} canales", n),
            _ => "—".into(),
        }
    }
}
