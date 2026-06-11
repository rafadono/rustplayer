use super::{ActiveModal, PlayerApp, SidePanel};
use crate::bookmarks::Bookmark;
use crate::ui::context_menu::{ContextAction, Panel as CtxPanel};
use crate::ui::menu::MenuAction;
use crate::up_next::QueueItem;
use egui::Key;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone)]
pub(super) enum AppAction {
    TogglePause,
    Stop,
    SeekRelative(f64),
    SeekAbsolute(f64),
    PrevTrack,
    NextTrack,
    FrameStep,
    FrameBackStep,
    ToggleMute,
    VolumeDelta(i64),
    ToggleLoudnorm,
    SpeedUp,
    SpeedDown,
    SetSpeed(f64),
    CycleRepeat,
    ToggleShuffle,
    CycleAbRepeat,
    Screenshot,
    TogglePip,
    SetAspect(crate::config::AspectRatio),
    RotateCw,
    RotateCcw,
    FlipH,
    FlipV,
    ToggleDeinterlace,
    ToggleIntegerScaling,
    ResetImage,
    OpenTrimPanel,
    OpenConverterPanel,
    ToggleSleepTimer,
    OpenRemoteUrl,
    TogglePanel(SidePanel),
    ReportBug,
    About,
    Quit,
    OpenFile,
    OpenMultiple,
    OpenUrl,
    OpenKaraoke,
    ImportM3u,
    ExportM3u,
    PlayIndex(usize),
    RemoveFromPlaylist(usize),
    MoveUp(usize),
    MoveDown(usize),
    EnqueueNext(usize),
    EnqueueLast(usize),
    AddBookmarkAt(usize, f64),
    AddBookmarkCurrent,
    CopyPath(PathBuf),
    OpenInExplorer(PathBuf),
    ClearPlaylist,
    OpenFromHistory(PathBuf),
    RemoveFromHistory(PathBuf),
    ClearHistory,
    DeleteBookmark(String, u64),
    RenameBookmark(String, u64, String),
}

pub(super) fn handle_keyboard(app: &mut PlayerApp, ctx: &egui::Context) {
    let mut pending: Vec<AppAction> = Vec::new();
    let mut open_pending = false;

    ctx.input(|i| {
        if i.key_pressed(Key::Space) {
            pending.push(AppAction::TogglePause);
        }

        let shift = i.modifiers.shift;
        let ctrl = i.modifiers.ctrl;

        if i.key_pressed(Key::ArrowRight) {
            if ctrl {
                pending.push(AppAction::RotateCw);
            } else {
                pending.push(AppAction::SeekRelative(if shift { 60.0 } else { 5.0 }));
            }
        }
        if i.key_pressed(Key::ArrowLeft) {
            if ctrl {
                pending.push(AppAction::RotateCcw);
            } else {
                pending.push(AppAction::SeekRelative(if shift { -60.0 } else { -5.0 }));
            }
        }
        if i.key_pressed(Key::ArrowUp) {
            pending.push(AppAction::VolumeDelta(5));
        }
        if i.key_pressed(Key::ArrowDown) {
            pending.push(AppAction::VolumeDelta(-5));
        }

        if i.key_pressed(Key::M) {
            pending.push(AppAction::ToggleMute);
        }
        if i.key_pressed(Key::N) {
            pending.push(AppAction::NextTrack);
        }
        if i.key_pressed(Key::P) {
            pending.push(AppAction::PrevTrack);
        }
        if i.key_pressed(Key::Period) {
            pending.push(AppAction::FrameStep);
        }
        if i.key_pressed(Key::Comma) {
            pending.push(AppAction::FrameBackStep);
        }
        if i.key_pressed(Key::S) {
            pending.push(AppAction::Screenshot);
        }
        if i.key_pressed(Key::R) {
            pending.push(AppAction::CycleAbRepeat);
        }
        if i.key_pressed(Key::B) {
            pending.push(AppAction::AddBookmarkCurrent);
        }
        if ctrl && i.key_pressed(Key::O) {
            open_pending = true;
        }

        app.drop_hover = !i.raw.hovered_files.is_empty();
        if !i.raw.dropped_files.is_empty() {
            for f in &i.raw.dropped_files {
                if let Some(p) = &f.path {
                    app.playlist.add(p.clone());
                }
            }
            if let Some(f) = i.raw.dropped_files.first() {
                if let Some(p) = f.path.clone() {
                    app.pending_open = Some(p);
                }
            }
        }
    });

    for action in pending {
        dispatch_action(app, ctx, action);
    }
    if open_pending {
        dispatch_action(app, ctx, AppAction::OpenFile);
    }
    if let Some(p) = app.pending_open.take() {
        app.playlist.set_current_by_path(&p);
        app.open_path(p);
    }
}

