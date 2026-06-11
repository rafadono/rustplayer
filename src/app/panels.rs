use super::{ActiveModal, PlayerApp, SidePanel};
use crate::chapters::current_chapter;
use crate::i18n::tr;
use crate::remote::RemoteServer;
use crate::ui::audio_tracks::AudioTrackPanel;
use crate::ui::bookmarks_panel::BookmarksPanel;
use crate::ui::chapters_panel::ChaptersPanel;
use crate::ui::codec_diagnostics_panel;
use crate::ui::context_menu::ContextAction;
use crate::ui::equalizer_panel::EqualizerPanel;
use crate::ui::history_panel::HistoryPanel;
use crate::ui::image_controls_panel::ImageControlsPanel;
use crate::ui::karaoke_panel::KaraokePanel;
use crate::ui::library_panel::LibraryAction;
use crate::ui::media_info_panel::MediaInfoPanel;
use crate::ui::notes_panel::NotesPanel;
use crate::ui::playlist_panel;
use crate::ui::sleep_timer_panel::SleepTimerPanel;
use crate::ui::subtitles::SubtitlePanel;
use crate::ui::sync_panel::SyncPanel;
use crate::ui::theme;
use crate::ui::theme_panel::ThemePanel;
use crate::ui::up_next_panel::{self, UpNextAction};
use egui::RichText;
use std::time::Instant;

fn build_bug_report_text(
    app: &PlayerApp,
    state: &crate::player::PlayerState,
    title: &str,
    description: &str,
    email: &str,
) -> String {
    let renderer = if app.renderer.is_some() {
        "enabled"
    } else {
        "disabled"
    };
    let current_media = state
        .current_file
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "-".to_string());
    let user_email = if email.trim().is_empty() {
        "-".to_string()
    } else {
        email.trim().to_string()
    };

    format!(
        "Title: {title}\n\nDescription:\n{description}\n\nReporter contact: {user_email}\n\n--- Runtime info ---\nApp version: {}\nOS: {} ({})\nLanguage: {:?}\nRenderer: {renderer}\nCurrent media: {current_media}\nPlayback position: {:.2}s\nPlayback speed: {:.2}x\nMuted: {}\nVolume: {}\n",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
        app.config.language,
        state.position,
        state.speed,
        state.muted,
        state.volume,
    )
}

