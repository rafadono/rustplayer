use crate::components::modals::{AudioTab, ToolsTab, VideoTab};
use crate::components::PlayerControls;
use dioxus::prelude::*;

#[component]
pub fn AppPlayerControls() -> Element {
    let state = use_context::<crate::state::AppState>();

    let mut playing = state.playing;
    let mut paused = state.paused;
    let mut time_pos = state.time_pos;
    let duration = state.duration;
    let mut volume = state.volume;
    let mut muted = state.muted;
    let mut speed = state.speed;
    let mut ab_repeat = state.ab_repeat;
    let mut hover_thumb = state.hover_thumb;
    let thumb_cache = state.thumb_cache;
    let player_ref = state.player_ref;
    let mut config = state.config;
    let has_file = state.has_file;
    let mut audio_tab = state.audio_tab;
    let mut show_audio_modal = state.show_audio_modal;
    let mut tools_tab = state.tools_tab;
    let mut show_tools_modal = state.show_tools_modal;
    let mut video_tab = state.video_tab;
    let mut show_video_modal = state.show_video_modal;
    let mut is_fullscreen = state.is_fullscreen;

    let window = dioxus::desktop::use_window();

    rsx! {
        PlayerControls {
            playing: playing(),
            paused: paused(),
            time_pos: time_pos(),
            duration: duration(),
            volume: volume(),
            muted: muted(),
            speed: speed(),
            ab_loop_active: ab_repeat.read().active,
            ab_loop_label: ab_repeat.read().label(),
            hover_thumb_uri: hover_thumb(),
            on_seek_preview: move |t: f64| {
                hover_thumb.set(thumb_cache.read().nearest_data_uri(t));
            },
            on_seek_preview_end: move |_| hover_thumb.set(None),
            on_toggle_play: move |_| {
                if has_file() {
                    paused.toggle();
                    playing.set(!paused());
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            let _ = p.set_paused(paused());
                        }
                    }
                }
            },
            on_seek: move |pos| {
                time_pos.set(pos);
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() {
                        let _ = p.seek_absolute(pos);
                    }
                }
            },
            on_volume_change: move |v| {
                volume.set(v);
                config.write().volume = v;
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() {
                        let _ = p.set_volume(v);
                    }
                }
            },
            on_toggle_mute: move |_| {
                muted.toggle();
                config.write().muted = muted();
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() {
                        let _ = p.toggle_mute();
                    }
                }
            },
            on_change_speed: move |spd| {
                speed.set(spd);
                config.write().speed = spd;
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() {
                        let _ = p.set_speed(spd);
                    }
                }
            },
            on_toggle_ab_repeat: move |_| {
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() {
                        ab_repeat.write().cycle(time_pos(), &p.mpv_handle());
                    }
                }
            },
            on_open_karaoke: move |_| {
                audio_tab.set(AudioTab::Karaoke);
                show_audio_modal.set(true);
            },
            on_open_trim: move |_| {
                tools_tab.set(ToolsTab::Trim);
                show_tools_modal.set(true);
            },
            on_open_opensubtitles: move |_| {
                video_tab.set(VideoTab::Subtitles);
                show_video_modal.set(true);
            },
            on_toggle_fullscreen: move |_| {
                is_fullscreen.toggle();
                window.set_fullscreen(is_fullscreen());
            },
        }
    }
}