pub(super) fn handle_menu_action(app: &mut PlayerApp, ctx: &egui::Context, action: MenuAction) {
    if let Some(a) = map_menu_action(action) {
        dispatch_action(app, ctx, a);
    }
}

pub(super) fn handle_context_action(
    app: &mut PlayerApp,
    action: ContextAction,
    state: &crate::player::PlayerState,
    ctx: &egui::Context,
) {
    if let Some(a) = map_context_action(action, state.position) {
        dispatch_action(app, ctx, a);
    }
}

fn map_menu_action(action: MenuAction) -> Option<AppAction> {
    use MenuAction::*;
    Some(match action {
        OpenFile => AppAction::OpenFile,
        OpenMultiple => AppAction::OpenMultiple,
        OpenUrl => AppAction::OpenUrl,
        OpenKaraoke => AppAction::OpenKaraoke,
        ImportM3u => AppAction::ImportM3u,
        ExportM3u => AppAction::ExportM3u,
        Quit => AppAction::Quit,
        TogglePause => AppAction::TogglePause,
        Stop => AppAction::Stop,
        SeekForward5 => AppAction::SeekRelative(5.0),
        SeekBackward5 => AppAction::SeekRelative(-5.0),
        SeekForward60 => AppAction::SeekRelative(60.0),
        SeekBackward60 => AppAction::SeekRelative(-60.0),
        FrameStep => AppAction::FrameStep,
        FrameBackStep => AppAction::FrameBackStep,
        PrevTrack => AppAction::PrevTrack,
        NextTrack => AppAction::NextTrack,
        ToggleMute => AppAction::ToggleMute,
        VolumeUp => AppAction::VolumeDelta(5),
        VolumeDown => AppAction::VolumeDelta(-5),
        ToggleLoudnorm => AppAction::ToggleLoudnorm,
        SpeedUp => AppAction::SpeedUp,
        SpeedDown => AppAction::SpeedDown,
        SpeedReset => AppAction::SetSpeed(1.0),
        SetSpeed(v) => AppAction::SetSpeed(v as f64 / 100.0),
        CycleRepeat => AppAction::CycleRepeat,
        ToggleShuffle => AppAction::ToggleShuffle,
        CycleAbRepeat => AppAction::CycleAbRepeat,
        Screenshot => AppAction::Screenshot,
        TogglePip => AppAction::TogglePip,
        SetAspect(r) => AppAction::SetAspect(r),
        RotateCw => AppAction::RotateCw,
        RotateCcw => AppAction::RotateCcw,
        FlipH => AppAction::FlipH,
        FlipV => AppAction::FlipV,
        ToggleDeinterlace => AppAction::ToggleDeinterlace,
        ToggleIntegerScaling => AppAction::ToggleIntegerScaling,
        ResetImage => AppAction::ResetImage,
        OpenTrimPanel => AppAction::OpenTrimPanel,
        OpenConverterPanel => AppAction::OpenConverterPanel,
        ToggleSleepTimer => AppAction::ToggleSleepTimer,
        OpenRemoteUrl => AppAction::OpenRemoteUrl,
        TogglePlaylist => AppAction::TogglePanel(SidePanel::Playlist),
        ToggleHistory => AppAction::TogglePanel(SidePanel::History),
        ToggleEqualizer => AppAction::TogglePanel(SidePanel::Equalizer),
        ToggleAudioTracks => AppAction::TogglePanel(SidePanel::AudioTracks),
        ToggleSync => AppAction::TogglePanel(SidePanel::Sync),
        ToggleBookmarks => AppAction::TogglePanel(SidePanel::Bookmarks),
        ToggleNotes => AppAction::TogglePanel(SidePanel::Notes),
        ToggleChapters => AppAction::TogglePanel(SidePanel::Chapters),
        ToggleSubtitles => AppAction::TogglePanel(SidePanel::Subtitles),
        ToggleSubtitlesDownload => AppAction::TogglePanel(SidePanel::SubtitlesDownload),
        ToggleImageControls => AppAction::TogglePanel(SidePanel::ImageControls),
        ToggleMediaInfo => AppAction::TogglePanel(SidePanel::MediaInfo),
        ToggleKaraoke => AppAction::TogglePanel(SidePanel::Karaoke),
        ToggleTheme => AppAction::TogglePanel(SidePanel::Theme),
        ToggleUpNext => AppAction::TogglePanel(SidePanel::UpNext),
        TogglePerformance => AppAction::TogglePanel(SidePanel::Performance),
        ToggleLibrary => AppAction::TogglePanel(SidePanel::Library),
        ToggleCodecDiagnostics => AppAction::TogglePanel(SidePanel::CodecDiagnostics),
        ToggleSettings => AppAction::TogglePanel(SidePanel::Settings),
        ReportBug => AppAction::ReportBug,
        About => AppAction::About,
        MenuAction::None => return Option::None,
    })
}

