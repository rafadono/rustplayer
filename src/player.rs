//! player.rs — Complete libmpv wrapper.
//!
//! Displays playback commands, audio/subtitle tracks,
//! chapters, appearance, EQ and screenshots.
//! All communication with mpv is via Arc<Mpv> thread-safe.

use crossbeam_channel::{bounded, Receiver, Sender};
use log::{debug, warn};
use serde::Deserialize;
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// ── Events ──────────────────────────────── ────────────────────────────────

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    FileLoaded { duration: f64, title: String },
    Paused,
    Playing,
    EndOfFile,
    PositionChanged,
    TracksChanged,
}

// ── Audio/subtitle track (parsed from mpv JSON) ──────────────────

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

// ── Visible state for UI ─────────────────────── ───────────────────────

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

// ──Player ──────────────────────────────── ─────────────────────────────────

pub struct Player {
    mpv: Arc<libmpv2::Mpv>,
    pub state: Arc<Mutex<PlayerState>>,
    event_rx: Receiver<PlayerEvent>,
}

impl Player {
    pub fn new(volume: i64, muted: bool, speed: f64) -> Result<Self, libmpv2::Error> {
        let mpv = Arc::new(libmpv2::Mpv::new()?);

        mpv.set_property("vo", "libmpv")?;
        mpv.set_property("audio-display", "no")?;
        mpv.set_property("keep-open", "yes")?;
        mpv.set_property("hr-seek", "yes")?;
        mpv.set_property("cache", "yes")?;
        mpv.set_property("demuxer-max-bytes", "150MiB")?;
        mpv.set_property("volume", volume)?;
        mpv.set_property("mute", muted)?;
        mpv.set_property("speed", speed)?;

        // Screenshots go to ~/Images or ~/Pictures
        let ss_dir = dirs::picture_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_default())
            .join("RPlayer");
        let _ = std::fs::create_dir_all(&ss_dir);
        mpv.set_property("screenshot-directory", ss_dir.to_string_lossy().as_ref())?;
        mpv.set_property("screenshot-format", "png")?;

        let state = Arc::new(Mutex::new(PlayerState {
            volume,
            muted,
            speed,
            ..Default::default()
        }));

        let (tx, rx) = bounded::<PlayerEvent>(128);
        Self::spawn_monitor_thread(Arc::clone(&mpv), Arc::clone(&state), tx);

