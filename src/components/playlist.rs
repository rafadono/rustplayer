use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn PlaylistPanel(
    items: Vec<PathBuf>,
    current_index: Option<usize>,
    on_select_item: EventHandler<usize>,
    on_close: EventHandler<()>,
) -> Element {
    rsx! {
        aside { class: "side-panel",
            div { class: "panel-header",
                span { "📋 Lista de reproducción" }
                button {
                    class: "btn-icon",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }
            div { class: "playlist-items",
                if items.is_empty() {
                    div { style: "padding: 20px; text-align: center; color: var(--text-muted); font-size: 13px;",
                        "Lista vacía. Arrastra archivos o usa 'Abrir archivo'."
                    }
                }
                for (idx, path) in items.iter().enumerate() {
                    {
                        let filename = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                        let is_active = current_index == Some(idx);
                        let class_name = if is_active { "playlist-item active" } else { "playlist-item" };

                        rsx! {
                            div {
                                key: "{idx}",
                                class: "{class_name}",
                                onclick: move |_| on_select_item.call(idx),
                                span { style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                    "{idx + 1}. {filename}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