pub(super) fn draw_side_panel(
    app: &mut PlayerApp,
    ctx: &egui::Context,
    state: &crate::player::PlayerState,
) {
    if app.side_panel == SidePanel::None {
        return;
    }
    let lang = app.config.language;
    let label = match app.side_panel {
        SidePanel::Playlist => tr(lang, "menu.playlist"),
        SidePanel::History => tr(lang, "menu.history"),
        SidePanel::Bookmarks => tr(lang, "menu.bookmarks"),
        SidePanel::Notes => tr(lang, "menu.notes"),
        SidePanel::Chapters => tr(lang, "menu.chapters"),
        SidePanel::Equalizer => tr(lang, "menu.equalizer"),
        SidePanel::AudioTracks => tr(lang, "menu.audio_tracks"),
        SidePanel::Subtitles => tr(lang, "menu.subtitle_tracks"),
        SidePanel::SubtitlesDownload => tr(lang, "panel.subtitle_download"),
        SidePanel::ImageControls => tr(lang, "menu.image_controls"),
        SidePanel::Sync => tr(lang, "menu.sync"),
        SidePanel::Karaoke => tr(lang, "menu.karaoke"),
        SidePanel::MediaInfo => tr(lang, "menu.media_info"),
        SidePanel::Theme => tr(lang, "menu.theme"),
        SidePanel::Settings => tr(lang, "menu.settings"),
        SidePanel::UpNext => tr(lang, "menu.up_next"),
        SidePanel::Performance => tr(lang, "menu.performance"),
        SidePanel::Library => tr(lang, "menu.library"),
        SidePanel::CodecDiagnostics => tr(lang, "menu.codec_diag"),
        SidePanel::None => "",
    };

    egui::SidePanel::right("side")
        .default_width(280.0)
        .width_range(200.0..=480.0)
        .show(ctx, |ui| {
            egui::Frame {
                fill: *theme::SURFACE,
                inner_margin: egui::Margin::symmetric(12.0, 10.0),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(label).color(theme::TEXT).size(13.0).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .small_button(RichText::new("x").color(theme::MUTED))
                            .clicked()
                        {
                            app.side_panel = SidePanel::None;
                        }
                    });
                });
            });
            ui.separator();

            egui::Frame {
                inner_margin: egui::Margin::symmetric(10.0, 8.0),
                ..Default::default()
            }
            .show(ui, |ui| match app.side_panel {
                SidePanel::Playlist => {
                    let (play_idx, ca) = playlist_panel::draw_playlist(ui, &app.playlist, lang);
                    if let Some(idx) = play_idx {
                        if let Some(t) = app.playlist.tracks.get(idx) {
                            let path = t.path.clone();
                            app.playlist.current = Some(idx);
                            app.open_path(path);
                        }
                    }
                    app.handle_context_action(ca, &state.clone(), ui.ctx());
                }
                SidePanel::History => {
                    let (open, ca) = HistoryPanel::show(ui, &mut app.history, state, lang);
                    if let Some(path) = open {
                        app.open_path(path);
                    }
                    app.handle_context_action(ca, &state.clone(), ui.ctx());
                }
                SidePanel::Bookmarks => {
                    let (seek, ca) = BookmarksPanel::show(ui, &mut app.bookmarks, state, lang);
                    if let Some(pos) = seek {
                        let _ = app.player.seek_absolute(pos);
                    }
                    app.handle_context_action(ca, &state.clone(), ui.ctx());
                }
                SidePanel::Notes => NotesPanel::show(ui, &mut app.notes, state),
                SidePanel::Chapters => {
                    let chapters = state.chapters.clone();
                    let cur = current_chapter(&chapters, state.position);
                    if let Some(pos) = ChaptersPanel::show(ui, &chapters, cur) {
                        let _ = app.player.seek_absolute(pos);
                    }
                }
                SidePanel::Equalizer => {
                    if EqualizerPanel::show(
                        ui,
                        &mut app.config.equalizer,
                        &app.player,
                        app.config.loudnorm,
                    ) {
                        app.config.save();
                    }
                }
                SidePanel::AudioTracks => {
                    AudioTrackPanel::show(ui, &state.audio_tracks, &app.player);
                }
                SidePanel::Subtitles => {
                    SubtitlePanel::show(ui, &state.sub_tracks, &app.player);
                }
                SidePanel::SubtitlesDownload => {
                    let file = state.current_file.as_deref();
                    if let Some(path) = app.opensubs_panel.show(ui, file) {
                        let _ = app.player.add_sub_file(&path);
                        app.success_msg = Some(("✓ Subtítulo cargado".into(), Instant::now()));
                    }
                }
                SidePanel::ImageControls => {
                    if ImageControlsPanel::show(
                        ui,
                        &mut app.config.image_controls,
                        &app.player,
                        app.config.language,
                    ) {
                        app.config.save();
                    }
                }
                SidePanel::Sync => {
                    let (ac, sc) = SyncPanel::show(
                        ui,
                        &mut app.config.audio_delay,
                        &mut app.config.sub_delay,
                        &state.sub_tracks,
                        &app.player,
                    );
                    if ac || sc {
                        app.config.save();
                    }
                }
                SidePanel::Karaoke => {
                    let file = state.current_file.as_deref();
                    if let Some(path) = KaraokePanel::show(ui, file) {
                        app.open_path(path);
                    }
                }
                SidePanel::MediaInfo => {
                    MediaInfoPanel::show(ui, &app.media_info, state);
                }
                SidePanel::Theme => {
                    let ctx_clone = ui.ctx().clone();
                    if ThemePanel::show(ui, &mut app.config.theme, &ctx_clone) {
                        app.config.save();
                    }
                }
                SidePanel::Settings => {
                    if app.settings_panel.show(ui, &mut app.config) {
                        app.config.save();
                        if app.config.remote_enabled && app.remote.is_none() {
                            app.remote = RemoteServer::start(app.config.remote_port);
                        }
                        let _ = app.player.set_aspect_ratio(&app.config.aspect_ratio);
                        app.apply_subtitle_style();
                    }
                }
                SidePanel::UpNext => {
                    let items = app.up_next.as_slice();
                    match up_next_panel::show(ui, &items) {
                        UpNextAction::PlayNow(idx) => {
                            if let Some(item) = app.up_next.dequeue(idx) {
                                app.open_path(item.path);
                            }
                        }
                        UpNextAction::Dequeue(idx) => {
                            let _ = app.up_next.dequeue(idx);
                        }
                        UpNextAction::Clear => app.up_next.clear_queue(),
                        UpNextAction::None => {}
                    }
                }
                SidePanel::Performance => {
                    if crate::ui::performance_panel::show(
                        ui,
                        state,
                        &mut app.config.show_metrics_overlay,
                    ) {
                        app.config.save();
                    }
                }
                SidePanel::Library => {
                    let entries = app.history.all_entries();
                    match app.library_panel.show(ui, &entries) {
                        LibraryAction::AddSelectedToPlaylist(paths) => {
                            app.playlist.add_many(paths);
                        }
                        LibraryAction::AddSelectedToQueue(paths) => {
                            for p in paths {
                                app.up_next.enqueue_last(crate::up_next::QueueItem {
                                    title: p
                                        .file_name()
                                        .map(|n| n.to_string_lossy().to_string())
                                        .unwrap_or_else(|| p.to_string_lossy().to_string()),
                                    path: p,
                                });
                            }
                        }
                        LibraryAction::RemoveSelectedFromIndex(paths) => {
                            app.history.remove_many(&paths);
                            app.history.save();
                        }
                        LibraryAction::RescanSelected(paths) => {
                            let missing: Vec<_> =
                                paths.into_iter().filter(|p| !p.exists()).collect();
                            app.history.remove_many(&missing);
                            app.history.save();
                        }
                        LibraryAction::None => {}
                    }
                }
                SidePanel::CodecDiagnostics => {
                    codec_diagnostics_panel::show(ui, &app.player);
                }
                SidePanel::None => {
                    let _ = ContextAction::None;
                }
            });
        });
}

