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
    aspect_ratio: rplayer::config::AspectRatio,
    crop: f64,
    deinterlace: bool,
    on_change_aspect_ratio: EventHandler<rplayer::config::AspectRatio>,
    on_change_crop: EventHandler<f64>,
    on_change_deinterlace: EventHandler<bool>,
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

            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"video_modal.aspect_ratio\")}" }
                    span { class: "eq-val-label", "{aspect_ratio.label()}" }
                }
                select {
                    class: "select-input",
                    onchange: move |e| {
                        let ar = match e.value().as_str() {
                            "Auto" => rplayer::config::AspectRatio::Auto,
                            "16:9" => rplayer::config::AspectRatio::Ratio16_9,
                            "4:3" => rplayer::config::AspectRatio::Ratio4_3,
                            "21:9" => rplayer::config::AspectRatio::Ratio21_9,
                            "1:1" => rplayer::config::AspectRatio::Ratio1_1,
                            _ => rplayer::config::AspectRatio::Auto,
                        };
                        on_change_aspect_ratio.call(ar);
                    },
                    for opt in rplayer::config::AspectRatio::all() {
                        {
                            let is_sel = *opt == aspect_ratio;
                            rsx! {
                                option { value: "{opt.label()}", selected: is_sel, "{opt.label()}" }
                            }
                        }
                    }
                }
            }

            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"video_modal.crop\")}" }
                    span { class: "eq-val-label", "{crop:.2}" }
                }
                input {
                    r#type: "range", min: "0.0", max: "1.0", step: "0.01", value: "{crop}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<f64>() { on_change_crop.call(v); } }
                }
            }

            div { class: "slider-row", style: "display: flex; align-items: center; justify-content: space-between;",
                span { "{tr(language(), \"video_modal.deinterlace\")}" }
                input {
                    r#type: "checkbox", checked: "{deinterlace}",
                    oninput: move |e| on_change_deinterlace.call(e.value() == "true")
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
