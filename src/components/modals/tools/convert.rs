use dioxus::prelude::*;
use rplayer::converter::ConvertPreset;
use rplayer::i18n::{tr, Language};

#[component]
pub fn ConvertTab(
    on_start_convert: EventHandler<ConvertPreset>,
    convert_status: String,
    has_current_file: bool,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let mut convert_preset = use_signal(|| ConvertPreset::Mp4H264);
    rsx! {
        div { class: "control-group-col",
            h4 { class: "section-title", "{tr(language(), \"tools_modal.convert_title\")}" }
            div { class: "slider-row",
                span { "{tr(language(), \"tools_modal.convert_preset_label\")}" }
                select {
                    class: "select-input",
                    onchange: move |e| {
                        if let Some(p) = ConvertPreset::all().iter().find(|p| p.label() == e.value()) {
                            convert_preset.set(p.clone());
                        }
                    },
                    for preset in ConvertPreset::all() {
                        option {
                            key: "{preset.label()}",
                            value: "{preset.label()}",
                            selected: *preset == convert_preset(),
                            "{preset.label()}"
                        }
                    }
                }
            }
            div { style: "margin-top: 16px; display: flex; align-items: center; gap: 12px;",
                button {
                    class: "btn-primary",
                    disabled: !has_current_file,
                    onclick: move |_| on_start_convert.call(convert_preset()),
                    "{tr(language(), \"tools_modal.convert_button\")}"
                }
                if !convert_status.is_empty() {
                    span { style: "font-size: 12px; color: var(--text-muted);", "{convert_status}" }
                }
            }
        }
    }
}