        Ok(Self {
            mpv,
            state,
            event_rx: rx,
        })
    }

    fn spawn_monitor_thread(
        mpv: Arc<libmpv2::Mpv>,
        state: Arc<Mutex<PlayerState>>,
        tx: Sender<PlayerEvent>,
    ) {
        thread::Builder::new()
            .name("mpv-monitor".into())
            .spawn(move || {
                if !Self::run_event_loop(&mpv, &state, &tx) {
                    warn!("mpv: create_client no disponible, usando sondeo de propiedades");
                    Self::run_poll_loop(&mpv, &state, &tx);
                }
            })
            .expect("failed to spawn mpv-monitor thread");
    }

    fn run_event_loop(
        mpv: &Arc<libmpv2::Mpv>,
        state: &Arc<Mutex<PlayerState>>,
        tx: &Sender<PlayerEvent>,
    ) -> bool {
        let Ok(name) = CString::new("rplayer_events") else {
            return false;
        };
        let raw = unsafe { libmpv2_sys::mpv_create_client(mpv.ctx.as_ptr(), name.as_ptr()) };
        if raw.is_null() {
            return false;
        }

        let observe = |prop: &str, fmt: libmpv2_sys::mpv_format, id: u64| -> bool {
            let Ok(cprop) = CString::new(prop) else {
                return false;
            };
            unsafe { libmpv2_sys::mpv_observe_property(raw, id, cprop.as_ptr(), fmt) == 0 }
        };
        let _ = observe("time-pos", libmpv2_sys::mpv_format_MPV_FORMAT_DOUBLE, 0);
        let _ = observe("pause", libmpv2_sys::mpv_format_MPV_FORMAT_FLAG, 1);
        let _ = observe("duration", libmpv2_sys::mpv_format_MPV_FORMAT_DOUBLE, 2);
        let _ = observe("volume", libmpv2_sys::mpv_format_MPV_FORMAT_INT64, 3);
        let _ = observe("speed", libmpv2_sys::mpv_format_MPV_FORMAT_DOUBLE, 4);

        loop {
            let ev_ptr = unsafe { libmpv2_sys::mpv_wait_event(raw, 0.05) };
            if ev_ptr.is_null() {
                continue;
            }
            let ev = unsafe { &*ev_ptr };
            match ev.event_id {
                libmpv2_sys::mpv_event_id_MPV_EVENT_PROPERTY_CHANGE => {
                    if ev.data.is_null() {
                        continue;
                    }
                    let prop = unsafe { &*(ev.data as *const libmpv2_sys::mpv_event_property) };
                    if prop.name.is_null() {
                        continue;
                    }
                    let name = unsafe { CStr::from_ptr(prop.name) }.to_string_lossy();
                    let mut s = state.lock().unwrap();
                    match (name.as_ref(), prop.format) {
                        ("time-pos", libmpv2_sys::mpv_format_MPV_FORMAT_DOUBLE)
                            if !prop.data.is_null() =>
                        {
                            let v = unsafe { *(prop.data as *const f64) };
                            s.position = v;
                            let _ = tx.try_send(PlayerEvent::PositionChanged);
                            s.render_fps = mpv.get_property("estimated-vf-fps").unwrap_or(0.0);
                            s.dropped_frames = mpv.get_property("vo-drop-frame-count").unwrap_or(0);
                            let hwdec: String =
                                mpv.get_property("hwdec-current").unwrap_or_default();
                            s.hwdec_active = !hwdec.is_empty() && hwdec != "no";
                            s.buffer_seconds =
                                mpv.get_property("demuxer-cache-duration").unwrap_or(0.0);
                        }
                        ("duration", libmpv2_sys::mpv_format_MPV_FORMAT_DOUBLE)
                            if !prop.data.is_null() =>
                        {
                            s.duration = unsafe { *(prop.data as *const f64) };
                        }
                        ("pause", libmpv2_sys::mpv_format_MPV_FORMAT_FLAG)
                            if !prop.data.is_null() =>
                        {
                            let v = unsafe { *(prop.data as *const i32) } != 0;
                            s.paused = v;
                            let _ = tx.try_send(if v {
                                PlayerEvent::Paused
                            } else {
                                PlayerEvent::Playing
                            });
                        }
                        ("volume", libmpv2_sys::mpv_format_MPV_FORMAT_INT64)
                            if !prop.data.is_null() =>
                        {
                            s.volume = unsafe { *(prop.data as *const i64) };
                        }
                        ("speed", libmpv2_sys::mpv_format_MPV_FORMAT_DOUBLE)
                            if !prop.data.is_null() =>
                        {
                            s.speed = unsafe { *(prop.data as *const f64) };
                        }
                        _ => {}
                    }
                }
                libmpv2_sys::mpv_event_id_MPV_EVENT_FILE_LOADED => {
                    debug!("mpv: FileLoaded");
                    let tracks = Self::load_tracks(mpv);
                    let chapters = if let Ok(json) = mpv.get_property::<String>("chapter-list") {
                        crate::chapters::parse_chapter_list(&json)
                    } else {
                        vec![]
                    };
                    let mut s = state.lock().unwrap();
                    let title = s
                        .current_file
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Sin título".into());
                    s.title = title.clone();
                    s.audio_tracks = tracks
                        .iter()
                        .filter(|t| t.kind == TrackKind::Audio)
                        .cloned()
                        .collect();
                    s.sub_tracks = tracks
                        .iter()
                        .filter(|t| t.kind == TrackKind::Sub)
                        .cloned()
                        .collect();
                    s.chapters = chapters;
                    let dur = s.duration;
                    drop(s);
                    let _ = tx.try_send(PlayerEvent::FileLoaded {
                        duration: dur,
                        title,
                    });
                    let _ = tx.try_send(PlayerEvent::TracksChanged);
                }
                libmpv2_sys::mpv_event_id_MPV_EVENT_END_FILE => {
                    let _ = tx.try_send(PlayerEvent::EndOfFile);
                }
                libmpv2_sys::mpv_event_id_MPV_EVENT_SHUTDOWN => {
                    unsafe { libmpv2_sys::mpv_destroy(raw) };
                    return true;
                }
                _ => {}
            }
        }
    }

    fn run_poll_loop(
        mpv: &Arc<libmpv2::Mpv>,
        state: &Arc<Mutex<PlayerState>>,
        tx: &Sender<PlayerEvent>,
    ) {
        let mut prev_pos = 0.0;
        let mut prev_paused = false;
        let mut prev_path = String::new();

        loop {
            let pos: f64 = mpv.get_property("time-pos").unwrap_or(0.0);
            let paused: bool = mpv.get_property("pause").unwrap_or(false);
            let duration: f64 = mpv.get_property("duration").unwrap_or(0.0);
            let volume: i64 = mpv.get_property("volume").unwrap_or(0);
            let speed: f64 = mpv.get_property("speed").unwrap_or(1.0);
            let path: String = mpv.get_property("path").unwrap_or_default();

            let mut s = state.lock().unwrap();
            s.position = pos;
            s.paused = paused;
            s.duration = duration;
            s.volume = volume;
            s.speed = speed;
            s.render_fps = mpv.get_property("estimated-vf-fps").unwrap_or(0.0);
            s.dropped_frames = mpv.get_property("vo-drop-frame-count").unwrap_or(0);
            let hwdec: String = mpv.get_property("hwdec-current").unwrap_or_default();
            s.hwdec_active = !hwdec.is_empty() && hwdec != "no";
            s.buffer_seconds = mpv.get_property("demuxer-cache-duration").unwrap_or(0.0);

            if path != prev_path {
                if path.is_empty() {
                    s.current_file = None;
                    s.title.clear();
                    s.audio_tracks.clear();
                    s.sub_tracks.clear();
                    s.chapters.clear();
                    let _ = tx.try_send(PlayerEvent::EndOfFile);
                } else {
                    s.current_file = Some(PathBuf::from(&path));
                    let title = PathBuf::from(&path)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Sin título".into());
                    s.title = title.clone();
                    let tracks = Self::load_tracks(mpv);
                    s.audio_tracks = tracks
                        .iter()
                        .filter(|t| t.kind == TrackKind::Audio)
                        .cloned()
                        .collect();
                    s.sub_tracks = tracks
                        .iter()
                        .filter(|t| t.kind == TrackKind::Sub)
                        .cloned()
                        .collect();
                    s.chapters = if let Ok(json) = mpv.get_property::<String>("chapter-list") {
                        crate::chapters::parse_chapter_list(&json)
                    } else {
                        vec![]
                    };
                    let _ = tx.try_send(PlayerEvent::FileLoaded { duration, title });
                    let _ = tx.try_send(PlayerEvent::TracksChanged);
                }
            }
            drop(s);

            if (pos - prev_pos).abs() > 0.02 {
                let _ = tx.try_send(PlayerEvent::PositionChanged);
            }
            if paused != prev_paused {
                let _ = tx.try_send(if paused {
                    PlayerEvent::Paused
                } else {
                    PlayerEvent::Playing
                });
            }

            prev_pos = pos;
            prev_paused = paused;
            prev_path = path;
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn load_tracks(mpv: &libmpv2::Mpv) -> Vec<MediaTrack> {
        let Ok(json) = mpv.get_property::<String>("track-list") else {
            return vec![];
        };

        #[derive(Deserialize)]
        struct RawTrack {
            id: i64,
            #[serde(rename = "type")]
            kind: String,
            #[serde(default)]
            title: String,
            #[serde(rename = "lang", default)]
            lang: String,
            #[serde(default)]
            selected: bool,
        }

        let raw: Vec<RawTrack> = serde_json::from_str(&json).unwrap_or_default();
        raw.into_iter()
            .filter_map(|t| {
                let kind = match t.kind.as_str() {
                    "audio" => TrackKind::Audio,
                    "sub" => TrackKind::Sub,
                    "video" => TrackKind::Video,
                    _ => return None,
                };
                Some(MediaTrack {
                    id: t.id,
                    kind,
                    title: if t.title.is_empty() {
                        format!("Pista {}", t.id)
                    } else {
                        t.title
                    },
                    lang: t.lang,
                    selected: t.selected,
                })
            })
            .collect()
    }

    // ── Public API ──────────────────────────── ────────────────────────────

    pub fn drain_events(&self) -> Vec<PlayerEvent> {
        self.event_rx.try_iter().collect()
    }

    pub fn open(&self, path: &PathBuf) -> Result<(), libmpv2::Error> {
        let s = path.to_string_lossy();
        self.mpv.command("loadfile", &[s.as_ref()])?;
        self.state.lock().unwrap().current_file = Some(path.clone());
        Ok(())
    }

    pub fn toggle_pause(&self) -> Result<(), libmpv2::Error> {
        let p: bool = self.mpv.get_property("pause").unwrap_or(false);
        self.mpv.set_property("pause", !p)
    }

    pub fn set_paused(&self, paused: bool) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("pause", paused)
    }

    pub fn seek_absolute(&self, secs: f64) -> Result<(), libmpv2::Error> {
        self.mpv
            .command("seek", &[&format!("{:.3}", secs), "absolute"])
    }

    pub fn seek_relative(&self, secs: f64) -> Result<(), libmpv2::Error> {
        self.mpv
            .command("seek", &[&format!("{:.3}", secs), "relative"])
    }

    pub fn set_volume(&self, vol: i64) -> Result<(), libmpv2::Error> {
        let v = vol.clamp(0, 150);
        self.mpv.set_property("volume", v)?;
        self.state.lock().unwrap().volume = v;
        Ok(())
    }

    pub fn toggle_mute(&self) -> Result<(), libmpv2::Error> {
        let m = !self.state.lock().unwrap().muted;
        self.mpv.set_property("mute", m)?;
        self.state.lock().unwrap().muted = m;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), libmpv2::Error> {
        self.mpv.command("stop", &[])
    }

    pub fn set_speed(&self, speed: f64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("speed", speed)
    }

    pub fn set_aspect_ratio(
        &self,
        ratio: &crate::config::AspectRatio,
    ) -> Result<(), libmpv2::Error> {
        let val = ratio.to_mpv_value();
        self.mpv.set_property("video-aspect-override", val.as_str())
    }

    pub fn set_audio_track(&self, id: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("aid", id)
    }

    pub fn set_sub_track(&self, id: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sid", id)
    }

    pub fn disable_subs(&self) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sid", "no")
    }

    pub fn add_sub_file(&self, path: &std::path::Path) -> Result<(), libmpv2::Error> {
        self.mpv
            .command("sub-add", &[&path.to_string_lossy(), "select"])
    }

    pub fn set_sub_font_size(&self, size: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sub-font-size", size)
    }

    pub fn set_sub_color_rgb(&self, r: u8, g: u8, b: u8) -> Result<(), libmpv2::Error> {
        let color = format!("#{r:02X}{g:02X}{b:02X}");
        self.mpv.set_property("sub-color", color.as_str())
    }

    pub fn set_sub_opacity(&self, opacity: f32) -> Result<(), libmpv2::Error> {
        self.mpv
            .set_property("sub-opacity", opacity.clamp(0.0, 1.0) as f64)
    }

    pub fn set_sub_font_family(&self, family: &str) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sub-font", family)
    }

    pub fn set_sub_bold(&self, bold: bool) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sub-bold", bold)
    }

    pub fn screenshot(&self) -> Result<(), libmpv2::Error> {
        self.mpv.command("screenshot", &["video"])
    }

    pub fn set_audio_filters(&self, eq: &crate::equalizer::Equalizer, loudnorm: bool) {
        let af = eq.to_mpv_af_chain(loudnorm);
        let _ = self.mpv.set_property("af", af.as_str());
    }

    pub fn mpv_handle(&self) -> Arc<libmpv2::Mpv> {
        Arc::clone(&self.mpv)
    }
}