fn map_context_action(action: ContextAction, current_pos: f64) -> Option<AppAction> {
    use ContextAction::*;
    Some(match action {
        TogglePause => AppAction::TogglePause,
        Stop => AppAction::Stop,
        SeekForward5 => AppAction::SeekRelative(5.0),
        SeekForward60 => AppAction::SeekRelative(60.0),
        SeekBackward5 => AppAction::SeekRelative(-5.0),
        SeekBackward60 => AppAction::SeekRelative(-60.0),
        PrevTrack => AppAction::PrevTrack,
        NextTrack => AppAction::NextTrack,
        ToggleMute => AppAction::ToggleMute,
        VolumeUp => AppAction::VolumeDelta(5),
        VolumeDown => AppAction::VolumeDelta(-5),
        ToggleLoudnorm => AppAction::ToggleLoudnorm,
        Screenshot => AppAction::Screenshot,
        TogglePip => AppAction::TogglePip,
        SetAspect(r) => AppAction::SetAspect(r),
        RotateCw => AppAction::RotateCw,
        RotateCcw => AppAction::RotateCcw,
        FlipH => AppAction::FlipH,
        FlipV => AppAction::FlipV,
        ToggleDeinterlace => AppAction::ToggleDeinterlace,
        ToggleIntegerScaling => AppAction::ToggleIntegerScaling,
        ResetImage => AppAction::ResetImage,
        PlayIndex(i) => AppAction::PlayIndex(i),
        RemoveFromPlaylist(i) => AppAction::RemoveFromPlaylist(i),
        MoveUp(i) => AppAction::MoveUp(i),
        MoveDown(i) => AppAction::MoveDown(i),
        EnqueueNext(i) => AppAction::EnqueueNext(i),
        EnqueueLast(i) => AppAction::EnqueueLast(i),
        AddBookmarkAt(i) => AppAction::AddBookmarkAt(i, current_pos),
        OpenInExplorer(p) => AppAction::OpenInExplorer(p),
        CopyPath(p) => AppAction::CopyPath(p),
        ClearPlaylist => AppAction::ClearPlaylist,
        OpenFromHistory(p) => AppAction::OpenFromHistory(p),
        RemoveFromHistory(p) => AppAction::RemoveFromHistory(p),
        ClearHistory => AppAction::ClearHistory,
        SeekToBookmark(pos) => AppAction::SeekAbsolute(pos),
        DeleteBookmark(file, id) => AppAction::DeleteBookmark(file, id),
        RenameBookmark(file, id, name) => AppAction::RenameBookmark(file, id, name),
        TogglePanel(p) => AppAction::TogglePanel(map_ctx_panel(p)),
        OpenTrimPanel => AppAction::OpenTrimPanel,
        OpenConverterPanel => AppAction::OpenConverterPanel,
        ContextAction::None => return Option::None,
    })
}

