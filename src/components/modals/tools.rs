use dioxus::prelude::*;
use rplayer::bookmarks::Bookmark;
use rplayer::chapters::Chapter;
use rplayer::history::HistoryEntry;
use rplayer::i18n::{tr, Language};
use rplayer::media_info::MediaInfo;
use rplayer::notes::Note;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolsTab {
    Bookmarks,
    Trim,
    MediaInfo,
    History,
    Notes,
    Chapters,
    Settings,
}

#[component]
pub fn ToolsModal(
    active_tab: ToolsTab,
    on_change_tab: EventHandler<ToolsTab>,
    on_close: EventHandler<()>,
    // Bookmarks
    bookmarks: Vec<Bookmark>,
    current_time: f64,
    on_add_bookmark: EventHandler<f64>,
    on_jump_bookmark: EventHandler<f64>,
    on_delete_bookmark: EventHandler<usize>,
    // Trim
    duration: f64,
    on_start_trim: EventHandler<(f64, f64)>,
    trim_status: String,
    // Media Info
    filename: String,
    media_info: Option<MediaInfo>,
    // History
    history_entries: Vec<HistoryEntry>,
    on_resume_history: EventHandler<PathBuf>,
    on_remove_history: EventHandler<PathBuf>,
    on_clear_history: EventHandler<()>,
    // Notes
    notes: Vec<Note>,
    has_current_file: bool,
    on_add_note: EventHandler<String>,
    on_delete_note: EventHandler<u64>,
    on_export_notes: EventHandler<()>,
    // Chapters
    chapters: Vec<Chapter>,
    // Remote server
    remote_running: bool,
    remote_port: u16,
    on_toggle_remote: EventHandler<()>,
    // Sleep timer
    sleep_remaining_secs: Option<u64>,
    on_set_sleep_timer: EventHandler<u64>,
    on_cancel_sleep_timer: EventHandler<()>,
    // Performance overlay
    show_metrics: bool,
    on_toggle_metrics: EventHandler<bool>,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let mut note_input = use_signal(String::new);
    let tabs = [
        (ToolsTab::Bookmarks, "tools_modal.tab_bookmarks"),
        (ToolsTab::Trim, "tools_modal.tab_trim"),
        (ToolsTab::MediaInfo, "tools_modal.tab_media_info"),
        (ToolsTab::History, "tools_modal.tab_history"),
        (ToolsTab::Notes, "tools_modal.tab_notes"),
        (ToolsTab::Chapters, "tools_modal.tab_chapters"),
        (ToolsTab::Settings, "tools_modal.tab_settings"),
    ];

    let mut start_time = use_signal(|| 0.0f64);
    let mut end_time = use_signal(|| duration);

    rsx! {
        div { class: "modal-overlay", onclick: move |_| on_close.call(()),
            div { class: "modal-card-large", onclick: move |e| e.stop_propagation(),
                div { class: "modal-header",
                    h3 { "{tr(language(), \"tools_modal.title\")}" }
                    button { class: "btn-icon", onclick: move |_| on_close.call(()), "✕" }
                }

                div { class: "modal-layout-tabbed",
                    nav { class: "modal-sidebar-tabs",
                        for (tab, label_key) in tabs {
                            {
                                let is_active = active_tab == tab;
                                let class_name = if is_active { "tab-button active" } else { "tab-button" };
                                rsx! {
                                    button {
                                        key: "{label_key}",
                                        class: "{class_name}",
                                        onclick: move |_| on_change_tab.call(tab),
                                        "{tr(language(), label_key)}"
                                    }
                                }
                            }
                        }
                    }

                    main { class: "modal-tab-content",
                        match active_tab {
                            ToolsTab::Bookmarks => rsx! {
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
                            },
                            ToolsTab::Trim => rsx! {
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
                            },
                            ToolsTab::MediaInfo => {
                                let info = media_info.clone().unwrap_or_default();
                                rsx! {
                                    div { class: "info-table",
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_file\")}" } span { "{filename}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_container\")}" } span { "{info.format}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_resolution\")}" } span { "{info.resolution()}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_fps\")}" } span { "{info.fps:.3}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_video_codec\")}" } span { "{info.video_codec}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_audio_codec\")}" } span { "{info.audio_codec}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_audio_channels\")}" } span { "{info.channel_str()}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_video_bitrate\")}" } span { "{info.video_bitrate_str()}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_audio_bitrate\")}" } span { "{info.audio_bitrate_str()}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_file_size\")}" } span { "{info.size_str()}" } }
                                        div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_total_duration\")}" } span { "{rplayer::player::PlayerState::format_time(info.duration)}" } }
                                    }
                                }
                            },
                            ToolsTab::History => rsx! {
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
                            },
                            ToolsTab::Notes => rsx! {
                                div { class: "control-group-col",
                                    div { style: "display: flex; gap: 8px; align-items: flex-start;",
                                        input {
                                            class: "select-input",
                                            style: "flex: 1;",
                                            r#type: "text",
                                            placeholder: "{tr(language(), \"tools_modal.notes_placeholder\")}",
                                            value: "{note_input}",
                                            disabled: !has_current_file,
                                            oninput: move |e| note_input.set(e.value()),
                                        }
                                        button {
                                            class: "btn-primary",
                                            disabled: !has_current_file || note_input.read().trim().is_empty(),
                                            onclick: move |_| {
                                                let text = note_input.read().trim().to_string();
                                                if !text.is_empty() {
                                                    on_add_note.call(text);
                                                    note_input.set(String::new());
                                                }
                                            },
                                            "{tr(language(), \"tools_modal.notes_add_button\")}"
                                        }
                                    }
                                    if notes.is_empty() {
                                        div { style: "padding: 20px; text-align: center; color: var(--text-muted); font-size: 13px;",
                                            "{tr(language(), \"tools_modal.notes_empty\")}"
                                        }
                                    }
                                    div { class: "tracks-list", style: "margin-top: 12px;",
                                        for note in notes.iter() {
                                            {
                                                let note_id = note.id;
                                                let pos = note.position;
                                                let text = note.text.clone();
                                                rsx! {
                                                    div { key: "{note.id}", class: "track-item",
                                                        div { style: "display: flex; gap: 8px; align-items: center;",
                                                            span { style: "font-family: monospace; color: var(--accent-color);", "{pos:.1}s" }
                                                            span { "{text}" }
                                                        }
                                                        button { class: "btn-icon", onclick: move |_| on_delete_note.call(note_id), "🗑️" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if !notes.is_empty() {
                                        div { style: "margin-top: 12px;",
                                            button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_export_notes.call(()), "{tr(language(), \"tools_modal.notes_export_button\")}" }
                                        }
                                    }
                                }
                            },
                            ToolsTab::Chapters => rsx! {
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
                            },
                            ToolsTab::Settings => {
                                let remote_description = tr(language(), "tools_modal.remote_description")
                                    .replacen("{}", &remote_port.to_string(), 1);
                                let remote_start_label = tr(language(), "tools_modal.remote_start_button")
                                    .replacen("{}", &remote_port.to_string(), 1);
                                let sleep_active_label = sleep_remaining_secs.map(|secs| {
                                    tr(language(), "tools_modal.sleep_timer_active").replacen(
                                        "{}",
                                        &rplayer::player::PlayerState::format_time(secs as f64),
                                        1,
                                    )
                                });
                                rsx! {
                                    div { style: "padding: 4px;",
                                        h4 { class: "section-title", "{tr(language(), \"tools_modal.remote_section_title\")}" }
                                        p { style: "font-size: 13px; color: var(--text-muted); margin-bottom: 12px;",
                                            "{remote_description}"
                                        }
                                        button {
                                            class: "btn-primary",
                                            onclick: move |_| on_toggle_remote.call(()),
                                            if remote_running {
                                                "{tr(language(), \"tools_modal.remote_stop_button\")}"
                                            } else {
                                                "{remote_start_label}"
                                            }
                                        }

                                        h4 { class: "section-title", style: "margin-top: 24px;", "{tr(language(), \"tools_modal.sleep_timer_section_title\")}" }
                                        if let Some(active_label) = sleep_active_label {
                                            p { style: "font-size: 13px; color: var(--text-muted); margin-bottom: 8px;",
                                                "{active_label}"
                                            }
                                            button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_cancel_sleep_timer.call(()), "{tr(language(), \"tools_modal.sleep_timer_cancel\")}" }
                                        } else {
                                            div { style: "display: flex; gap: 8px; margin-top: 8px;",
                                                button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_set_sleep_timer.call(15), "15 min" }
                                                button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_set_sleep_timer.call(30), "30 min" }
                                                button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_set_sleep_timer.call(45), "45 min" }
                                                button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_set_sleep_timer.call(60), "60 min" }
                                            }
                                        }

                                        h4 { class: "section-title", style: "margin-top: 24px;", "{tr(language(), \"menu.performance\")}" }
                                        label { style: "display: flex; align-items: center; gap: 8px; cursor: pointer;",
                                            input {
                                                r#type: "checkbox",
                                                checked: "{show_metrics}",
                                                onchange: move |e| on_toggle_metrics.call(e.value() == "true")
                                            }
                                            span { "{tr(language(), \"tools_modal.metrics_toggle\")}" }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
