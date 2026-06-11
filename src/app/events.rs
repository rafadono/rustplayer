use super::PlayerApp;
use crate::converter::ConvertStatus;
use crate::karaoke;
use crate::lastfm::TrackInfo;
use crate::player::PlayerEvent;
use crate::trim::TrimStatus;
use std::time::Instant;

pub(super) fn process_player_events(app: &mut PlayerApp) {
    for event in app.player.drain_events() {
        match event {
            PlayerEvent::FileLoaded { title, duration } => {
                let file = app.player.state.lock().unwrap().current_file.clone();
                if let Some(ref f) = file {
                    if !f.to_string_lossy().starts_with("http") {
                        app.history.mark_play_start(f, &title, duration);
                        app.history.save();
                    }
                }

                if let Some(ref f) = file {
                    if !f.to_string_lossy().starts_with("http") && duration > 0.0 {
                        app.thumbnails.generate(f, duration);
                    }
                }

                app.media_info = app.player.media_info();

                if let Some(ref f) = file {
                    if let karaoke::KaraokeFileType::AudioWithCdg(_) = karaoke::classify(f) {}
                }

                if app.config.lastfm.enabled && !app.config.lastfm.session_key.is_empty() {
                    let track = TrackInfo::from_filename(&title);
                    app.scrobble
                        .start_track(track, &app.config.lastfm.session_key);
                }
            }
            PlayerEvent::EndOfFile => {
                app.save_history_entry();
                app.play_next_or_stop();
            }
            _ => {}
        }
    }
}

pub(super) fn process_remote_commands(app: &mut PlayerApp) {
    let cmds = app.remote.as_ref().map(|r| r.drain()).unwrap_or_default();
    for cmd in cmds {
        match cmd {
            crate::remote::RemoteCommand::TogglePause => {
                let _ = app.player.toggle_pause();
            }
            crate::remote::RemoteCommand::Pause => {
                let _ = app.player.set_paused(true);
            }
            crate::remote::RemoteCommand::Resume => {
                let _ = app.player.set_paused(false);
            }
            crate::remote::RemoteCommand::Stop => {
                let _ = app.player.stop();
            }
            crate::remote::RemoteCommand::Seek(t) => {
                let _ = app.player.seek_absolute(t);
            }
            crate::remote::RemoteCommand::SetVolume(v) => {
                let _ = app.player.set_volume(v);
            }
            crate::remote::RemoteCommand::Next => app.play_next(),
            crate::remote::RemoteCommand::Prev => app.play_prev(),
        }
    }
}

pub(super) fn tick_sleep_timer(app: &mut PlayerApp, ctx: &egui::Context) {
    if app.sleep_timer.tick() {
        match app.sleep_timer.action {
            crate::sleep_timer::SleepAction::Pause => {
                let _ = app.player.toggle_pause();
            }
            crate::sleep_timer::SleepAction::Stop => {
                let _ = app.player.stop();
            }
            crate::sleep_timer::SleepAction::Quit => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close)
            }
        }
        app.success_msg = Some(("⏲ Sleep timer disparado".into(), Instant::now()));
    }
}

pub(super) fn poll_jobs(app: &mut PlayerApp) {
    if let Some(job) = &app.trim_job {
        if let Ok(s) = job.status_rx.try_recv() {
            match s {
                TrimStatus::Done(p) => {
                    app.success_msg =
                        Some((format!("Recorte listo: {}", p.display()), Instant::now()));
                    app.trim_panel.clear_job();
                    app.trim_job = None;
                }
                TrimStatus::Error(e) => {
                    app.error_msg = Some(e);
                    app.trim_job = None;
                }
            }
        }
    }
    if let Some(job) = &app.convert_job {
        if let Ok(s) = job.status_rx.try_recv() {
            match s {
                ConvertStatus::Done(p) => {
                    app.success_msg =
                        Some((format!("Conversión lista: {}", p.display()), Instant::now()));
                    app.converter_panel.clear_job();
                    app.convert_job = None;
                }
                ConvertStatus::Error(e) => {
                    app.error_msg = Some(e);
                    app.convert_job = None;
                }
            }
        }
    }
}

pub(super) fn tick_scrobble(app: &mut PlayerApp, pos: f64, paused: bool, duration: f64) {
    if !paused && pos > 0.0 {
        let delta = (pos - app.last_tick_pos).abs();
        if delta < 5.0 {
            app.played_seconds += delta;
        }
    }
    app.last_tick_pos = pos;
    app.was_paused = paused;
    if app.config.lastfm.enabled && !app.config.lastfm.session_key.is_empty() {
        app.scrobble
            .tick(app.played_seconds, duration, &app.config.lastfm.session_key);
    }
}
