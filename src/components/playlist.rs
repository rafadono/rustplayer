use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};
use rplayer::playlist::Track;

#[component]
pub fn PlaylistPanel(
    tracks: Vec<Track>,
    current_index: Option<usize>,
    on_select_item: EventHandler<usize>,
    on_close: EventHandler<()>,
    on_import: EventHandler<()>,
    on_export: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();
    rsx! {
        aside { class: "side-panel",
            div { class: "panel-header",
                span { "📋 {tr(language(), \"panel.playlist_title\")}" }
                div { style: "display: flex; gap: 4px;",
                    button {
                        class: "btn-icon",
                        onclick: move |_| on_import.call(()),
                        title: "{tr(language(), \"playlist.import_tooltip\")}",
                        "⬆"
                    }
                    button {
                        class: "btn-icon",
                        onclick: move |_| on_export.call(()),
                        title: "{tr(language(), \"playlist.export_tooltip\")}",
                        "⬇"
                    }
                    button {
                        class: "btn-icon",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }
            }
            div { class: "playlist-items",
                if tracks.is_empty() {
                    div { style: "padding: 20px; text-align: center; color: var(--text-muted); font-size: 13px;",
                        "{tr(language(), \"playlist.empty_message\")}"
                    }
                }
                for (idx, track) in tracks.iter().enumerate() {
                    {
                        let title = track.title.clone();
                        let is_active = current_index == Some(idx);
                        let class_name = if is_active { "playlist-item active" } else { "playlist-item" };

                        rsx! {
                            div {
                                key: "{idx}",
                                class: "{class_name}",
                                onclick: move |_| on_select_item.call(idx),
                                span { style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                    "{idx + 1}. {title}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
