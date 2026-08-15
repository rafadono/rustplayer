use crate::state::AppState;
use dioxus::prelude::*;
use rplayer::config::Config as AppConfig;
use rplayer::history::History;
use rplayer::notes::NoteStore;
use rplayer::playback::player::Player;
use rplayer::playlist::Playlist;
use rplayer::sleep_timer::SleepTimer;
use rplayer::thumbnail::ThumbnailCache;
use std::sync::{Arc, Mutex};

pub fn init_app_state(initial_config: &AppConfig) -> AppState {
    let eq_bands = {
        let mut v: Vec<f64> = initial_config
            .equalizer
            .peq_filters
            .iter()
            .map(|f| f64::from(f.gain_db))
            .collect();
        v.resize(10, 0.0);
        v
    };

    let player_ref = use_signal(|| {
        if let Ok(p) = Player::new(
            initial_config.volume,
            initial_config.muted,
            initial_config.speed,
        ) {
            p.apply_image_controls(&initial_config.image_controls);
            let _ = p.set_audio_delay(initial_config.audio_delay);
            let _ = p.set_sub_delay(initial_config.sub_delay);
            let _ = p.set_aspect_ratio(&initial_config.aspect_ratio);
            let _ = p.set_crop(initial_config.crop);
            let _ = p.set_deinterlace(initial_config.deinterlace);
            let _ = p.set_sub_font_size(initial_config.sub_font_size);
            let _ = p.set_sub_pos(initial_config.sub_pos);
            if initial_config.equalizer.enabled
                || initial_config.karaoke_enabled
                || initial_config.karaoke_pitch.abs() > 0.01
            {
                p.set_audio_filters(
                    &initial_config.equalizer,
                    initial_config.loudnorm,
                    initial_config.karaoke_enabled,
                    initial_config.karaoke_pitch,
                );
            }
            Some(Arc::new(Mutex::new(p)))
        } else {
            None
        }
    });

    AppState {
        is_dark_mode: use_signal(|| !initial_config.theme.is_light()),
        playing: use_signal(|| false),
        paused: use_signal(|| false),
        volume: use_signal(|| initial_config.volume),
        muted: use_signal(|| initial_config.muted),
        speed: use_signal(|| initial_config.speed),
        time_pos: use_signal(|| 0.0f64),
        duration: use_signal(|| 0.0f64),
        current_title: use_signal(String::new),
        has_file: use_signal(|| false),
        ab_repeat: use_signal(rplayer::ab_repeat::AbRepeat::default),
        is_fullscreen: use_signal(|| false),
        language: use_signal(|| initial_config.language),
        show_playlist: use_signal(|| false),
        playlist: use_signal(Playlist::default),
        history: use_signal(History::load),
        notes: use_signal(NoteStore::load),
        chapters: use_signal(Vec::new),
        show_metrics: use_signal(|| initial_config.show_metrics_overlay),
        render_fps: use_signal(|| 0.0f64),
        dropped_frames: use_signal(|| 0i64),
        hwdec_active: use_signal(|| false),
        buffer_seconds: use_signal(|| 0.0f64),
        thumb_cache: use_signal(ThumbnailCache::new),
        hover_thumb: use_signal(|| None),
        show_audio_modal: use_signal(|| false),
        audio_tab: use_signal(|| crate::components::modals::AudioTab::Equalizer),
        show_video_modal: use_signal(|| false),
        video_tab: use_signal(|| crate::components::modals::VideoTab::Subtitles),
        show_tools_modal: use_signal(|| false),
        tools_tab: use_signal(|| crate::components::modals::ToolsTab::Bookmarks),
        eq_enabled: use_signal(|| initial_config.equalizer.enabled),
        eq_bands: use_signal(|| eq_bands),
        eq_preset: use_signal(|| "Flat".to_string()),
        brightness: use_signal(|| initial_config.image_controls.brightness),
        contrast: use_signal(|| initial_config.image_controls.contrast),
        saturation: use_signal(|| initial_config.image_controls.saturation),
        hue: use_signal(|| initial_config.image_controls.hue),
        gamma: use_signal(|| initial_config.image_controls.gamma),
        audio_delay: use_signal(|| initial_config.audio_delay),
        sub_delay: use_signal(|| initial_config.sub_delay),
        aspect_ratio: use_signal(|| initial_config.aspect_ratio.clone()),
        crop: use_signal(|| initial_config.crop),
        deinterlace: use_signal(|| initial_config.deinterlace),
        loudnorm: use_signal(|| initial_config.loudnorm),
        sub_font_size: use_signal(|| initial_config.sub_font_size),
        sub_color: use_signal(|| initial_config.sub_color.clone()),
        sub_pos: use_signal(|| initial_config.sub_pos),
        karaoke_enabled: use_signal(|| initial_config.karaoke_enabled),
        karaoke_pitch: use_signal(|| initial_config.karaoke_pitch),
        bookmarks: use_signal(Vec::new),
        convert_job: use_signal(|| None),
        convert_status: use_signal(String::new),
        show_open_url_modal: use_signal(|| false),
        open_url_error: use_signal(String::new),
        update_status: use_signal(String::new),
        update_info: use_signal(|| None),
        update_rx: use_signal(|| None),
        install_update_rx: use_signal(|| None),
        audio_tracks: use_signal(Vec::new),
        sub_tracks: use_signal(Vec::new),
        current_audio: use_signal(|| None),
        current_sub: use_signal(|| None),
        media_info_sig: use_signal(|| None),
        sub_search: use_signal(|| None),
        sub_search_status: use_signal(String::new),
        sub_search_results: use_signal(Vec::new),
        trim_job: use_signal(|| None),
        trim_status: use_signal(String::new),
        lastfm_config: use_signal(|| initial_config.lastfm.clone()),
        lastfm_status: use_signal(String::new),
        lastfm_pending_user: use_signal(String::new),
        lastfm_login_rx: use_signal(|| None),
        remote_server: use_signal(|| None),
        sleep_timer_sig: use_signal(SleepTimer::new),
        config: use_signal(|| initial_config.clone()),
        player_ref,
    }
}
