use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

use crate::components::modals::{AudioTab, ToolsTab, VideoTab};
use crate::components::TrackItem;
use crossbeam_channel::Receiver;
use rplayer::ab_repeat::AbRepeat;
use rplayer::bookmarks::Bookmark;
use rplayer::chapters::Chapter;
use rplayer::config::Config as AppConfig;
use rplayer::converter::ConvertJob;
use rplayer::history::History;
use rplayer::i18n::Language;
use rplayer::lastfm::LastFmConfig;
use rplayer::media_info::MediaInfo;
use rplayer::notes::NoteStore;
use rplayer::playback::player::Player;
use rplayer::playlist::Playlist;
use rplayer::remote::RemoteServer;
use rplayer::services::opensubtitles::{SubResult, SubSearchJob};
use rplayer::sleep_timer::SleepTimer;
use rplayer::thumbnail::ThumbnailCache;
use rplayer::trim::TrimJob;
use rplayer::updater::UpdateInfo;

#[derive(Clone, Copy)]
pub struct AppState {
    pub is_dark_mode: Signal<bool>,
    pub playing: Signal<bool>,
    pub paused: Signal<bool>,
    pub volume: Signal<i64>,
    pub muted: Signal<bool>,
    pub speed: Signal<f64>,
    pub time_pos: Signal<f64>,
    pub duration: Signal<f64>,
    pub current_title: Signal<String>,
    pub has_file: Signal<bool>,
    pub ab_repeat: Signal<AbRepeat>,
    pub is_fullscreen: Signal<bool>,
    pub language: Signal<Language>,
    pub show_playlist: Signal<bool>,
    pub playlist: Signal<Playlist>,
    pub history: Signal<History>,
    pub notes: Signal<NoteStore>,
    pub chapters: Signal<Vec<Chapter>>,
    pub show_metrics: Signal<bool>,
    pub render_fps: Signal<f64>,
    pub dropped_frames: Signal<i64>,
    pub hwdec_active: Signal<bool>,
    pub buffer_seconds: Signal<f64>,
    pub thumb_cache: Signal<ThumbnailCache>,
    pub hover_thumb: Signal<Option<String>>,
    pub show_audio_modal: Signal<bool>,
    pub audio_tab: Signal<AudioTab>,
    pub show_video_modal: Signal<bool>,
    pub video_tab: Signal<VideoTab>,
    pub show_tools_modal: Signal<bool>,
    pub tools_tab: Signal<ToolsTab>,
    pub eq_enabled: Signal<bool>,
    pub eq_bands: Signal<Vec<f64>>,
    pub eq_preset: Signal<String>,
    pub brightness: Signal<i64>,
    pub contrast: Signal<i64>,
    pub saturation: Signal<i64>,
    pub hue: Signal<i64>,
    pub gamma: Signal<i64>,
    pub audio_delay: Signal<f64>,
    pub sub_delay: Signal<f64>,
    pub aspect_ratio: Signal<rplayer::config::AspectRatio>,
    pub crop: Signal<f64>,
    pub deinterlace: Signal<bool>,
    pub loudnorm: Signal<bool>,
    pub sub_font_size: Signal<i64>,
    pub sub_color: Signal<String>,
    pub sub_pos: Signal<i64>,
    pub karaoke_enabled: Signal<bool>,
    pub karaoke_pitch: Signal<f64>,
    pub bookmarks: Signal<Vec<Bookmark>>,
    pub convert_job: Signal<Option<ConvertJob>>,
    pub convert_status: Signal<String>,
    pub show_open_url_modal: Signal<bool>,
    pub open_url_error: Signal<String>,
    pub update_status: Signal<String>,
    pub update_info: Signal<Option<UpdateInfo>>,
    pub update_rx: Signal<Option<Receiver<Result<UpdateInfo, String>>>>,
    pub install_update_rx: Signal<Option<Receiver<Result<(), String>>>>,
    pub audio_tracks: Signal<Vec<TrackItem>>,
    pub sub_tracks: Signal<Vec<TrackItem>>,
    pub current_audio: Signal<Option<i64>>,
    pub current_sub: Signal<Option<i64>>,
    pub media_info_sig: Signal<Option<MediaInfo>>,
    pub sub_search: Signal<Option<SubSearchJob>>,
    pub sub_search_status: Signal<String>,
    pub sub_search_results: Signal<Vec<SubResult>>,
    pub trim_job: Signal<Option<TrimJob>>,
    pub trim_status: Signal<String>,
    pub lastfm_config: Signal<LastFmConfig>,
    pub lastfm_status: Signal<String>,
    pub lastfm_pending_user: Signal<String>,
    pub lastfm_login_rx: Signal<Option<Receiver<Result<String, String>>>>,
    pub remote_server: Signal<Option<Arc<RemoteServer>>>,
    pub sleep_timer_sig: Signal<SleepTimer>,
    pub config: Signal<AppConfig>,
    pub player_ref: Signal<Option<Arc<Mutex<Player>>>>,
}