// ── Additional methods (v0.3) ────────────────────── ───────────────────────

impl Player {
    /// Frame by frame (forward). Requires the video to be paused.
    pub fn frame_step(&self) -> Result<(), libmpv2::Error> {
        self.mpv.command("frame-step", &[])
    }

    pub fn frame_back_step(&self) -> Result<(), libmpv2::Error> {
        self.mpv.command("frame-back-step", &[])
    }

    /// Audio delay with respect to video in seconds (+/-).
    pub fn set_audio_delay(&self, delay: f64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("audio-delay", delay)
    }

    /// Subtitle delay in seconds (+/-).
    pub fn set_sub_delay(&self, delay: f64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sub-delay", delay)
    }

    /// Second subtitle track (for double subtitles).
    pub fn set_second_sub_track(&self, id: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("secondary-sid", id)
    }

    pub fn disable_second_subs(&self) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("secondary-sid", "no")
    }

    ///Complete image controls.
    pub fn apply_image_controls(&self, ic: &crate::image_controls::ImageControls) {
        ic.apply(&self.mpv);
    }

    /// Loudness normalization (ffmpeg/mpv loudnorm).
    /// Gets the current position of the frame (for A-B and frame step).
    pub fn current_position(&self) -> f64 {
        self.state.lock().unwrap().position
    }

    /// Complete media information of the current file.
    pub fn media_info(&self) -> Option<crate::media_info::MediaInfo> {
        let s = self.state.lock().unwrap();
        let file = s.current_file.clone()?;
        drop(s);
        Some(crate::media_info::MediaInfo::from_mpv(&self.mpv, &file))
    }
}
