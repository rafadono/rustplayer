pub mod bookmarks;
pub mod chapters;
pub mod convert;
pub mod history;
pub mod media_info;
pub mod notes;
pub mod settings;
pub mod trim;

pub use bookmarks::BookmarksTab;
pub use chapters::ChaptersTab;
pub use convert::ConvertTab;
use dioxus::prelude::*;
pub use history::HistoryTab;
pub use media_info::MediaInfoTab;
pub use notes::NotesTab;
use rplayer::bookmarks::Bookmark;
use rplayer::chapters::Chapter;
use rplayer::converter::ConvertPreset;
use rplayer::history::HistoryEntry;
use rplayer::i18n::{tr, Language};
use rplayer::media_info::MediaInfo;
use rplayer::notes::Note;
use rplayer::updater::{UpdateChannel, UpdateInfo};
pub use settings::SettingsTab;
use std::path::PathBuf;
pub use trim::TrimTab;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolsTab {
    Bookmarks,
    Trim,
    Convert,
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
    // Convert
    on_start_convert: EventHandler<ConvertPreset>,
    convert_status: String,
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
    // Update checker
    update_channel: UpdateChannel,
    on_change_update_channel: EventHandler<UpdateChannel>,
    auto_check_updates: bool,
    on_toggle_auto_check_updates: EventHandler<bool>,
    update_manifest_url_stable: String,
    update_manifest_url_beta: String,
    on_change_manifest_url: EventHandler<(UpdateChannel, String)>,
    update_status: String,
    update_info: Option<UpdateInfo>,
    on_check_updates: EventHandler<()>,
    on_install_update: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let tabs = [
        (ToolsTab::Bookmarks, "tools_modal.tab_bookmarks"),
        (ToolsTab::Trim, "tools_modal.tab_trim"),
        (ToolsTab::Convert, "tools_modal.tab_convert"),
        (ToolsTab::MediaInfo, "tools_modal.tab_media_info"),
        (ToolsTab::History, "tools_modal.tab_history"),
        (ToolsTab::Notes, "tools_modal.tab_notes"),
        (ToolsTab::Chapters, "tools_modal.tab_chapters"),
        (ToolsTab::Settings, "tools_modal.tab_settings"),
    ];

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
                                BookmarksTab {
                                    bookmarks: bookmarks.clone(),
                                    current_time: current_time,
                                    on_add_bookmark: move |v| on_add_bookmark.call(v),
                                    on_jump_bookmark: move |v| on_jump_bookmark.call(v),
                                    on_delete_bookmark: move |v| on_delete_bookmark.call(v),
                                }
                            },
                            ToolsTab::Trim => rsx! {
                                TrimTab {
                                    duration: duration,
                                    on_start_trim: move |v| on_start_trim.call(v),
                                    trim_status: trim_status.clone(),
                                }
                            },
                            ToolsTab::Convert => rsx! {
                                ConvertTab {
                                    on_start_convert: move |v| on_start_convert.call(v),
                                    convert_status: convert_status.clone(),
                                    has_current_file: has_current_file,
                                }
                            },
                            ToolsTab::MediaInfo => rsx! {
                                MediaInfoTab {
                                    filename: filename.clone(),
                                    media_info: media_info.clone(),
                                }
                            },
                            ToolsTab::History => rsx! {
                                HistoryTab {
                                    history_entries: history_entries.clone(),
                                    on_resume_history: move |v| on_resume_history.call(v),
                                    on_remove_history: move |v| on_remove_history.call(v),
                                    on_clear_history: move |_| on_clear_history.call(()),
                                }
                            },
                            ToolsTab::Notes => rsx! {
                                NotesTab {
                                    notes: notes.clone(),
                                    has_current_file: has_current_file,
                                    on_add_note: move |v| on_add_note.call(v),
                                    on_delete_note: move |v| on_delete_note.call(v),
                                    on_export_notes: move |_| on_export_notes.call(()),
                                }
                            },
                            ToolsTab::Chapters => rsx! {
                                ChaptersTab {
                                    chapters: chapters.clone(),
                                    on_jump_bookmark: move |v| on_jump_bookmark.call(v),
                                }
                            },
                            ToolsTab::Settings => rsx! {
                                SettingsTab {
                                    remote_running: remote_running,
                                    remote_port: remote_port,
                                    on_toggle_remote: move |_| on_toggle_remote.call(()),
                                    sleep_remaining_secs: sleep_remaining_secs,
                                    on_set_sleep_timer: move |v| on_set_sleep_timer.call(v),
                                    on_cancel_sleep_timer: move |_| on_cancel_sleep_timer.call(()),
                                    show_metrics: show_metrics,
                                    on_toggle_metrics: move |v| on_toggle_metrics.call(v),
                                    update_channel: update_channel,
                                    on_change_update_channel: move |v| on_change_update_channel.call(v),
                                    auto_check_updates: auto_check_updates,
                                    on_toggle_auto_check_updates: move |v| on_toggle_auto_check_updates.call(v),
                                    update_manifest_url_stable: update_manifest_url_stable.clone(),
                                    update_manifest_url_beta: update_manifest_url_beta.clone(),
                                    on_change_manifest_url: move |v| on_change_manifest_url.call(v),
                                    update_status: update_status.clone(),
                                    update_info: update_info.clone(),
                                    on_check_updates: move |_| on_check_updates.call(()),
                                    on_install_update: move |_| on_install_update.call(()),
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
