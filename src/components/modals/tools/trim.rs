use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};

#[component]
pub fn TrimTab(
    duration: f64,
    on_start_trim: EventHandler<(f64, f64)>,
    trim_status: String,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let mut start_time = use_signal(|| 0.0f64);
    let mut end_time = use_signal(|| duration);
    rsx! {
        div { class: "control-group-col",
            h4 { class: "section-title", "{tr(language(), \"tools_modal.trim_title\")}" }
            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"tools_modal.trim_start_label\")}" }
                    span { class: "eq-val-label", "{start_time:.1}s" }
                }
                input {
                    r#type: "range", min: "0", max: "{duration}", value: "{start_time}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<f64>() { start_time.set(v); } }
                }
            }
            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"tools_modal.trim_end_label\")}" }
                    span { class: "eq-val-label", "{end_time:.1}s" }
                }
                input {
                    r#type: "range", min: "0", max: "{duration}", value: "{end_time}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<f64>() { end_time.set(v); } }
                }
            }
            div { style: "margin-top: 16px; display: flex; align-items: center; gap: 12px;",
                button { class: "btn-primary", onclick: move |_| on_start_trim.call((start_time(), end_time())), "{tr(language(), \"tools_modal.trim_export_button\")}" }
                if !trim_status.is_empty() {
                    span { style: "font-size: 12px; color: var(--text-muted);", "{trim_status}" }
                }
            }
        }
    }
}
