use dioxus::prelude::*;
use crate::components::TrackItem;

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
) -> Element {
    let tabs = [
        (AudioTab::Equalizer, "🎚️ Ecualizador PEQ"),
        (AudioTab::Tracks, "🎧 Pistas de Audio"),
        (AudioTab::Karaoke, "🎤 Karaoke & Tono"),
    ];

    let band_labels = ["60Hz", "170Hz", "310Hz", "600Hz", "1kHz", "3kHz", "6kHz", "12kHz", "14kHz", "16kHz"];
    let presets = ["Flat", "Bass Boost", "Treble Boost", "Vocal", "Rock", "Pop", "Classical", "Jazz"];

    rsx! {
        div { class: "modal-overlay", onclick: move |_| on_close.call(()),
            div { class: "modal-card-large", onclick: move |e| e.stop_propagation(),
                div { class: "modal-header",
                    h3 { "🎛️ Centro de Audio y Sonido" }
                    button { class: "btn-icon", onclick: move |_| on_close.call(()), "✕" }
                }

                div { class: "modal-layout-tabbed",
                    nav { class: "modal-sidebar-tabs",
                        for (tab, label) in tabs {
                            {
                                let is_active = active_tab == tab;
                                let class_name = if is_active { "tab-button active" } else { "tab-button" };
                                rsx! {
                                    button {
                                        key: "{label}",
                                        class: "{class_name}",
                                        onclick: move |_| on_change_tab.call(tab),
                                        "{label}"
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
                                        span { "Activar ecualizador paramétrico" }
                                    }
                                    select {
                                        class: "select-input",
                                        value: "{eq_preset}",
                                        onchange: move |e| on_select_preset.call(e.value()),
                                        for p in presets {
                                            option { value: "{p}", "{p}" }
                                        }
                                    }
                                }
                                div { class: "eq-bands-container",
                                    for (idx, &val) in eq_bands.iter().enumerate() {
                                        {
                                            let label = band_labels.get(idx).copied().unwrap_or("Band");
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
                                h4 { class: "section-title", "Pistas de Audio Disponibles" }
                                div { class: "tracks-list",
                                    for tr in audio_tracks {
                                        {
                                            let is_sel = current_audio == Some(tr.id);
                                            let class_name = if is_sel { "track-item active" } else { "track-item" };
                                            let tr_id = tr.id;
                                            rsx! {
                                                div {
                                                    key: "{tr.id}",
                                                    class: "{class_name}",
                                                    onclick: move |_| on_select_audio.call(tr_id),
                                                    span { "{tr.title}" }
                                                    span { class: "eq-val-label", "{tr.lang}" }
                                                }
                                            }
                                        }
                                    }
                                }
                                div { class: "slider-row", style: "margin-top: 16px;",
                                    div { style: "display: flex; justify-content: space-between;",
                                        span { "Desfase de Audio" }
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
                                h4 { class: "section-title", style: "margin-top: 24px;", "🎵 Integración Last.fm Scrobbler" }
                                p { style: "font-size: 13px; color: var(--text-muted); margin-bottom: 12px;",
                                    "Registra automáticamente tus canciones escuchadas en tu perfil de Last.fm."
                                }
                                button { class: "btn-primary", "🔑 Conectar Cuenta Last.fm" }
                            },
                            AudioTab::Karaoke => rsx! {
                                div { class: "control-group-col",
                                    label { style: "display: flex; align-items: center; gap: 8px; cursor: pointer;",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{karaoke_enabled}",
                                            onchange: move |e| on_toggle_karaoke.call(e.value() == "true")
                                        }
                                        span { "Activar Supresión Vocal (Modo Karaoke)" }
                                    }
                                    div { class: "slider-row", style: "margin-top: 12px;",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { "Ajuste de Tono (Pitch Shift)" }
                                            span { class: "eq-val-label", "{karaoke_pitch:+.1} semitonos" }
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
