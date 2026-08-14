use dioxus::prelude::*;
use rplayer::chapters::Chapter;
use rplayer::i18n::{tr, Language};

#[component]
pub fn ChaptersTab(chapters: Vec<Chapter>, on_jump_bookmark: EventHandler<f64>) -> Element {
    let language = use_context::<Signal<Language>>();

    rsx! {
        h4 { class: "section-title", "{tr(language(), \"tools_modal.tab_chapters\")}" }
        if chapters.is_empty() {
            div { style: "padding: 20px; text-align: center; color: var(--text-muted); font-size: 13px;",
                "{tr(language(), \"tools_modal.chapters_empty\")}"
            }
        }
        div { class: "tracks-list",
            for chapter in chapters.iter() {
                {
                    let time = chapter.time;
                    let title = chapter.title.clone();
                    rsx! {
                        div { key: "{chapter.index}", class: "track-item",
                            div { style: "display: flex; gap: 8px; align-items: center;",
                                span { style: "font-family: monospace; color: var(--accent-color);", "{rplayer::player::PlayerState::format_time(time)}" }
                                span { "{title}" }
                            }
                            button { class: "btn-icon", onclick: move |_| on_jump_bookmark.call(time), "{tr(language(), \"tools_modal.jump_to_bookmark\")}" }
                        }
                    }
                }
            }
        }
    }
}
