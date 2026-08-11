use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};
use rplayer::streaming::{self, RadioStation};

#[component]
pub fn OpenUrlModal(
    on_close: EventHandler<()>,
    on_open: EventHandler<String>,
    error: String,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let mut url_input = use_signal(String::new);
    let stations: Vec<RadioStation> = streaming::default_radio_stations();

    rsx! {
        div { class: "modal-overlay", onclick: move |_| on_close.call(()),
            div { class: "modal-card", onclick: move |e| e.stop_propagation(),
                div { class: "modal-header",
                    h3 { "{tr(language(), \"open_url_modal.title\")}" }
                    button { class: "btn-icon", onclick: move |_| on_close.call(()), "✕" }
                }
                div { style: "padding: 16px;",
                    div { style: "display: flex; gap: 8px; align-items: center;",
                        input {
                            class: "select-input",
                            style: "flex: 1;",
                            r#type: "text",
                            placeholder: "{tr(language(), \"open_url_modal.placeholder\")}",
                            value: "{url_input}",
                            oninput: move |e| url_input.set(e.value()),
                        }
                        button {
                            class: "btn-primary",
                            disabled: url_input.read().trim().is_empty(),
                            onclick: move |_| on_open.call(url_input.read().trim().to_string()),
                            "{tr(language(), \"open_url_modal.play_button\")}"
                        }
                    }
                    if !error.is_empty() {
                        p { style: "font-size: 12px; color: var(--danger-color, #e05252); margin-top: 8px;", "{error}" }
                    }

                    h4 { class: "section-title", style: "margin-top: 20px;", "{tr(language(), \"open_url_modal.quick_radio_title\")}" }
                    div { class: "tracks-list",
                        for station in stations {
                            {
                                let url = station.url.clone();
                                let name = station.name.clone();
                                rsx! {
                                    div { key: "{station.url}", class: "track-item",
                                        span { "{name}" }
                                        button { class: "btn-icon", onclick: move |_| on_open.call(url.clone()), "▶" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
