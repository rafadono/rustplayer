use super::{MediaTrack, Player, PlayerEvent, PlayerState, TrackKind};
use crossbeam_channel::Sender;
use log::{debug, warn};
use serde::Deserialize;
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

impl Player {
    pub(crate) fn spawn_monitor_thread(
        mpv: Arc<libmpv2::Mpv>,
        state: Arc<Mutex<PlayerState>>,
        tx: Sender<PlayerEvent>,
    ) {
        thread::Builder::new()
            .name("mpv-monitor".into())
            .spawn(move || {
                if !Self::run_event_loop(&mpv, &state, &tx) {
                    warn!("mpv: create_client not available, using property polling");
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
                        .unwrap_or_else(|| "Unknown title".into());
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
                        .unwrap_or_else(|| "Unknown title".into());
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
            title: Option<String>,
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
                    title: t
                        .title
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| format!("Track {}", t.id)),
                    lang: t.lang,
                    selected: t.selected,
                })
            })
            .collect()
    }
}
