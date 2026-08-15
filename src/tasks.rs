use crate::components::TrackItem;
use crate::state::AppState;
use dioxus::prelude::*;
use futures_util::StreamExt;
use rplayer::converter::ConvertStatus;
use rplayer::i18n::tr;
use rplayer::lastfm::LastFmConfig;
use rplayer::lastfm::{ScrobbleTracker, TrackInfo};
use rplayer::opensubtitles::SubSearchStatus;
use rplayer::player::{MediaTrack, TrackKind};
use rplayer::remote::RemoteCommand;
use rplayer::sleep_timer::SleepAction;
use rplayer::trim::TrimStatus;
use std::path::PathBuf;

pub fn spawn_polling_coroutine(
    mut state: AppState,
    remote_nav_fn: impl FnMut(PathBuf) + 'static + Clone,
) -> Coroutine<()> {
    use_coroutine(move |mut rx: UnboundedReceiver<()>| {
        let mut remote_nav_fn = remote_nav_fn.clone();
        async move {
            let mut last_title = String::new();
            let mut last_paused_for_history = true;
            let mut thumbs_generated_for: Option<PathBuf> = None;
            let mut scrobble = ScrobbleTracker::new();

            while rx.next().await.is_some() {
                let Some(p_arc) = state.player_ref.read().clone() else {
                    continue;
                };
                let snap = {
                    let Ok(p) = p_arc.lock() else { continue };
                    let s = p.state.lock().unwrap().clone();
                    s
                };

                state.time_pos.set(snap.position);
                state.duration.set(snap.duration);
                state.paused.set(snap.paused);
                state.playing.set(!snap.paused && (state.has_file)());
                state.volume.set(snap.volume);
                state.muted.set(snap.muted);
                state.speed.set(snap.speed);

                let to_item = |t: &MediaTrack| TrackItem {
                    id: t.id,
                    title: t.title.clone(),
                    lang: t.lang.clone(),
                    track_type: match t.kind {
                        TrackKind::Audio => "audio".to_string(),
                        TrackKind::Sub => "sub".to_string(),
                        TrackKind::Video => "video".to_string(),
                    },
                };
                state
                    .audio_tracks
                    .set(snap.audio_tracks.iter().map(to_item).collect());
                state
                    .sub_tracks
                    .set(snap.sub_tracks.iter().map(to_item).collect());
                state
                    .current_audio
                    .set(snap.audio_tracks.iter().find(|t| t.selected).map(|t| t.id));
                state
                    .current_sub
                    .set(snap.sub_tracks.iter().find(|t| t.selected).map(|t| t.id));

                state.chapters.set(snap.chapters.clone());
                state.render_fps.set(snap.render_fps);
                state.dropped_frames.set(snap.dropped_frames);
                state.hwdec_active.set(snap.hwdec_active);
                state.buffer_seconds.set(snap.buffer_seconds);

                if let Some(path) = &snap.current_file {
                    state
                        .history
                        .write()
                        .update(path, &snap.title, snap.position, snap.duration);

                    if snap.duration > 0.0 && thumbs_generated_for.as_ref() != Some(path) {
                        thumbs_generated_for = Some(path.clone());
                        state.thumb_cache.write().generate(path, snap.duration);
                    }
                }

                if !snap.title.is_empty() && snap.title != last_title {
                    last_title = snap.title.clone();
                    if let Ok(p) = p_arc.lock() {
                        state.media_info_sig.set(p.media_info());
                    }
                    let sk = state.lastfm_config.read().session_key.clone();
                    if state.lastfm_config.read().enabled && !sk.is_empty() {
                        scrobble.start_track(TrackInfo::from_filename(&snap.title), &sk);
                    }
                    state.history.read().save();
                }
                if snap.paused != last_paused_for_history {
                    last_paused_for_history = snap.paused;
                    state.history.read().save();
                }

                if state.lastfm_config.read().enabled && !snap.paused {
                    let sk = state.lastfm_config.read().session_key.clone();
                    if !sk.is_empty() {
                        scrobble.tick(snap.position, snap.duration, &sk);
                    }
                }

                if let Some(job) = state.sub_search.read().as_ref() {
                    for status in job.rx.try_iter().collect::<Vec<_>>() {
                        match status {
                            SubSearchStatus::Searching => state
                                .sub_search_status
                                .set(tr((state.language)(), "opensub.searching").to_string()),
                            SubSearchStatus::Results(r) => {
                                state.sub_search_status.set(
                                    tr((state.language)(), "opensub.results_count").replacen(
                                        "{}",
                                        &r.len().to_string(),
                                        1,
                                    ),
                                );
                                state.sub_search_results.set(r);
                            }
                            SubSearchStatus::Downloading => state
                                .sub_search_status
                                .set(tr((state.language)(), "opensub.downloading").to_string()),
                            SubSearchStatus::Done(path) => {
                                state.sub_search_status.set(
                                    tr((state.language)(), "opensub.saved").replacen(
                                        "{}",
                                        &path.display().to_string(),
                                        1,
                                    ),
                                );
                                if let Some(p_arc2) = state.player_ref.read().clone() {
                                    if let Ok(p) = p_arc2.lock() {
                                        let _ = p.add_sub_file(&path);
                                    }
                                }
                            }
                            SubSearchStatus::Error(e) => state.sub_search_status.set(
                                tr((state.language)(), "common.error_prefix").replacen("{}", &e, 1),
                            ),
                        }
                    }
                }

                if let Some(job) = state.trim_job.read().as_ref() {
                    for status in job.status_rx.try_iter().collect::<Vec<_>>() {
                        match status {
                            TrimStatus::Done(path) => {
                                state
                                    .trim_status
                                    .set(tr((state.language)(), "trim.done").replacen(
                                        "{}",
                                        &path.display().to_string(),
                                        1,
                                    ))
                            }
                            TrimStatus::Error(e) => state.trim_status.set(
                                tr((state.language)(), "common.error_prefix").replacen("{}", &e, 1),
                            ),
                        }
                    }
                }

                if let Some(job) = state.convert_job.read().as_ref() {
                    for status in job.status_rx.try_iter().collect::<Vec<_>>() {
                        match status {
                            ConvertStatus::Done(path) => state.convert_status.set(
                                tr((state.language)(), "tools_modal.convert_done").replacen(
                                    "{}",
                                    &path.display().to_string(),
                                    1,
                                ),
                            ),
                            ConvertStatus::Error(e) => state.convert_status.set(
                                tr((state.language)(), "common.error_prefix").replacen("{}", &e, 1),
                            ),
                        }
                    }
                }

                let update_result = state
                    .update_rx
                    .read()
                    .as_ref()
                    .and_then(|rx| rx.try_recv().ok());
                if let Some(result) = update_result {
                    state.update_rx.set(None);
                    match result {
                        Ok(info) => {
                            state.update_status.set(if info.download_url.is_empty() {
                                tr((state.language)(), "tools_modal.update_up_to_date").replacen(
                                    "{}",
                                    &info.version,
                                    1,
                                )
                            } else {
                                tr((state.language)(), "tools_modal.update_available").replacen(
                                    "{}",
                                    &info.version,
                                    1,
                                )
                            });
                            state.update_info.set(Some(info));
                        }
                        Err(e) => {
                            state.update_status.set(
                                tr((state.language)(), "tools_modal.update_error")
                                    .replacen("{}", &e, 1),
                            );
                            state.update_info.set(None);
                        }
                    }
                }

                let install_result = state
                    .install_update_rx
                    .read()
                    .as_ref()
                    .and_then(|rx| rx.try_recv().ok());
                if let Some(result) = install_result {
                    state.install_update_rx.set(None);
                    match result {
                        Ok(()) => state.update_status.set(
                            tr((state.language)(), "tools_modal.update_installed").to_string(),
                        ),
                        Err(e) => state.update_status.set(
                            tr((state.language)(), "tools_modal.update_install_error")
                                .replacen("{}", &e, 1),
                        ),
                    }
                }

                let login_result = state
                    .lastfm_login_rx
                    .read()
                    .as_ref()
                    .and_then(|rx2| rx2.try_recv().ok());
                if let Some(result) = login_result {
                    state.lastfm_login_rx.set(None);
                    match result {
                        Ok(session_key) => {
                            let username = state.lastfm_pending_user.read().clone();
                            let new_cfg = LastFmConfig {
                                enabled: true,
                                username,
                                session_key,
                            };
                            state.lastfm_config.set(new_cfg.clone());
                            state
                                .lastfm_status
                                .set(tr((state.language)(), "lastfm.connected_status").to_string());
                            state.config.write().lastfm = new_cfg;
                            state.config.read().save();
                        }
                        Err(e) => state.lastfm_status.set(
                            tr((state.language)(), "common.error_prefix").replacen("{}", &e, 1),
                        ),
                    }
                }

                if let Some(server) = state.remote_server.read().clone() {
                    for cmd in server.drain() {
                        match cmd {
                            RemoteCommand::Next => {
                                let next_path =
                                    state.playlist.read().next().map(|t| t.path.clone());
                                if let Some(path) = next_path {
                                    remote_nav_fn(path);
                                }
                            }
                            RemoteCommand::Prev => {
                                let prev_path =
                                    state.playlist.read().prev().map(|t| t.path.clone());
                                if let Some(path) = prev_path {
                                    remote_nav_fn(path);
                                }
                            }
                            other => {
                                if let Some(p_arc2) = state.player_ref.read().clone() {
                                    if let Ok(p) = p_arc2.lock() {
                                        match other {
                                            RemoteCommand::TogglePause => {
                                                let _ = p.toggle_pause();
                                            }
                                            RemoteCommand::Pause => {
                                                let _ = p.set_paused(true);
                                            }
                                            RemoteCommand::Resume => {
                                                let _ = p.set_paused(false);
                                            }
                                            RemoteCommand::Stop => {
                                                let _ = p.stop();
                                            }
                                            RemoteCommand::Seek(t) => {
                                                let _ = p.seek_absolute(t);
                                            }
                                            RemoteCommand::SetVolume(v) => {
                                                let _ = p.set_volume(v);
                                            }
                                            RemoteCommand::Next | RemoteCommand::Prev => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if state.sleep_timer_sig.write().tick() {
                    let action = state.sleep_timer_sig.read().action.clone();
                    if let Some(p_arc2) = state.player_ref.read().clone() {
                        if let Ok(p) = p_arc2.lock() {
                            match action {
                                SleepAction::Pause => {
                                    let _ = p.set_paused(true);
                                }
                                SleepAction::Stop => {
                                    let _ = p.stop();
                                }
                                SleepAction::Quit => {}
                            }
                        }
                    }
                    if action == SleepAction::Quit {
                        std::process::exit(0);
                    }
                }
            }
        }
    })
}

pub struct AppLogic {
    pub do_load_file: EventHandler<PathBuf>,
}

pub fn use_app_logic(state: AppState) -> AppLogic {
    let window = dioxus::desktop::use_window();
    #[cfg(target_os = "linux")]
    let window_for_embed = window.clone();

    let mut current_title = state.current_title;
    let mut has_file = state.has_file;
    let mut playing = state.playing;
    let mut paused = state.paused;
    let player_ref = state.player_ref;
    let mut history = state.history;
    let mut playlist = state.playlist;

    let do_load_file = move |path: PathBuf| {
        let title = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        current_title.set(title);
        has_file.set(true);
        playing.set(true);
        paused.set(false);

        #[cfg(target_os = "linux")]
        {
            use dioxus::desktop::tao::platform::unix::WindowExtUnix;
            use gtk::prelude::*;

            let gtk_win = window_for_embed.gtk_window();
            if let Some(gdk_win) = gtk_win.window() {
                let wid = if let Ok(x11_win) =
                    glib::object::Cast::downcast::<gdkx11::X11Window>(gdk_win.clone())
                {
                    x11_win.xid() as i64
                } else {
                    gdk_win.as_ptr() as i64
                };

                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() {
                        if let Err(e) = p.set_wid(wid) {
                            error!("mpv: could not assign wid {wid}: {e:?}");
                        }
                    }
                }
            }
        }

        if let Some(ref p_arc) = *player_ref.read() {
            if let Ok(p) = p_arc.lock() {
                let _ = p.open(&path);

                let resume_at = history
                    .read()
                    .get(&path)
                    .filter(|e| e.should_resume())
                    .map(|e| e.last_position);
                if let Some(pos) = resume_at {
                    let _ = p.seek_absolute(pos);
                }
            }
        }
        {
            let title = current_title.read().clone();
            history.write().mark_play_start(&path, &title, 0.0);
            history.read().save();
        }

        playlist.write().add(path.clone());
        playlist.write().set_current_by_path(&path);
    };

    let remote_nav_fn = do_load_file.clone();
    let poll_tx = spawn_polling_coroutine(state, remote_nav_fn);

    use_hook(|| {
        let tx = poll_tx.tx();
        let _ = std::thread::Builder::new()
            .name("ui-poll".into())
            .spawn(move || loop {
                std::thread::sleep(std::time::Duration::from_millis(200));
                if tx.unbounded_send(()).is_err() {
                    break;
                }
            });
    });

    AppLogic {
        do_load_file: EventHandler::new(do_load_file),
    }
}
