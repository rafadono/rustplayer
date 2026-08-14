use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};

#[component]
pub fn EqualizerTab(
    eq_bands: Vec<f64>,
    eq_enabled: bool,
    eq_preset: String,
    on_band_change: EventHandler<(usize, f64)>,
    on_toggle_eq: EventHandler<bool>,
    on_select_preset: EventHandler<String>,
) -> Element {
    let language = use_context::<Signal<Language>>();
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
    }
}
