#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;

use components::*;
use dioxus::desktop::{Config as DesktopConfig, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use futures_util::StreamExt;
#[cfg(target_os = "linux")]
use log::warn;
use rplayer::ab_repeat::AbRepeat;
use rplayer::bookmarks::Bookmark;
use rplayer::chapters::Chapter;
use rplayer::config::Config as AppConfig;
use rplayer::converter::{ConvertJob, ConvertPreset, ConvertStatus};
use rplayer::equalizer::Equalizer;
use rplayer::history::History;
use rplayer::i18n::{tr, Language};
use rplayer::image_controls::ImageControls;
use rplayer::lastfm::{self, LastFmConfig, ScrobbleTracker, TrackInfo};
use rplayer::media_info::MediaInfo;
use rplayer::notes::{Note, NoteStore};
use rplayer::opensubtitles::{SubResult, SubSearchJob, SubSearchStatus};
use rplayer::player::{MediaTrack, Player, TrackKind};
use rplayer::playlist::{Playlist, Track};
use rplayer::remote::{RemoteCommand, RemoteServer};
use rplayer::sleep_timer::{SleepAction, SleepTimer};
use rplayer::streaming;
use rplayer::theme_manager::{ThemeColors, ThemePreset};
use rplayer::thumbnail::ThumbnailCache;
use rplayer::trim::{self, TrimJob, TrimStatus};
use rplayer::updater::{self, UpdateChannel, UpdateInfo};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    std::env::set_var("LC_NUMERIC", "C");
    if std::env::args().any(|a| a == "--self-check") {
        std::process::exit(0);
    }

    // Fedora Workstation and other GNOME/KDE distros default to native Wayland.
    // mpv video embedding here only works via X11 (gdkx11::X11Window). Forcing
    // the X11 backend makes GTK use XWayland (present by default on Fedora,
    // Ubuntu and Debian), so the embedding path below always gets a valid `wid`
    // instead of silently falling back to an invalid pointer-derived one.
    #[cfg(target_os = "linux")]
    if std::env::var_os("GDK_BACKEND").is_none() {
        std::env::set_var("GDK_BACKEND", "x11");
    }

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cfg = DesktopConfig::new()
        .with_menu(None)
        .with_background_color((0, 0, 0, 0))
        .with_custom_head(r#"<link rel="stylesheet" href="assets/style.css">"#.to_string())
        .with_window(
            WindowBuilder::new()
                .with_title("RPlayer")
                .with_inner_size(LogicalSize::new(1140.0, 720.0))
                .with_min_inner_size(LogicalSize::new(600.0, 400.0)),
        );

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let initial_config = AppConfig::load();

    let mut is_dark_mode = use_signal(|| !initial_config.theme.is_light());
    let mut playing = use_signal(|| false);
    let mut paused = use_signal(|| true);
    let mut volume = use_signal(|| initial_config.volume);
    let mut muted = use_signal(|| initial_config.muted);
    let mut speed = use_signal(|| initial_config.speed);
    let mut time_pos = use_signal(|| 0.0f64);
    let mut duration = use_signal(|| 0.0f64);
    let mut current_title = use_signal(String::new);
    let mut has_file = use_signal(|| false);
    let mut ab_repeat = use_signal(AbRepeat::default);
    let mut is_fullscreen = use_signal(|| false);
    let mut language = use_signal(|| initial_config.language);
    use_context_provider(|| language);

    // Playlist
    let mut show_playlist = use_signal(|| false);
    let mut playlist = use_signal(Playlist::default);

    // History (resume playback) & Notes
    let mut history = use_signal(History::load);
    let mut notes = use_signal(NoteStore::load);

    // Chapters (populated from Player.state by the polling coroutine)
    let mut chapters = use_signal(Vec::<Chapter>::new);

    // Performance/metrics overlay (populated from Player.state)
    let mut show_metrics = use_signal(|| initial_config.show_metrics_overlay);
    let mut render_fps = use_signal(|| 0.0f64);
    let mut dropped_frames = use_signal(|| 0i64);
    let mut hwdec_active = use_signal(|| false);
    let mut buffer_seconds = use_signal(|| 0.0f64);

    // Seekbar thumbnail preview
    let mut thumb_cache = use_signal(ThumbnailCache::new);
    let mut hover_thumb = use_signal(|| None::<String>);

    // Modal Visibility & Active Tab States
    let mut show_audio_modal = use_signal(|| false);
    let mut audio_tab = use_signal(|| AudioTab::Equalizer);

    let mut show_video_modal = use_signal(|| false);
    let mut video_tab = use_signal(|| VideoTab::Subtitles);

    let mut show_tools_modal = use_signal(|| false);
    let mut tools_tab = use_signal(|| ToolsTab::Bookmarks);

    // Feature states
    let mut eq_enabled = use_signal(|| initial_config.equalizer.enabled);
    let mut eq_bands = use_signal(|| {
        let mut v: Vec<f64> = initial_config
            .equalizer
            .peq_filters
            .iter()
            .map(|f| f.gain_db as f64)
            .collect();
        v.resize(10, 0.0);
        v
    });
    let mut eq_preset = use_signal(|| "Flat".to_string());
    let mut brightness = use_signal(|| initial_config.image_controls.brightness);
    let mut contrast = use_signal(|| initial_config.image_controls.contrast);
    let mut saturation = use_signal(|| initial_config.image_controls.saturation);
    let mut hue = use_signal(|| initial_config.image_controls.hue);
    let mut gamma = use_signal(|| initial_config.image_controls.gamma);
    let mut audio_delay = use_signal(|| initial_config.audio_delay);
    let mut sub_delay = use_signal(|| initial_config.sub_delay);
    let mut karaoke_enabled = use_signal(|| initial_config.karaoke_enabled);
    let mut karaoke_pitch = use_signal(|| initial_config.karaoke_pitch);
    let mut bookmarks = use_signal(Vec::<Bookmark>::new);

    // Convert (tools/converter.rs)
    let mut convert_job = use_signal(|| None::<ConvertJob>);
    let mut convert_status = use_signal(String::new);

    // Open URL / streams (services/streaming.rs)
    let mut show_open_url_modal = use_signal(|| false);
    let mut open_url_error = use_signal(String::new);

    // Update checker (services/updater.rs)
    let mut update_status = use_signal(String::new);
    let mut update_info = use_signal(|| None::<UpdateInfo>);
    let mut update_rx =
        use_signal(|| None::<crossbeam_channel::Receiver<Result<UpdateInfo, String>>>);
    let mut install_update_rx =
        use_signal(|| None::<crossbeam_channel::Receiver<Result<(), String>>>);

    // Live player state (synced from Player.state by the polling coroutine below)
    let mut audio_tracks = use_signal(Vec::<TrackItem>::new);
    let mut sub_tracks = use_signal(Vec::<TrackItem>::new);
    let mut current_audio = use_signal(|| None::<i64>);
    let mut current_sub = use_signal(|| None::<i64>);
    let mut media_info_sig = use_signal(|| None::<MediaInfo>);

    // OpenSubtitles
    let mut sub_search = use_signal(|| None::<SubSearchJob>);
    let mut sub_search_status = use_signal(String::new);
    let mut sub_search_results = use_signal(Vec::<SubResult>::new);

    // Trim
    let mut trim_job = use_signal(|| None::<TrimJob>);
    let mut trim_status = use_signal(String::new);

    // Last.fm
    let mut lastfm_config = use_signal(|| initial_config.lastfm.clone());
    let mut lastfm_status = use_signal(String::new);
    let mut lastfm_pending_user = use_signal(String::new);
    let mut lastfm_login_rx =
        use_signal(|| None::<crossbeam_channel::Receiver<Result<String, String>>>);

    // Remote HTTP server
    let mut remote_server = use_signal(|| None::<Arc<RemoteServer>>);

    // Sleep timer
    let mut sleep_timer_sig = use_signal(SleepTimer::new);

    // App settings (persisted). Individual UI signals above stay the live source
    // of truth for rendering; `config` mirrors the fields that get saved so
    // fields with no UI (aspect ratio, repeat mode, etc.) round-trip untouched.
    let mut config = use_signal(|| initial_config.clone());

    // Single Player Instance
    let player_ref = use_signal(|| {
        if let Ok(p) = Player::new(
            initial_config.volume,
            initial_config.muted,
            initial_config.speed,
        ) {
            p.apply_image_controls(&initial_config.image_controls);
            let _ = p.set_audio_delay(initial_config.audio_delay);
            let _ = p.set_sub_delay(initial_config.sub_delay);
            if initial_config.equalizer.enabled
                || initial_config.karaoke_enabled
                || initial_config.karaoke_pitch.abs() > 0.01
            {
                p.set_audio_filters(
                    &initial_config.equalizer,
                    initial_config.loudnorm,
                    initial_config.karaoke_enabled,
                    initial_config.karaoke_pitch,
                );
            }
            Some(Arc::new(Mutex::new(p)))
        } else {
            None
        }
    });

    let window = dioxus::desktop::use_window();
    // `window` is `Rc<DesktopService>` (Clone, not Copy). Clone it for the
    // Linux embedding closure below so the original stays available for the
    // fullscreen handler further down.
    #[cfg(target_os = "linux")]
    let window_for_embed = window.clone();

    // File Open Helper
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
                            warn!("mpv: no se pudo asignar wid {wid}: {e:?}");
                        }
                    }
                }
            }
        }

        if let Some(ref p_arc) = *player_ref.read() {
            if let Ok(p) = p_arc.lock() {
                let _ = p.open(&path);
                let _ = p.set_paused(false);
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

    // Not `.clone()`-redundant despite what clippy reports on Windows: on
    // Linux this closure also captures `window` (used for X11 embedding
    // below), which is `Clone` but not `Copy`, so it can only be reused via
    // an explicit clone on every platform that actually compiles that path.
    #[allow(clippy::clone_on_copy)]
    let mut open_file_fn = do_load_file.clone();
    let open_file_dialog = move |_| {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(
                tr(language(), "dialog.media_filter"),
                &["mp4", "mkv", "avi", "webm", "mp3", "flac", "ogg", "wav"],
            )
            .pick_file()
        {
            open_file_fn(path);
        }
    };

    #[allow(clippy::clone_on_copy)]
    let mut playlist_load_fn = do_load_file.clone();
    #[allow(clippy::clone_on_copy)]
    let mut drop_file_fn = do_load_file.clone();
    #[allow(clippy::clone_on_copy)]
    let mut remote_nav_fn = do_load_file.clone();
    #[allow(clippy::clone_on_copy)]
    let mut open_url_fn = do_load_file.clone();
    #[allow(clippy::clone_on_copy)]
    let mut resume_history_fn = do_load_file.clone();

    // ── Background polling: syncs Player.state -> UI signals, and drains
    // background job channels (subtitle search, trim, Last.fm login, remote
    // control, sleep timer). Runs every ~200ms via a coroutine woken up by a
    // plain OS thread (Coroutine::tx() is Send, confirmed against the
    // installed dioxus-hooks 0.5.6 source).
    let poll_tx = use_coroutine(move |mut rx: UnboundedReceiver<()>| async move {
        let mut last_title = String::new();
        let mut last_paused_for_history = true;
        let mut thumbs_generated_for: Option<PathBuf> = None;
        let mut scrobble = ScrobbleTracker::new();

        while rx.next().await.is_some() {
            let Some(p_arc) = player_ref.read().clone() else {
                continue;
            };
            let snap = {
                let Ok(p) = p_arc.lock() else { continue };
                let s = p.state.lock().unwrap().clone();
                s
            };

            time_pos.set(snap.position);
            duration.set(snap.duration);
            paused.set(snap.paused);
            playing.set(!snap.paused && has_file());
            volume.set(snap.volume);
            muted.set(snap.muted);
            speed.set(snap.speed);

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
            audio_tracks.set(snap.audio_tracks.iter().map(to_item).collect());
            sub_tracks.set(snap.sub_tracks.iter().map(to_item).collect());
            current_audio.set(snap.audio_tracks.iter().find(|t| t.selected).map(|t| t.id));
            current_sub.set(snap.sub_tracks.iter().find(|t| t.selected).map(|t| t.id));

            chapters.set(snap.chapters.clone());
            render_fps.set(snap.render_fps);
            dropped_frames.set(snap.dropped_frames);
            hwdec_active.set(snap.hwdec_active);
            buffer_seconds.set(snap.buffer_seconds);

            if let Some(path) = &snap.current_file {
                history
                    .write()
                    .update(path, &snap.title, snap.position, snap.duration);

                if snap.duration > 0.0 && thumbs_generated_for.as_ref() != Some(path) {
                    thumbs_generated_for = Some(path.clone());
                    thumb_cache.write().generate(path, snap.duration);
                }
            }

            if !snap.title.is_empty() && snap.title != last_title {
                last_title = snap.title.clone();
                if let Ok(p) = p_arc.lock() {
                    media_info_sig.set(p.media_info());
                }
                let sk = lastfm_config.read().session_key.clone();
                if lastfm_config.read().enabled && !sk.is_empty() {
                    scrobble.start_track(TrackInfo::from_filename(&snap.title), &sk);
                }
                // Throttle history disk writes to file-change/pause-toggle
                // events instead of every ~200ms tick (Dioxus 0.5 has no
                // window-close hook to guarantee a final save otherwise).
                history.read().save();
            }
            if snap.paused != last_paused_for_history {
                last_paused_for_history = snap.paused;
                history.read().save();
            }

            if lastfm_config.read().enabled && !snap.paused {
                let sk = lastfm_config.read().session_key.clone();
                if !sk.is_empty() {
                    scrobble.tick(snap.position, snap.duration, &sk);
                }
            }

            if let Some(job) = sub_search.read().as_ref() {
                for status in job.rx.try_iter().collect::<Vec<_>>() {
                    match status {
                        SubSearchStatus::Searching => {
                            sub_search_status.set(tr(language(), "opensub.searching").to_string())
                        }
                        SubSearchStatus::Results(r) => {
                            sub_search_status.set(
                                tr(language(), "opensub.results_count").replacen(
                                    "{}",
                                    &r.len().to_string(),
                                    1,
                                ),
                            );
                            sub_search_results.set(r);
                        }
                        SubSearchStatus::Downloading => {
                            sub_search_status.set(tr(language(), "opensub.downloading").to_string())
                        }
                        SubSearchStatus::Done(path) => {
                            sub_search_status.set(tr(language(), "opensub.saved").replacen(
                                "{}",
                                &path.display().to_string(),
                                1,
                            ));
                            if let Some(p_arc2) = player_ref.read().clone() {
                                if let Ok(p) = p_arc2.lock() {
                                    let _ = p.add_sub_file(&path);
                                }
                            }
                        }
                        SubSearchStatus::Error(e) => sub_search_status
                            .set(tr(language(), "common.error_prefix").replacen("{}", &e, 1)),
                    }
                }
            }

            if let Some(job) = trim_job.read().as_ref() {
                for status in job.status_rx.try_iter().collect::<Vec<_>>() {
                    match status {
                        TrimStatus::Done(path) => {
                            trim_status.set(tr(language(), "trim.done").replacen(
                                "{}",
                                &path.display().to_string(),
                                1,
                            ))
                        }
                        TrimStatus::Error(e) => trim_status
                            .set(tr(language(), "common.error_prefix").replacen("{}", &e, 1)),
                    }
                }
            }

            if let Some(job) = convert_job.read().as_ref() {
                for status in job.status_rx.try_iter().collect::<Vec<_>>() {
                    match status {
                        ConvertStatus::Done(path) => {
                            convert_status.set(tr(language(), "tools_modal.convert_done").replacen(
                                "{}",
                                &path.display().to_string(),
                                1,
                            ))
                        }
                        ConvertStatus::Error(e) => convert_status
                            .set(tr(language(), "common.error_prefix").replacen("{}", &e, 1)),
                    }
                }
            }

            let update_result = update_rx.read().as_ref().and_then(|rx| rx.try_recv().ok());
            if let Some(result) = update_result {
                update_rx.set(None);
                match result {
                    Ok(info) => {
                        update_status.set(if info.download_url.is_empty() {
                            tr(language(), "tools_modal.update_up_to_date").replacen(
                                "{}",
                                &info.version,
                                1,
                            )
                        } else {
                            tr(language(), "tools_modal.update_available").replacen(
                                "{}",
                                &info.version,
                                1,
                            )
                        });
                        update_info.set(Some(info));
                    }
                    Err(e) => {
                        update_status
                            .set(tr(language(), "tools_modal.update_error").replacen("{}", &e, 1));
                        update_info.set(None);
                    }
                }
            }

            let install_result = install_update_rx
                .read()
                .as_ref()
                .and_then(|rx| rx.try_recv().ok());
            if let Some(result) = install_result {
                install_update_rx.set(None);
                match result {
                    Ok(()) => update_status
                        .set(tr(language(), "tools_modal.update_installed").to_string()),
                    Err(e) => update_status.set(
                        tr(language(), "tools_modal.update_install_error").replacen("{}", &e, 1),
                    ),
                }
            }

            let login_result = lastfm_login_rx
                .read()
                .as_ref()
                .and_then(|rx2| rx2.try_recv().ok());
            if let Some(result) = login_result {
                lastfm_login_rx.set(None);
                match result {
                    Ok(session_key) => {
                        let username = lastfm_pending_user.read().clone();
                        let new_cfg = LastFmConfig {
                            enabled: true,
                            username,
                            session_key,
                        };
                        lastfm_config.set(new_cfg.clone());
                        lastfm_status.set(tr(language(), "lastfm.connected_status").to_string());
                        config.write().lastfm = new_cfg;
                        config.read().save();
                    }
                    Err(e) => lastfm_status
                        .set(tr(language(), "common.error_prefix").replacen("{}", &e, 1)),
                }
            }

            if let Some(server) = remote_server.read().clone() {
                for cmd in server.drain() {
                    match cmd {
                        RemoteCommand::Next => {
                            let next_path = playlist.read().next().map(|t| t.path.clone());
                            if let Some(path) = next_path {
                                remote_nav_fn(path);
                            }
                        }
                        RemoteCommand::Prev => {
                            let prev_path = playlist.read().prev().map(|t| t.path.clone());
                            if let Some(path) = prev_path {
                                remote_nav_fn(path);
                            }
                        }
                        other => {
                            if let Some(p_arc2) = player_ref.read().clone() {
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

            if sleep_timer_sig.write().tick() {
                let action = sleep_timer_sig.read().action.clone();
                if let Some(p_arc2) = player_ref.read().clone() {
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
    });

    use_hook(|| {
        let tx = poll_tx.tx();
        let _ = thread::Builder::new()
            .name("ui-poll".into())
            .spawn(move || loop {
                thread::sleep(Duration::from_millis(200));
                if tx.unbounded_send(()).is_err() {
                    break;
                }
            });
    });

    let theme_attr = if is_dark_mode() { "dark" } else { "light" };
    let current_file: Option<PathBuf> = playlist
        .read()
        .current
        .and_then(|i| playlist.read().tracks.get(i).cloned())
        .map(|t| t.path);

    rsx! {
        div { id: "main", "data-theme": "{theme_attr}",
            HeaderBar {
                is_dark_mode: is_dark_mode(),
                on_toggle_theme: move |_| {
                    is_dark_mode.toggle();
                    let preset = if is_dark_mode() { ThemePreset::DarkBlue } else { ThemePreset::Light };
                    config.write().theme = ThemeColors::from_preset(&preset);
                    config.read().save();
                },
                on_open_file: open_file_dialog,
                on_open_url_modal: move |_| {
                    open_url_error.set(String::new());
                    show_open_url_modal.set(true);
                },
                on_open_audio_modal: move |_| show_audio_modal.set(true),
                on_open_video_modal: move |_| show_video_modal.set(true),
                on_open_tools_modal: move |_| show_tools_modal.set(true),
                on_toggle_playlist: move |_| show_playlist.toggle(),
                on_toggle_language: move |_| {
                    let next = if language() == Language::Es { Language::En } else { Language::Es };
                    language.set(next);
                    config.write().language = next;
                    config.read().save();
                },
            }

            div { class: "workspace",
                VideoStage {
                    has_file: has_file(),
                    current_title: current_title(),
                    on_drop_file: move |path: String| drop_file_fn(PathBuf::from(path)),
                    show_metrics: show_metrics(),
                    render_fps: render_fps(),
                    dropped_frames: dropped_frames(),
                    hwdec_active: hwdec_active(),
                    buffer_seconds: buffer_seconds(),
                    metrics_opacity: config.read().metrics_overlay_opacity,
                    metrics_font_size: config.read().metrics_overlay_font_size,
                }

                if show_playlist() {
                    PlaylistPanel {
                        tracks: playlist.read().tracks.clone(),
                        current_index: playlist.read().current,
                        on_select_item: move |idx| {
                            if let Some(path) = playlist.read().tracks.get(idx).map(|t: &Track| t.path.clone()) {
                                playlist_load_fn(path);
                            }
                        },
                        on_close: move |_| show_playlist.set(false),
                        on_import: move |_| {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter(tr(language(), "dialog.playlist_filter"), &["m3u", "m3u8", "pls"])
                                .pick_file()
                            {
                                playlist.write().load_m3u(&path);
                            }
                        },
                        on_export: move |_| {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter(tr(language(), "dialog.playlist_filter"), &["m3u"])
                                .set_file_name("playlist.m3u")
                                .save_file()
                            {
                                let _ = playlist.read().export_m3u(&path);
                            }
                        },
                    }
                }
            }

            PlayerControls {
                playing: playing(),
                paused: paused(),
                time_pos: time_pos(),
                duration: duration(),
                volume: volume(),
                muted: muted(),
                speed: speed(),
                ab_loop_active: ab_repeat.read().active,
                ab_loop_label: ab_repeat.read().label(),
                hover_thumb_uri: hover_thumb(),
                on_seek_preview: move |t: f64| {
                    hover_thumb.set(thumb_cache.read().nearest_data_uri(t));
                },
                on_seek_preview_end: move |_| hover_thumb.set(None),
                on_toggle_play: move |_| {
                    if has_file() {
                        paused.toggle();
                        playing.set(!paused());
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() {
                                let _ = p.set_paused(paused());
                            }
                        }
                    }
                },
                on_seek: move |pos| {
                    time_pos.set(pos);
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            let _ = p.seek_absolute(pos);
                        }
                    }
                },
                on_volume_change: move |v| {
                    volume.set(v);
                    config.write().volume = v;
                    config.read().save();
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            let _ = p.set_volume(v);
                        }
                    }
                },
                on_toggle_mute: move |_| {
                    muted.toggle();
                    config.write().muted = muted();
                    config.read().save();
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            let _ = p.toggle_mute();
                        }
                    }
                },
                on_change_speed: move |spd| {
                    speed.set(spd);
                    config.write().speed = spd;
                    config.read().save();
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            let _ = p.set_speed(spd);
                        }
                    }
                },
                on_toggle_ab_repeat: move |_| {
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            ab_repeat.write().cycle(time_pos(), &p.mpv_handle());
                        }
                    }
                },
                on_open_karaoke: move |_| {
                    audio_tab.set(AudioTab::Karaoke);
                    show_audio_modal.set(true);
                },
                on_open_trim: move |_| {
                    tools_tab.set(ToolsTab::Trim);
                    show_tools_modal.set(true);
                },
                on_open_opensubtitles: move |_| {
                    video_tab.set(VideoTab::Subtitles);
                    show_video_modal.set(true);
                },
                on_toggle_fullscreen: move |_| {
                    is_fullscreen.toggle();
                    window.set_fullscreen(is_fullscreen());
                },
            }

            /* 1. Modal de Audio */
            if show_audio_modal() {
                AudioModal {
                    active_tab: audio_tab(),
                    on_change_tab: move |tab| audio_tab.set(tab),
                    on_close: move |_| show_audio_modal.set(false),

                    eq_bands: eq_bands(),
                    eq_enabled: eq_enabled(),
                    eq_preset: eq_preset(),
                    on_band_change: move |(idx, val): (usize, f64)| {
                        if idx < eq_bands.read().len() {
                            eq_bands.write()[idx] = val;
                        }
                        {
                            let mut cfg = config.write();
                            if let Some(f) = cfg.equalizer.peq_filters.get_mut(idx) {
                                f.gain_db = val as f32;
                                f.enabled = true;
                            }
                            cfg.equalizer.enabled = true;
                        }
                        eq_enabled.set(true);
                        let eq_snapshot = config.read().equalizer.clone();
                        let loudnorm = config.read().loudnorm;
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq_snapshot, loudnorm, karaoke_enabled(), karaoke_pitch()); }
                        }
                        config.read().save();
                    },
                    on_toggle_eq: move |en: bool| {
                        eq_enabled.set(en);
                        config.write().equalizer.enabled = en;
                        let eq_snapshot = config.read().equalizer.clone();
                        let loudnorm = config.read().loudnorm;
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq_snapshot, loudnorm, karaoke_enabled(), karaoke_pitch()); }
                        }
                        config.read().save();
                    },
                    on_select_preset: move |p: String| {
                        eq_preset.set(p.clone());
                        // Only Flat/Bass Boost/Vocal/Rock have a dedicated backend preset;
                        // the remaining UI options fall back to Flat until more are added.
                        let eq = match p.as_str() {
                            "Bass Boost" => Equalizer::preset_bass_boost(),
                            "Vocal" => Equalizer::preset_vocal(),
                            "Rock" => Equalizer::preset_rock(),
                            _ => Equalizer::preset_flat(),
                        };
                        let mut bands: Vec<f64> = eq.peq_filters.iter().map(|f| f.gain_db as f64).collect();
                        bands.resize(10, 0.0);
                        eq_bands.set(bands);
                        eq_enabled.set(eq.enabled);
                        let loudnorm = config.read().loudnorm;
                        config.write().equalizer = eq.clone();
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq, loudnorm, karaoke_enabled(), karaoke_pitch()); }
                        }
                        config.read().save();
                    },

                    audio_tracks: audio_tracks(),
                    current_audio: current_audio(),
                    audio_delay: audio_delay(),
                    on_select_audio: move |id| {
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { let _ = p.set_audio_track(id); }
                        }
                    },
                    on_change_audio_delay: move |d| {
                        audio_delay.set(d);
                        config.write().audio_delay = d;
                        config.read().save();
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { let _ = p.set_audio_delay(d); }
                        }
                    },

                    karaoke_enabled: karaoke_enabled(),
                    karaoke_pitch: karaoke_pitch(),
                    on_toggle_karaoke: move |en: bool| {
                        karaoke_enabled.set(en);
                        config.write().karaoke_enabled = en;
                        let eq_snapshot = config.read().equalizer.clone();
                        let loudnorm = config.read().loudnorm;
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq_snapshot, loudnorm, en, karaoke_pitch()); }
                        }
                        config.read().save();
                    },
                    on_change_pitch: move |pitch: f64| {
                        karaoke_pitch.set(pitch);
                        config.write().karaoke_pitch = pitch;
                        let eq_snapshot = config.read().equalizer.clone();
                        let loudnorm = config.read().loudnorm;
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq_snapshot, loudnorm, karaoke_enabled(), pitch); }
                        }
                        config.read().save();
                    },

                    lastfm_connected: lastfm_config.read().enabled,
                    lastfm_username: lastfm_config.read().username.clone(),
                    lastfm_status: lastfm_status(),
                    on_lastfm_connect: move |(user, pass): (String, String)| {
                        lastfm_status.set(tr(language(), "lastfm.connecting").to_string());
                        lastfm_pending_user.set(user.clone());
                        let (tx, rx) = crossbeam_channel::bounded(1);
                        thread::spawn(move || {
                            let _ = tx.send(lastfm::get_session(&user, &pass));
                        });
                        lastfm_login_rx.set(Some(rx));
                    },
                    on_lastfm_disconnect: move |_| {
                        lastfm_config.set(LastFmConfig::default());
                        config.write().lastfm = LastFmConfig::default();
                        config.read().save();
                        lastfm_status.set(String::new());
                    },
                }
            }

            /* 2. Modal de Video y Subtítulos */
            if show_video_modal() {
                VideoModal {
                    active_tab: video_tab(),
                    on_change_tab: move |tab| video_tab.set(tab),
                    on_close: move |_| show_video_modal.set(false),

                    sub_tracks: sub_tracks(),
                    current_sub: current_sub(),
                    sub_delay: sub_delay(),
                    on_select_sub: move |id| {
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { let _ = p.set_sub_track(id); }
                        }
                    },
                    on_load_external_sub: move |_| {
                        if let Some(path) = rfd::FileDialog::new().add_filter(tr(language(), "dialog.subtitles_filter"), &["srt", "vtt", "ass"]).pick_file() {
                            if let Some(ref p_arc) = *player_ref.read() {
                                if let Ok(p) = p_arc.lock() { let _ = p.add_sub_file(&path); }
                            }
                        }
                    },
                    on_change_sub_delay: move |d| {
                        sub_delay.set(d);
                        config.write().sub_delay = d;
                        config.read().save();
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { let _ = p.set_sub_delay(d); }
                        }
                    },
                    opensubtitles_query: current_title(),
                    on_search_opensubtitles: move |_query: String| {
                        if let Some(path) = playlist.read().current.and_then(|i| playlist.read().tracks.get(i).cloned()).map(|t| t.path) {
                            sub_search_results.set(vec![]);
                            sub_search_status.set(tr(language(), "opensub.searching").to_string());
                            let job = SubSearchJob::search(&path, "es");
                            if job.is_none() {
                                sub_search_status.set(tr(language(), "opensub.not_configured").to_string());
                            }
                            sub_search.set(job);
                        }
                    },
                    sub_search_status: sub_search_status(),
                    sub_search_results: sub_search_results(),
                    on_download_sub: move |result: SubResult| {
                        if let Some(path) = playlist.read().current.and_then(|i| playlist.read().tracks.get(i).cloned()).map(|t| t.path) {
                            let dest_dir = path.parent().map(|p| p.to_path_buf()).unwrap_or_default();
                            sub_search_status.set(tr(language(), "opensub.downloading").to_string());
                            sub_search.set(SubSearchJob::download(&result, &dest_dir));
                        }
                    },

                    brightness: brightness(),
                    contrast: contrast(),
                    saturation: saturation(),
                    hue: hue(),
                    gamma: gamma(),
                    on_change_brightness: move |b| {
                        brightness.set(b);
                        config.write().image_controls.brightness = b;
                        config.read().save();
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_change_contrast: move |c| {
                        contrast.set(c);
                        config.write().image_controls.contrast = c;
                        config.read().save();
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_change_saturation: move |s| {
                        saturation.set(s);
                        config.write().image_controls.saturation = s;
                        config.read().save();
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_change_hue: move |h| {
                        hue.set(h);
                        config.write().image_controls.hue = h;
                        config.read().save();
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_change_gamma: move |g| {
                        gamma.set(g);
                        config.write().image_controls.gamma = g;
                        config.read().save();
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_reset_image: move |_| {
                        brightness.set(0); contrast.set(0); saturation.set(0); hue.set(0); gamma.set(0);
                        config.write().image_controls = ImageControls::default();
                        config.read().save();
                        let ic = ImageControls::default();
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                }
            }

            if show_open_url_modal() {
                OpenUrlModal {
                    on_close: move |_| show_open_url_modal.set(false),
                    error: open_url_error(),
                    on_open: move |url: String| {
                        if streaming::is_valid_url(&url) {
                            open_url_fn(PathBuf::from(url));
                            show_open_url_modal.set(false);
                        } else {
                            open_url_error.set(tr(language(), "open_url_modal.invalid_url").to_string());
                        }
                    },
                }
            }

            /* 3. Modal de Herramientas y Ajustes */
            if show_tools_modal() {
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
                    on_start_trim: move |(start, end): (f64, f64)| {
                        if let Some(path) = playlist.read().current.and_then(|i| playlist.read().tracks.get(i).cloned()).map(|t| t.path) {
                            let output = trim::default_output(&path, start, end);
                            let job = TrimJob::start(&path, start, end, output);
                            trim_status.set(if job.is_some() {
                                tr(language(), "trim.processing").to_string()
                            } else {
                                tr(language(), "trim.ffmpeg_not_found").to_string()
                            });
                            trim_job.set(job);
                        }
                    },
                    trim_status: trim_status(),

                    on_start_convert: move |preset: ConvertPreset| {
                        if let Some(path) = playlist.read().current.and_then(|i| playlist.read().tracks.get(i).cloned()).map(|t| t.path) {
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
                    },
                    convert_status: convert_status(),

                    filename: current_title(),
                    media_info: media_info_sig(),

                    history_entries: history.read().all_entries(),
                    on_resume_history: move |path: PathBuf| resume_history_fn(path),
                    on_remove_history: move |path: PathBuf| {
                        history.write().remove(&path);
                        history.read().save();
                    },
                    on_clear_history: move |_| {
                        history.write().clear();
                        history.read().save();
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
                    on_export_notes: move |_| {
                        if let Some(path) = current_file.clone() {
                            if let Some(out) = rfd::FileDialog::new().set_file_name("notes.txt").save_file() {
                                let _ = std::fs::write(out, notes.read().export_text(&path));
                            }
                        }
                    },

                    chapters: chapters(),

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
                            let download_url = info.download_url.clone();
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
    }
}
