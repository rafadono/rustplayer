use dioxus::prelude::*;

#[component]
pub fn VideoStage(
    has_file: bool,
    current_title: String,
    on_drop_file: EventHandler<String>,
) -> Element {
    rsx! {
        main { class: "video-container",
            if !has_file {
                div { class: "empty-state",
                    div { class: "empty-icon", "🎬" }
                    h2 { style: "font-size: 18px; font-weight: 500;", "Ningún archivo cargado" }
                    p { style: "font-size: 13px;", "Haz clic en 'Abrir archivo' para seleccionar un video o música." }
                }
            } else {
                div { id: "video-canvas", style: "width: 100%; height: 100%; position: relative;",
                    div { style: "position: absolute; top: 12px; left: 16px; background: rgba(0,0,0,0.6); padding: 4px 10px; border-radius: 4px; font-size: 12px; color: #fff;",
                        "{current_title}"
                    }
                }
            }
        }
    }
}
