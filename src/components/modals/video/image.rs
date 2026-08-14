use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};

#[component]
pub fn ImageTab(
    brightness: i64,
    contrast: i64,
    saturation: i64,
    hue: i64,
    gamma: i64,
    on_change_brightness: EventHandler<i64>,
    on_change_contrast: EventHandler<i64>,
    on_change_saturation: EventHandler<i64>,
    on_change_hue: EventHandler<i64>,
    on_change_gamma: EventHandler<i64>,
    on_reset_image: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();

    rsx! {
        div { class: "control-group-col",
            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"video_modal.brightness_label\")}" }
                    span { class: "eq-val-label", "{brightness}" }
                }
                input {
                    r#type: "range", min: "-100", max: "100", value: "{brightness}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_brightness.call(v); } }
                }
            }

            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"video_modal.contrast_label\")}" }
                    span { class: "eq-val-label", "{contrast}" }
                }
                input {
                    r#type: "range", min: "-100", max: "100", value: "{contrast}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_contrast.call(v); } }
                }
            }

            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"video_modal.saturation_label\")}" }
                    span { class: "eq-val-label", "{saturation}" }
                }
                input {
                    r#type: "range", min: "-100", max: "100", value: "{saturation}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_saturation.call(v); } }
                }
            }

            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"video_modal.hue_label\")}" }
                    span { class: "eq-val-label", "{hue}" }
                }
                input {
                    r#type: "range", min: "-100", max: "100", value: "{hue}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_hue.call(v); } }
                }
            }

            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"video_modal.gamma_label\")}" }
                    span { class: "eq-val-label", "{gamma}" }
                }
                input {
                    r#type: "range", min: "-100", max: "100", value: "{gamma}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_gamma.call(v); } }
                }
            }

            div { style: "margin-top: 12px;",
                button { class: "btn-primary", style: "background: var(--bg-surface-hover); color: var(--text-main); border: 1px solid var(--border-color);",
                    onclick: move |_| on_reset_image.call(()),
                    "{tr(language(), \"video_modal.reset_defaults\")}"
                }
            }
        }
    }
}
