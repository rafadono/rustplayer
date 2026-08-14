use dioxus::prelude::*;
use rplayer::history::HistoryEntry;
use rplayer::i18n::{tr, Language};
use std::path::PathBuf;

#[component]
pub fn HistoryTab(
    history_entries: Vec<HistoryEntry>,
    on_resume_history: EventHandler<PathBuf>,
    on_remove_history: EventHandler<PathBuf>,
    on_clear_history: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();

    rsx! {
        div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",
            h4 { class: "section-title", style: "margin: 0;", "{tr(language(), \"tools_modal.tab_history\")}" }
            if !history_entries.is_empty() {
                button { class: "btn-icon", onclick: move |_| on_clear_history.call(()), "{tr(language(), \"panel.clear_all\")}" }
            }
        }
        if history_entries.is_empty() {
            div { style: "padding: 20px; text-align: center; color: var(--text-muted); font-size: 13px;",
                "{tr(language(), \"panel.no_recent_files\")}"
            }
        }
        div { class: "tracks-list",
            for entry in history_entries.iter() {
                {
                    let path = entry.path.clone();
                    let path_remove = path.clone();
                    let title = entry.title.clone();
                    let resume_label = tr(language(), "tools_modal.history_resume_at")
                        .replacen("{}", &rplayer::player::PlayerState::format_time(entry.last_position), 1);
                    let count_label = tr(language(), "tools_modal.history_play_count")
                        .replacen("{}", &entry.play_count.to_string(), 1);
                    rsx! {
                        div { key: "{entry.path.display()}", class: "track-item",
                            div { style: "display: flex; flex-direction: column;",
                                span { "{title}" }
                                span { class: "eq-val-label", "{resume_label} · {count_label}" }
                            }
                            div { style: "display: flex; gap: 8px;",
                                button { class: "btn-icon", onclick: move |_| on_resume_history.call(path.clone()), "{tr(language(), \"tools_modal.history_resume_button\")}" }
                                button { class: "btn-icon", onclick: move |_| on_remove_history.call(path_remove.clone()), "🗑️" }
                            }
                        }
                    }
                }
            }
        }
    }
}
