use dioxus::prelude::*;
use rplayer::bookmarks::Bookmark;
use rplayer::converter::{ConvertJob, ConvertPreset};
use rplayer::i18n::tr;
use rplayer::notes::Note;
use rplayer::remote::RemoteServer;
use rplayer::trim::TrimJob;
use rplayer::updater::{self, UpdateChannel};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use crate::components::modals::ToolsModal;

#[derive(Props, Clone, PartialEq)]
pub struct AppToolsModalProps {
    resume_history_fn: EventHandler<PathBuf>,
}

#[component]
pub fn AppToolsModal(props: AppToolsModalProps) -> Element {
    let state = use_context::<crate::state::AppState>();
    let mut show_tools_modal = state.show_tools_modal;
    let mut tools_tab = state.tools_tab;
    let player_ref = state.player_ref;
    let mut bookmarks = state.bookmarks;
    let mut convert_job = state.convert_job;
    let mut convert_status = state.convert_status;
    let mut trim_job = state.trim_job;
    let mut trim_status = state.trim_status;
    let mut update_status = state.update_status;
    let update_info = state.update_info;
    let mut remote_server = state.remote_server;
    let mut config = state.config;
    let mut history_sig = state.history;
    let mut notes = state.notes;
    let chapters = state.chapters;
    let mut show_metrics = state.show_metrics;
    let mut sleep_timer_sig = state.sleep_timer_sig;
    let mut update_rx = state.update_rx;
    let mut install_update_rx = state.install_update_rx;

    // Derived
    let language = state.language;
    let current_title = state.current_title;
    let media_info_sig = state.media_info_sig;
    let duration = state.duration;
    let mut time_pos = state.time_pos;
    let playlist = state.playlist;

    let current_file: Option<PathBuf> = playlist
        .read()
        .current
        .and_then(|i| playlist.read().tracks.get(i).cloned())
        .map(|t| t.path);
    let resume_history_fn = props.resume_history_fn;

    rsx! {
        ToolsModal {
            active_tab: tools_tab(),
            on_change_tab: move |tab| tools_tab.set(tab),
            on_close: move |_| show_tools_modal.set(false),

            bookmarks: bookmarks(),
            current_time: time_pos(),
            on_add_bookmark: move |t| {
                let label = tr(language(), "bookmarks.default_label")
                    .replacen("{}", &(bookmarks.read().len() + 1).to_string(), 1);
                let b = Bookmark::new(t, label);
                bookmarks.write().push(b);
            },
            on_jump_bookmark: move |t| {
                time_pos.set(t);
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.seek_absolute(t); }
                }
            },
            on_delete_bookmark: move |idx| { bookmarks.write().remove(idx); },

            duration: duration(),
            on_start_trim: {
                let path_clone = current_file.clone();
                move |(start, end): (f64, f64)| {
                    if let Some(path) = path_clone.clone() {
                        let output = rplayer::trim::default_output(&path, start, end);
                        let job = TrimJob::start(&path, start, end, output);
                        trim_status.set(if job.is_some() {
                            tr(language(), "trim.processing").to_string()
                        } else {
                            tr(language(), "trim.ffmpeg_not_found").to_string()
                        });
                        trim_job.set(job);
                    }
                }
            },
            trim_status: trim_status(),

            on_start_convert: {
                let path_clone = current_file.clone();
                move |preset: ConvertPreset| {
                    if let Some(path) = path_clone.clone() {
                        let output = ConvertJob::default_output(&path, &preset);
                        let job = ConvertJob::start(&path, output, &preset);
                        convert_status.set(if job.is_some() {
                            tr(language(), "tools_modal.convert_processing").to_string()
                        } else {
                            tr(language(), "tools_modal.convert_ffmpeg_not_found").to_string()
                        });
                        convert_job.set(job);
                    } else {
                        convert_status.set(tr(language(), "tools_modal.convert_no_file").to_string());
                    }
                }
            },
            convert_status: convert_status(),

            filename: current_title.read().clone(),
            media_info: media_info_sig.read().clone(),

            history_entries: history_sig.read().all_entries(),
            on_resume_history: move |path: PathBuf| resume_history_fn.call(path),
            on_remove_history: move |path: PathBuf| {
                history_sig.write().remove(&path);
                history_sig.read().save();
            },
            on_clear_history: move |_| {
                history_sig.write().clear();
                history_sig.read().save();
            },

            notes: current_file
                .as_ref()
                .map(|p| notes.read().get(p).to_vec())
                .unwrap_or_default(),
            has_current_file: current_file.is_some(),
            on_add_note: {
                let current_file = current_file.clone();
                move |text: String| {
                    if let Some(path) = current_file.clone() {
                        notes.write().add(&path, Note::new(time_pos(), text));
                        notes.read().save();
                    }
                }
            },
            on_delete_note: {
                let current_file = current_file.clone();
                move |id: u64| {
                    if let Some(path) = current_file.clone() {
                        notes.write().remove(&path, id);
                        notes.read().save();
                    }
                }
            },
            on_export_notes: {
                let current_file = current_file.clone();
                move |_| {
                    if let Some(p) = current_file.clone() {
                        dioxus::prelude::spawn(async move {
                            if let Some(out) = rfd::AsyncFileDialog::new().set_file_name("notes.txt").save_file().await {
                                let text = notes.read().export_text(&p);
                                let _ = std::fs::write(out.path(), text);
                            }
                        });
                    }
                }
            },

            chapters: chapters.read().clone(),

            show_metrics: show_metrics(),
            on_toggle_metrics: move |v: bool| {
                show_metrics.set(v);
                config.write().show_metrics_overlay = v;
                config.read().save();
            },

            remote_running: remote_server.read().is_some(),
            remote_port: config.read().remote_port,
            on_toggle_remote: move |_| {
                if remote_server.read().is_some() {
                    remote_server.set(None);
                } else {
                    let port = config.read().remote_port;
                    remote_server.set(RemoteServer::start(port).map(Arc::new));
                }
            },

            sleep_remaining_secs: sleep_timer_sig.read().remaining().map(|d| d.as_secs()),
            on_set_sleep_timer: move |mins: u64| {
                sleep_timer_sig.write().set_minutes(mins);
                sleep_timer_sig.write().start();
            },
            on_cancel_sleep_timer: move |_| {
                sleep_timer_sig.write().cancel();
            },

            update_channel: config.read().update_channel,
            on_change_update_channel: move |ch: UpdateChannel| {
                config.write().update_channel = ch;
                config.read().save();
            },
            auto_check_updates: config.read().auto_check_updates,
            on_toggle_auto_check_updates: move |v: bool| {
                config.write().auto_check_updates = v;
                config.read().save();
            },
            update_manifest_url_stable: config.read().update_manifest_url_stable.clone(),
            update_manifest_url_beta: config.read().update_manifest_url_beta.clone(),
            on_change_manifest_url: move |(ch, url): (UpdateChannel, String)| {
                match ch {
                    UpdateChannel::Stable => config.write().update_manifest_url_stable = url,
                    UpdateChannel::Beta => config.write().update_manifest_url_beta = url,
                }
                config.read().save();
            },
            update_status: update_status(),
            update_info: update_info(),
            on_check_updates: move |_| {
                let channel = config.read().update_channel;
                let manifest_url = match channel {
                    UpdateChannel::Stable => config.read().update_manifest_url_stable.clone(),
                    UpdateChannel::Beta => config.read().update_manifest_url_beta.clone(),
                };
                update_status.set(tr(language(), "tools_modal.update_checking").to_string());
                let (tx, rx) = crossbeam_channel::bounded(1);
                thread::spawn(move || {
                    let _ = tx.send(updater::check_for_updates(channel, &manifest_url));
                });
                update_rx.set(Some(rx));
            },
            on_install_update: move |_| {
                if let Some(info) = update_info() {
                    if info.download_url.is_empty() {
                        return;
                    }
                    update_status.set(tr(language(), "tools_modal.update_installing").to_string());
                    let download_url = info.download_url;
                    let (tx, rx) = crossbeam_channel::bounded(1);
                    thread::spawn(move || {
                        let result = std::env::current_exe()
                            .map_err(|e| e.to_string())
                            .and_then(|exe| updater::install_update_with_rollback(&download_url, &exe));
                        let _ = tx.send(result);
                    });
                    install_update_rx.set(Some(rx));
                }
            },
        }
    }
}
