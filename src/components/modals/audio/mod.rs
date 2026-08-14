pub mod equalizer;
pub mod karaoke;
pub mod tracks;

use crate::components::TrackItem;
use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};

use equalizer::EqualizerTab;
use karaoke::KaraokeTab;
use tracks::TracksTab;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioTab {
    Equalizer,
    Tracks,
    Karaoke,
}

#[component]
pub fn AudioModal(
    active_tab: AudioTab,
    on_change_tab: EventHandler<AudioTab>,
    on_close: EventHandler<()>,
    // EQ
    eq_bands: Vec<f64>,
    eq_enabled: bool,
    eq_preset: String,
    on_band_change: EventHandler<(usize, f64)>,
    on_toggle_eq: EventHandler<bool>,
    on_select_preset: EventHandler<String>,
    // Audio Tracks
    audio_tracks: Vec<TrackItem>,
    current_audio: Option<i64>,
    audio_delay: f64,
    on_select_audio: EventHandler<i64>,
    on_change_audio_delay: EventHandler<f64>,
    // Karaoke
    karaoke_enabled: bool,
    karaoke_pitch: f64,
    on_toggle_karaoke: EventHandler<bool>,
    on_change_pitch: EventHandler<f64>,
    // Last.fm
    lastfm_connected: bool,
    lastfm_username: String,
    lastfm_status: String,
    on_lastfm_connect: EventHandler<(String, String)>,
    on_lastfm_disconnect: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let tabs = [
        (AudioTab::Equalizer, "audio.tab_equalizer"),
        (AudioTab::Tracks, "audio.tab_tracks"),
        (AudioTab::Karaoke, "audio.tab_karaoke"),
    ];

    rsx! {
        div { class: "modal-overlay", onclick: move |_| on_close.call(()),
            div { class: "modal-card-large", onclick: move |e| e.stop_propagation(),
                div { class: "modal-header",
                    h3 { "{tr(language(), \"audio.modal_title\")}" }
                    button { class: "btn-icon", onclick: move |_| on_close.call(()), "✕" }
                }

                div { class: "modal-layout-tabbed",
                    nav { class: "modal-sidebar-tabs",
                        for (tab, label_key) in tabs {
                            {
                                let is_active = active_tab == tab;
                                let class_name = if is_active { "tab-button active" } else { "tab-button" };
                                rsx! {
                                    button {
                                        key: "{label_key}",
                                        class: "{class_name}",
                                        onclick: move |_| on_change_tab.call(tab),
                                        "{tr(language(), label_key)}"
                                    }
                                }
                            }
                        }
                    }

                    main { class: "modal-tab-content",
                        match active_tab {
                            AudioTab::Equalizer => rsx! {
                                EqualizerTab {
                                    eq_bands,
                                    eq_enabled,
                                    eq_preset,
                                    on_band_change,
                                    on_toggle_eq,
                                    on_select_preset,
                                }
                            },
                            AudioTab::Tracks => rsx! {
                                TracksTab {
                                    audio_tracks,
                                    current_audio,
                                    audio_delay,
                                    on_select_audio,
                                    on_change_audio_delay,
                                    lastfm_connected,
                                    lastfm_username,
                                    lastfm_status,
                                    on_lastfm_connect,
                                    on_lastfm_disconnect,
                                }
                            },
                            AudioTab::Karaoke => rsx! {
                                KaraokeTab {
                                    karaoke_enabled,
                                    karaoke_pitch,
                                    on_toggle_karaoke,
                                    on_change_pitch,
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
