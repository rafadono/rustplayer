use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};

#[component]
pub fn KaraokeTab(
    karaoke_enabled: bool,
    karaoke_pitch: f64,
    on_toggle_karaoke: EventHandler<bool>,
    on_change_pitch: EventHandler<f64>,
) -> Element {
    let language = use_context::<Signal<Language>>();

    rsx! {
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
    }
}