fn map_ctx_panel(panel: CtxPanel) -> SidePanel {
    match panel {
        CtxPanel::Equalizer => SidePanel::Equalizer,
        CtxPanel::ImageControls => SidePanel::ImageControls,
        CtxPanel::Sync => SidePanel::Sync,
        CtxPanel::MediaInfo => SidePanel::MediaInfo,
        CtxPanel::Subtitles => SidePanel::Subtitles,
        CtxPanel::SubtitlesDownload => SidePanel::SubtitlesDownload,
        CtxPanel::AudioTracks => SidePanel::AudioTracks,
    }
}

fn dispatch_action(app: &mut PlayerApp, ctx: &egui::Context, action: AppAction) {
    match action {
        AppAction::TogglePause => {
            let _ = app.player.toggle_pause();
        }
        AppAction::Stop => {
            let _ = app.player.stop();
        }
        AppAction::SeekRelative(v) => {
            let _ = app.player.seek_relative(v);
        }
        AppAction::SeekAbsolute(v) => {
            let _ = app.player.seek_absolute(v);
        }
        AppAction::PrevTrack => app.play_prev(),
        AppAction::NextTrack => app.play_next(),
        AppAction::FrameStep => {
            let _ = app.player.frame_step();
        }
        AppAction::FrameBackStep => {
            let _ = app.player.frame_back_step();
        }
        AppAction::ToggleMute => {
            let _ = app.player.toggle_mute();
        }
        AppAction::VolumeDelta(delta) => change_volume(app, delta),
        AppAction::ToggleLoudnorm => {
            app.config.loudnorm = !app.config.loudnorm;
            app.player
                .set_audio_filters(&app.config.equalizer, app.config.loudnorm);
            app.config.save();
        }
        AppAction::SpeedUp => {
            let s = app.player.state.lock().unwrap().speed;
            let _ = app.player.set_speed((s + 0.25).min(4.0));
        }
        AppAction::SpeedDown => {
            let s = app.player.state.lock().unwrap().speed;
            let _ = app.player.set_speed((s - 0.25).max(0.25));
        }
        AppAction::SetSpeed(v) => {
            let _ = app.player.set_speed(v);
        }
        AppAction::CycleRepeat => {
            app.config.repeat_mode = app.config.repeat_mode.next();
            app.config.save();
        }
        AppAction::ToggleShuffle => {
            app.config.shuffle = !app.config.shuffle;
            app.config.save();
        }
        AppAction::CycleAbRepeat => {
            let pos = app.player.current_position();
            let mpv = app.player.mpv_handle();
            app.ab_repeat.cycle(pos, &mpv);
        }
        AppAction::Screenshot => do_screenshot(app),
        AppAction::TogglePip => app.pip.open = !app.pip.open,
        AppAction::SetAspect(r) => {
            let _ = app.player.set_aspect_ratio(&r);
            app.config.aspect_ratio = r;
            app.config.save();
        }
        AppAction::RotateCw => mutate_image_controls(app, |ic| ic.rotate_cw()),
        AppAction::RotateCcw => mutate_image_controls(app, |ic| ic.rotate_ccw()),
        AppAction::FlipH => mutate_image_controls(app, |ic| ic.flip_h = !ic.flip_h),
        AppAction::FlipV => mutate_image_controls(app, |ic| ic.flip_v = !ic.flip_v),
        AppAction::ToggleDeinterlace => {
            mutate_image_controls(app, |ic| ic.deinterlace = !ic.deinterlace)
        }
        AppAction::ToggleIntegerScaling => {
            mutate_image_controls(app, |ic| ic.integer_scaling = !ic.integer_scaling)
        }
        AppAction::ResetImage => mutate_image_controls(app, |ic| ic.reset()),
        AppAction::OpenTrimPanel => app.active_modal = Some(ActiveModal::Trim),
        AppAction::OpenConverterPanel => app.active_modal = Some(ActiveModal::Converter),
        AppAction::ToggleSleepTimer => {
            app.active_modal = if app.active_modal == Some(ActiveModal::SleepTimer) {
                None
            } else {
                Some(ActiveModal::SleepTimer)
            };
        }
        AppAction::OpenRemoteUrl => {
            let _ = open::that(format!("http://127.0.0.1:{}", app.config.remote_port));
        }
        AppAction::TogglePanel(sp) => app.toggle_side(sp),
        AppAction::ReportBug => app.active_modal = Some(ActiveModal::BugReport),
        AppAction::About => app.active_modal = Some(ActiveModal::About),
        AppAction::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
        AppAction::OpenFile => app.open_file_dialog_single(),
        AppAction::OpenMultiple => app.open_file_dialog_multiple(),
        AppAction::OpenUrl => app.active_modal = Some(ActiveModal::Streaming),
        AppAction::OpenKaraoke => app.open_karaoke_dialog(),
        AppAction::ImportM3u => app.import_m3u(),
        AppAction::ExportM3u => app.export_m3u(),
        AppAction::PlayIndex(idx) => {
            if let Some(t) = app.playlist.tracks.get(idx) {
                let path = t.path.clone();
                app.playlist.current = Some(idx);
                app.open_path(path);
            }
        }
        AppAction::RemoveFromPlaylist(idx) => app.playlist.remove(idx),
        AppAction::MoveUp(idx) => app.playlist.move_up(idx),
        AppAction::MoveDown(idx) => app.playlist.move_down(idx),
        AppAction::EnqueueNext(idx) => {
            if let Some(t) = app.playlist.tracks.get(idx) {
                app.up_next.enqueue_next(QueueItem::from_track(t));
                app.success_msg = Some(("⏭ Añadido a Up Next".into(), Instant::now()));
            }
        }
        AppAction::EnqueueLast(idx) => {
            if let Some(t) = app.playlist.tracks.get(idx) {
                app.up_next.enqueue_last(QueueItem::from_track(t));
                app.success_msg = Some(("➕ Añadido al final de Up Next".into(), Instant::now()));
            }
        }
        AppAction::AddBookmarkAt(idx, pos) => {
            if let Some(t) = app.playlist.tracks.get(idx) {
                let bm = Bookmark::new(pos, &t.title);
                app.bookmarks.add(&t.path, bm);
                app.bookmarks.save();
                app.success_msg = Some(("🔖 Marcador añadido".into(), Instant::now()));
            }
        }
        AppAction::AddBookmarkCurrent => {
            let s = app.player.state.lock().unwrap();
            if let Some(f) = s.current_file.clone() {
                let bm = Bookmark::new(s.position, "Marcador");
                drop(s);
                app.bookmarks.add(&f, bm);
                app.bookmarks.save();
                app.success_msg = Some(("🔖 Marcador añadido".into(), Instant::now()));
            }
        }
        AppAction::CopyPath(p) => {
            ctx.copy_text(p.to_string_lossy().to_string());
            app.success_msg = Some(("📋 Ruta copiada".into(), Instant::now()));
        }
        AppAction::OpenInExplorer(p) => {
            #[cfg(target_os = "linux")]
            {
                let _ = std::process::Command::new("xdg-open")
                    .arg(p.parent().unwrap_or(&p))
                    .spawn();
            }
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("explorer")
                    .arg(p.parent().unwrap_or(&p))
                    .spawn();
            }
            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("open").arg("-R").arg(&p).spawn();
            }
        }
        AppAction::ClearPlaylist => app.playlist.clear(),
        AppAction::OpenFromHistory(p) => app.open_path(p),
        AppAction::RemoveFromHistory(p) => {
            app.history.remove(&p);
            app.history.save();
        }
        AppAction::ClearHistory => {
            app.history.clear();
            app.history.save();
        }
        AppAction::DeleteBookmark(file_key, id) => {
            let path = PathBuf::from(&file_key);
            app.bookmarks.remove(&path, id);
            app.bookmarks.save();
        }
        AppAction::RenameBookmark(file_key, id, new_name) => {
            let path = PathBuf::from(&file_key);
            app.bookmarks.update_label(&path, id, new_name);
            app.bookmarks.save();
        }
    }
}

fn do_screenshot(app: &mut PlayerApp) {
    let _ = app.player.screenshot();
    app.success_msg = Some(("📷 Frame capturado".into(), Instant::now()));
}

fn change_volume(app: &mut PlayerApp, delta: i64) {
    let v = app.player.state.lock().unwrap().volume;
    let _ = app.player.set_volume((v + delta).clamp(0, 150));
}

fn mutate_image_controls(
    app: &mut PlayerApp,
    f: impl FnOnce(&mut crate::image_controls::ImageControls),
) {
    f(&mut app.config.image_controls);
    app.player.apply_image_controls(&app.config.image_controls);
    app.config.save();
}
