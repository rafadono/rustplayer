use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    FileLoaded { duration: f64, title: String },
    Paused,
    Playing,
    EndOfFile,
    PositionChanged,
    TracksChanged,
}

#[derive(Debug, Clone)]
pub struct MediaTrack {
    pub id: i64,
    pub kind: TrackKind,
    pub title: String,
    pub lang: String,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrackKind {
    Audio,
    Sub,
    Video,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerState {
    pub position: f64,
    pub duration: f64,
    pub paused: bool,
    pub volume: i64,
    pub muted: bool,
    pub speed: f64,
    pub current_file: Option<PathBuf>,
    pub title: String,
    pub audio_tracks: Vec<MediaTrack>,
    pub sub_tracks: Vec<MediaTrack>,
    pub chapters: Vec<crate::chapters::Chapter>,
    pub render_fps: f64,
    pub dropped_frames: i64,
    pub hwdec_active: bool,
    pub buffer_seconds: f64,
}

impl PlayerState {
    pub fn progress_ratio(&self) -> f32 {
        if self.duration > 0.0 {
            (self.position / self.duration).clamp(0.0, 1.0) as f32
        } else {
            0.0
        }
    }

    pub fn format_time(secs: f64) -> String {
        let s = secs.max(0.0) as u64;
        let h = s / 3600;
        let m = (s % 3600) / 60;
        let s = s % 60;
        if h > 0 {
            format!("{:02}:{:02}:{:02}", h, m, s)
        } else {
            format!("{:02}:{:02}", m, s)
        }
    }
}
