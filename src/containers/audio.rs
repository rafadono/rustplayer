use dioxus::prelude::*;
use rplayer::equalizer::Equalizer;
use rplayer::i18n::tr;
use rplayer::lastfm;
use std::thread;

use crate::components::modals::AudioModal;

#[component]
pub fn AppAudioModal() -> Element {
    let state = use_context::<crate::state::AppState>();
    let mut show_audio_modal = state.show_audio_modal;
    let mut audio_tab = state.audio_tab;
    let mut config = state.config;
    let player_ref = state.player_ref;
    let mut eq_enabled = state.eq_enabled;
    let mut eq_bands = state.eq_bands;
    let mut eq_preset = state.eq_preset;
    let mut loudnorm = state.loudnorm;
    let mut audio_delay = state.audio_delay;
    let mut karaoke_enabled = state.karaoke_enabled;
    let mut karaoke_pitch = state.karaoke_pitch;
    let mut lastfm_status = state.lastfm_status;
    let mut lastfm_pending_user = state.lastfm_pending_user;
    let mut lastfm_login_rx = state.lastfm_login_rx;

    // Some variables need to be derived from state signals
    let audio_tracks = state.audio_tracks.read().clone();
    let current_audio = *state.current_audio.read();
    let language = state.language;
    let lastfm_config_enabled = state.lastfm_config.read().enabled;
    let lastfm_config_username = state.lastfm_config.read().username.clone();

    rsx! {
        AudioModal {
            active_tab: audio_tab(),
            on_change_tab: move |tab| audio_tab.set(tab),
            on_close: move |_| show_audio_modal.set(false),

            eq_bands: eq_bands(),
            eq_enabled: eq_enabled(),
            eq_preset: eq_preset(),
            loudnorm: loudnorm(),
            on_toggle_loudnorm: move |en: bool| {
                loudnorm.set(en);
                config.write().loudnorm = en;
                let eq_snapshot = config.read().equalizer.clone();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq_snapshot, en, karaoke_enabled(), karaoke_pitch()); }
                }
                config.read().save();
            },
            on_band_change: move |(idx, val): (usize, f64)| {
                if idx < eq_bands.read().len() {
                    eq_bands.write()[idx] = val;
                }
                {
                    let mut cfg = config.write();
                    if let Some(f) = cfg.equalizer.peq_filters.get_mut(idx) {
                        f.gain_db = val as f32;
                        f.enabled = true;
                    }
                    cfg.equalizer.enabled = true;
                }
                eq_enabled.set(true);
                let eq_snapshot = config.read().equalizer.clone();
                let loudnorm = config.read().loudnorm;
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq_snapshot, loudnorm, karaoke_enabled(), karaoke_pitch()); }
                }
                config.read().save();
            },
            on_toggle_eq: move |en: bool| {
                eq_enabled.set(en);
                config.write().equalizer.enabled = en;
                let eq_snapshot = config.read().equalizer.clone();
                let loudnorm = config.read().loudnorm;
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq_snapshot, loudnorm, karaoke_enabled(), karaoke_pitch()); }
                }
                config.read().save();
            },
            on_select_preset: move |p: String| {
                eq_preset.set(p.clone());
                // Only Flat/Bass Boost/Vocal/Rock have a dedicated backend preset;
                // the remaining UI options fall back to Flat until more are added.
                let eq = match p.as_str() {
                    "Bass Boost" => Equalizer::preset_bass_boost(),
                    "Vocal" => Equalizer::preset_vocal(),
                    "Rock" => Equalizer::preset_rock(),
                    _ => Equalizer::preset_flat(),
                };
                let mut bands: Vec<f64> = eq.peq_filters.iter().map(|f| f64::from(f.gain_db)).collect();
                bands.resize(10, 0.0);
                eq_bands.set(bands);
                eq_enabled.set(eq.enabled);
                let loudnorm = config.read().loudnorm;
                config.write().equalizer = eq.clone();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq, loudnorm, karaoke_enabled(), karaoke_pitch()); }
                }
                config.read().save();
            },

            audio_tracks: audio_tracks,
            current_audio: current_audio,
            audio_delay: audio_delay(),
            on_select_audio: move |id| {
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.set_audio_track(id); }
                }
            },
            on_change_audio_delay: move |d| {
                audio_delay.set(d);
                config.write().audio_delay = d;
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.set_audio_delay(d); }
                }
            },

            karaoke_enabled: karaoke_enabled(),
            karaoke_pitch: karaoke_pitch(),
            on_toggle_karaoke: move |en: bool| {
                karaoke_enabled.set(en);
                config.write().karaoke_enabled = en;
                let eq_snapshot = config.read().equalizer.clone();
                let loudnorm = config.read().loudnorm;
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq_snapshot, loudnorm, en, karaoke_pitch()); }
                }
                config.read().save();
            },
            on_change_pitch: move |pitch: f64| {
                karaoke_pitch.set(pitch);
                config.write().karaoke_pitch = pitch;
                let eq_snapshot = config.read().equalizer.clone();
                let loudnorm = config.read().loudnorm;
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq_snapshot, loudnorm, karaoke_enabled(), pitch); }
                }
                config.read().save();
            },

            lastfm_connected: lastfm_config_enabled,
            lastfm_username: lastfm_config_username,
            lastfm_status: lastfm_status(),
            on_lastfm_connect: move |(user, pass): (String, String)| {
                lastfm_status.set(tr(language(), "lastfm.connecting").to_string());
                lastfm_pending_user.set(user.clone());
                let (tx, rx) = crossbeam_channel::bounded(1);
                thread::spawn(move || {
                    let _ = tx.send(lastfm::get_session(&user, &pass));
                });
                lastfm_login_rx.set(Some(rx));
            },
            on_lastfm_disconnect: move |_| {
                config.write().lastfm = rplayer::lastfm::LastFmConfig::default();
                config.read().save();
                lastfm_status.set(String::new());
            },
        }
    }
}
