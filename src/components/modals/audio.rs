use crate::components::TrackItem;
use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};

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
    let mut lastfm_user_input = use_signal(String::new);
    let mut lastfm_pass_input = use_signal(String::new);
    let tabs = [
        (AudioTab::Equalizer, "audio.tab_equalizer"),
        (AudioTab::Tracks, "audio.tab_tracks"),
        (AudioTab::Karaoke, "audio.tab_karaoke"),
    ];

    let band_labels = [
        "60Hz", "170Hz", "310Hz", "600Hz", "1kHz", "3kHz", "6kHz", "12kHz", "14kHz", "16kHz",
    ];
    // (internal id sent to on_select_preset / matched in main.rs, i18n key for the display label)
    let presets = [
        ("Flat", "audio.preset_flat"),
        ("Bass Boost", "audio.preset_bass_boost"),
        ("Treble Boost", "audio.preset_treble_boost"),
        ("Vocal", "audio.preset_vocal"),
        ("Rock", "audio.preset_rock"),
        ("Pop", "audio.preset_pop"),
        ("Classical", "audio.preset_classical"),
        ("Jazz", "audio.preset_jazz"),
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
                                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;",
                                    label { style: "display: flex; align-items: center; gap: 8px; cursor: pointer;",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{eq_enabled}",
                                            onchange: move |e| on_toggle_eq.call(e.value() == "true")
                                        }
                                        span { "{tr(language(), \"audio.enable_eq\")}" }
                                    }
                                    select {
                                        class: "select-input",
                                        value: "{eq_preset}",
                                        onchange: move |e| on_select_preset.call(e.value()),
                                        for (id, label_key) in presets {
                                            option { value: "{id}", "{tr(language(), label_key)}" }
                                        }
                                    }
                                }
                                div { class: "eq-bands-container",
                                    for (idx, &val) in eq_bands.iter().enumerate() {
                                        {
                                            let label = band_labels.get(idx).copied().unwrap_or(tr(language(), "audio.band_fallback"));
                                            rsx! {
                                                div { class: "eq-band-col",
                                                    span { class: "eq-val-label", "{val:+.1}dB" }
                                                    input {
                                                        class: "eq-slider-vertical",
                                                        r#type: "range",
                                                        min: "-12", max: "12", step: "0.5",
                                                        value: "{val}",
                                                        disabled: !eq_enabled,
                                                        oninput: move |e| {
                                                            if let Ok(v) = e.value().parse::<f64>() {
                                                                on_band_change.call((idx, v));
                                                            }
                                                        }
                                                    }
                                                    span { class: "eq-freq-label", "{label}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            AudioTab::Tracks => rsx! {
                                h4 { class: "section-title", "{tr(language(), \"audio.tracks_available\")}" }
                                div { class: "tracks-list",
                                    for track in audio_tracks {
                                        {
                                            let is_sel = current_audio == Some(track.id);
                                            let class_name = if is_sel { "track-item active" } else { "track-item" };
                                            let track_id = track.id;
                                            rsx! {
                                                div {
                                                    key: "{track.id}",
                                                    class: "{class_name}",
                                                    onclick: move |_| on_select_audio.call(track_id),
                                                    span { "{track.title}" }
                                                    span { class: "eq-val-label", "{track.lang}" }
                                                }
                                            }
                                        }
                                    }
                                }
                                div { class: "slider-row", style: "margin-top: 16px;",
                                    div { style: "display: flex; justify-content: space-between;",
                                        span { "{tr(language(), \"audio.delay_label\")}" }
                                        span { class: "eq-val-label", "{audio_delay:+.1}s" }
                                    }
                                    input {
                                        r#type: "range", min: "-5.0", max: "5.0", step: "0.1",
                                        value: "{audio_delay}",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse::<f64>() {
                                                on_change_audio_delay.call(v);
                                            }
                                        }
                                    }
                                }
                                h4 { class: "section-title", style: "margin-top: 24px;", "{tr(language(), \"audio.lastfm_section_title\")}" }
                                p { style: "font-size: 13px; color: var(--text-muted); margin-bottom: 12px;",
                                    "{tr(language(), \"audio.lastfm_description\")}"
                                }
                                if lastfm_connected {
                                    div { style: "display: flex; align-items: center; gap: 12px;",
                                        span { "{tr(language(), \"audio.lastfm_connected_as\")}" strong { "{lastfm_username}" } }
                                        button { class: "btn-icon", onclick: move |_| on_lastfm_disconnect.call(()), "{tr(language(), \"audio.lastfm_disconnect\")}" }
                                    }
                                } else {
                                    div { class: "control-group-col", style: "gap: 8px; max-width: 320px;",
                                        input {
                                            class: "select-input",
                                            r#type: "text",
                                            placeholder: "{tr(language(), \"audio.lastfm_username_placeholder\")}",
                                            value: "{lastfm_user_input}",
                                            oninput: move |e| lastfm_user_input.set(e.value()),
                                        }
                                        input {
                                            class: "select-input",
                                            r#type: "password",
                                            placeholder: "{tr(language(), \"audio.lastfm_password_placeholder\")}",
                                            value: "{lastfm_pass_input}",
                                            oninput: move |e| lastfm_pass_input.set(e.value()),
                                        }
                                        button {
                                            class: "btn-primary",
                                            onclick: move |_| on_lastfm_connect.call((lastfm_user_input(), lastfm_pass_input())),
                                            "{tr(language(), \"audio.lastfm_connect_button\")}"
                                        }
                                        if !lastfm_status.is_empty() {
                                            span { style: "font-size: 12px; color: var(--text-muted);", "{lastfm_status}" }
                                        }
                                    }
                                }
                            },
                            AudioTab::Karaoke => rsx! {
                                div { class: "control-group-col",
                                    label { style: "display: flex; align-items: center; gap: 8px; cursor: pointer;",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{karaoke_enabled}",
                                            onchange: move |e| on_toggle_karaoke.call(e.value() == "true")
                                        }
                                        span { "{tr(language(), \"audio.enable_karaoke\")}" }
                                    }
                                    div { class: "slider-row", style: "margin-top: 12px;",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { "{tr(language(), \"audio.pitch_label\")}" }
                                            span { class: "eq-val-label", "{karaoke_pitch:+.1} {tr(language(), \"audio.semitones_unit\")}" }
                                        }
                                        input {
                                            r#type: "range", min: "-6.0", max: "6.0", step: "0.5",
                                            value: "{karaoke_pitch}",
                                            oninput: move |e| {
                                                if let Ok(v) = e.value().parse::<f64>() {
                                                    on_change_pitch.call(v);
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
