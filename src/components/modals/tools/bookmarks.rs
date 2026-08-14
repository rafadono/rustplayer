use dioxus::prelude::*;
use rplayer::bookmarks::Bookmark;
use rplayer::i18n::{tr, Language};

#[component]
pub fn BookmarksTab(
    bookmarks: Vec<Bookmark>,
    current_time: f64,
    on_add_bookmark: EventHandler<f64>,
    on_jump_bookmark: EventHandler<f64>,
    on_delete_bookmark: EventHandler<usize>,
) -> Element {
    let language = use_context::<Signal<Language>>();

    rsx! {
        div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",
            h4 { class: "section-title", style: "margin: 0;", "{tr(language(), \"tools_modal.bookmarks_of_file\")}" }
            button { class: "btn-primary", onclick: move |_| on_add_bookmark.call(current_time), "{tr(language(), \"tools_modal.add_bookmark_button\")}" }
        }
        div { class: "tracks-list",
            for (idx, b) in bookmarks.iter().enumerate() {
                {
                    let pos = b.position;
                    let label = b.label.clone();
                    rsx! {
                        div { key: "{b.id}", class: "track-item",
                            div { style: "display: flex; gap: 8px; align-items: center;",
                                span { style: "font-family: monospace; color: var(--accent-color);", "{pos:.1}s" }
                                span { "{label}" }
                            }
                            div { style: "display: flex; gap: 8px;",
                                button { class: "btn-icon", onclick: move |_| on_jump_bookmark.call(pos), "{tr(language(), \"tools_modal.jump_to_bookmark\")}" }
                                button { class: "btn-icon", onclick: move |_| on_delete_bookmark.call(idx), "🗑️" }
                            }
                        }
                    }
                }
            }
        }
    }
}