pub(super) fn draw_floating_windows(
    app: &mut PlayerApp,
    ctx: &egui::Context,
    state: &crate::player::PlayerState,
) {
    let lang = app.config.language;

    if app.active_modal == Some(ActiveModal::About) {
        egui::Window::new("Acerca de RPlayer")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.label(
                    RichText::new("RPlayer v0.5.0-alpha")
                        .size(20.0)
                        .color(theme::ACCENT),
                );
                ui.add_space(4.0);
                ui.label("Reproductor libre. Sin publicidad invasiva.");
                ui.add_space(8.0);
                ui.label(
                    RichText::new("Rust · egui · libmpv · OpenGL · ffmpeg")
                        .color(theme::MUTED)
                        .size(12.0),
                );
                ui.add_space(8.0);
                ui.separator();
                if ui.button("Cerrar").clicked() {
                    app.active_modal = None;
                }
            });
    }

    if app.active_modal == Some(ActiveModal::BugReport) {
        egui::Window::new(tr(lang, "bug.title"))
            .collapsible(false)
            .resizable(true)
            .default_width(560.0)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new(tr(lang, "bug.intro"))
                        .color(theme::MUTED)
                        .size(11.0),
                );
                ui.add_space(6.0);

                ui.label(
                    RichText::new(tr(lang, "bug.summary"))
                        .color(theme::MUTED)
                        .size(11.0),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut app.bug_report_title_input)
                        .desired_width(520.0)
                        .hint_text(tr(lang, "bug.summary_hint")),
                );
                ui.add_space(6.0);

                ui.label(
                    RichText::new(tr(lang, "bug.steps"))
                        .color(theme::MUTED)
                        .size(11.0),
                );
                ui.add_sized(
                    [520.0, 150.0],
                    egui::TextEdit::multiline(&mut app.bug_report_desc_input)
                        .hint_text(tr(lang, "bug.steps_hint")),
                );
                ui.add_space(6.0);

                ui.label(
                    RichText::new(tr(lang, "bug.contact"))
                        .color(theme::MUTED)
                        .size(11.0),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut app.bug_report_email_input)
                        .desired_width(320.0)
                        .hint_text("email@example.com"),
                );

                let title = if app.bug_report_title_input.trim().is_empty() {
                    tr(lang, "bug.default_title").to_string()
                } else {
                    app.bug_report_title_input.trim().to_string()
                };
                let report = build_bug_report_text(
                    app,
                    state,
                    &title,
                    app.bug_report_desc_input.trim(),
                    app.bug_report_email_input.trim(),
                );

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button(tr(lang, "bug.copy")).clicked() {
                        ui.ctx().copy_text(report.clone());
                        app.success_msg =
                            Some((tr(lang, "bug.copied_to_clipboard").into(), Instant::now()));
                    }

                    let has_target = !app.config.bug_report_url.trim().is_empty();
                    if ui
                        .add_enabled(has_target, egui::Button::new(tr(lang, "bug.send")))
                        .clicked()
                    {
                        let target = super::build_bug_report_target(
                            app.config.bug_report_url.trim(),
                            &title,
                            &report,
                        );
                        if let Err(e) = open::that(target) {
                            app.error_msg = Some(format!("{}: {}", tr(lang, "bug.open_failed"), e));
                        }
                    }
                });
                if app.config.bug_report_url.trim().is_empty() {
                    ui.label(
                        RichText::new(tr(lang, "bug.configure_target"))
                            .color(theme::MUTED)
                            .size(10.0),
                    );
                }
                ui.add_space(6.0);
                if ui.button(tr(lang, "bug.close")).clicked() {
                    app.active_modal = None;
                }
            });
    }

    if let Some(err) = app.error_msg.clone() {
        egui::Window::new("Error")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(RichText::new(&err).color(theme::DANGER));
                ui.add_space(4.0);
                if ui.button("Cerrar").clicked() {
                    app.error_msg = None;
                }
            });
    }

    if let Some((msg, time)) = &app.success_msg {
        if time.elapsed().as_secs() < 3 {
            egui::Window::new("##toast")
                .title_bar(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -80.0])
                .show(ctx, |ui| {
                    ui.label(RichText::new(msg).color(theme::SUCCESS).size(13.0));
                });
        } else {
            app.success_msg = None;
        }
    }

    if app.active_modal == Some(ActiveModal::Streaming) {
        egui::Window::new("Abrir stream / URL")
            .collapsible(false)
            .resizable(false)
            .default_width(460.0)
            .show(ctx, |ui| {
                if let Some(url) = app.streaming_panel.show(ui) {
                    app.open_url(url);
                    app.active_modal = None;
                }
                ui.add_space(4.0);
                if ui.button("Cancelar").clicked() {
                    app.active_modal = None;
                }
            });
    }

    if app.active_modal == Some(ActiveModal::Trim) {
        egui::Window::new("Recortar video")
            .collapsible(false)
            .default_width(480.0)
            .show(ctx, |ui| {
                if let Some(job) = app.trim_panel.show(
                    ui,
                    state.current_file.as_deref(),
                    state.duration,
                    state.position,
                ) {
                    app.trim_job = Some(job);
                }
                if ui.button("Cerrar").clicked() {
                    app.active_modal = None;
                }
            });
    }

    if app.active_modal == Some(ActiveModal::Converter) {
        egui::Window::new("Convertir formato")
            .collapsible(false)
            .default_width(460.0)
            .show(ctx, |ui| {
                if let Some(job) = app.converter_panel.show(ui, state.current_file.as_deref()) {
                    app.convert_job = Some(job);
                }
                if ui.button("Cerrar").clicked() {
                    app.active_modal = None;
                }
            });
    }

    if app.active_modal == Some(ActiveModal::SleepTimer) {
        egui::Window::new("Sleep Timer")
            .collapsible(false)
            .resizable(false)
            .default_width(300.0)
            .show(ctx, |ui| {
                SleepTimerPanel::show(ui, &mut app.sleep_timer);
                ui.add_space(4.0);
                if ui.button("Cerrar").clicked() {
                    app.active_modal = None;
                }
            });
    }

    app.pip.show_if_open(ctx, &state.title);
}
