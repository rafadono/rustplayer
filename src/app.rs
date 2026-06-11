//! app.rs — RPlayer Orchestrator v0.3.

mod actions;
mod events;
mod panels;

use crate::ab_repeat::AbRepeat;
use crate::bookmarks::BookmarkStore;
use crate::config::{Config, RepeatMode};
use crate::converter::ConvertJob;
use crate::donation::DonationBanner;
use crate::history::History;
// image_controls used via config.image_controls
use crate::i18n::tr;
use crate::lastfm::ScrobbleTracker;
use crate::media_info::MediaInfo;
use crate::notes::NoteStore;
use crate::player::Player;
use crate::playlist::Playlist;
use crate::remote::RemoteServer;
use crate::renderer::MpvRenderer;
use crate::sleep_timer::SleepTimer;
use crate::streaming::StreamRequest;
use crate::thumbnail::ThumbnailCache;
use crate::trim::TrimJob;
use crate::ui::{
    context_menu::{video_context_menu, ContextAction},
    controls,
    converter_panel::ConverterPanel,
    library_panel::LibraryPanel,
    menu::{draw_menu, MenuAction, MenuState},
    opensubtitles_panel::OpenSubtitlesPanel,
    pip::PipWindow,
    settings_panel::SettingsPanel,
    streaming_panel::StreamingPanel,
    theme,
    trim_panel::TrimPanel,
};
use crate::up_next::UpNextQueue;
use egui::{Color32, RichText};
use log::{error, warn};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, PartialEq, Clone, Copy)]
enum SidePanel {
    None,
    Playlist,
    History,
    Bookmarks,
    Notes,
    Chapters,
    Equalizer,
    AudioTracks,
    Subtitles,
    SubtitlesDownload,
    ImageControls,
    Sync,
    Karaoke,
    MediaInfo,
    Theme,
    Settings,
    UpNext,
    Performance,
    Library,
    CodecDiagnostics,
}

#[derive(PartialEq, Clone, Copy)]
enum ActiveModal {
    About,
    BugReport,
    Streaming,
    Trim,
    Converter,
    SleepTimer,
}

pub struct PlayerApp {
    player: Player,
    renderer: Option<MpvRenderer>,
    playlist: Playlist,
    up_next: UpNextQueue,
    config: Config,
    donation: DonationBanner,

    history: History,
    bookmarks: BookmarkStore,
    notes: NoteStore,

    thumbnails: ThumbnailCache,
    scrobble: ScrobbleTracker,
    remote: Option<RemoteServer>,
    trim_job: Option<TrimJob>,
    convert_job: Option<ConvertJob>,

    // New modules v0.3
    ab_repeat: AbRepeat,
    sleep_timer: SleepTimer,
    media_info: Option<MediaInfo>,

    side_panel: SidePanel,
    active_modal: Option<ActiveModal>,
    drop_hover: bool,
    error_msg: Option<String>,
    success_msg: Option<(String, Instant)>,
    bug_report_title_input: String,
    bug_report_desc_input: String,
    bug_report_email_input: String,

    streaming_panel: StreamingPanel,
    trim_panel: TrimPanel,
    converter_panel: ConverterPanel,
    settings_panel: SettingsPanel,
    opensubs_panel: OpenSubtitlesPanel,
    pip: PipWindow,
    library_panel: LibraryPanel,

    last_tick_pos: f64,
    played_seconds: f64,
    was_paused: bool,
    pending_open: Option<PathBuf>,
    cover_texture: Option<egui::TextureHandle>,
    cover_checked_for: Option<PathBuf>,
}

impl PlayerApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let config = Config::load();

        // Apply saved theme
        config.theme.apply(&cc.egui_ctx);

        let player = Player::new(config.volume, config.muted, config.speed)
            .expect("No se pudo inicializar libmpv.");

        let _ = player.set_aspect_ratio(&config.aspect_ratio);
        player.set_audio_filters(&config.equalizer, config.loudnorm);
        let _ = player.set_sub_font_size(config.sub_font_size);
        let _ = player.set_sub_opacity(config.sub_opacity);
        let _ = player.set_sub_font_family(&config.sub_font_family);
        let _ = player.set_sub_bold(config.sub_bold);
        if let Some((r, g, b)) = parse_hex_rgb(&config.sub_color) {
            let _ = player.set_sub_color_rgb(r, g, b);
        }
        let _ = player.set_audio_delay(config.audio_delay);
        let _ = player.set_sub_delay(config.sub_delay);
        player.apply_image_controls(&config.image_controls);

        let renderer = cc.gl.as_ref().and_then(|gl| {
            MpvRenderer::new(gl, player.mpv_handle())
                .map_err(|e| error!("MpvRenderer: {}", e))
                .ok()
        });
        if renderer.is_none() {
            warn!("Render de video no disponible.");
        }

        let remote = if config.remote_enabled {
            RemoteServer::start(config.remote_port)
        } else {
            None
        };

        let settings_panel = SettingsPanel::new(&config);
        let donation = DonationBanner::new(config.show_donation_banner);
        let side = if config.show_playlist {
            SidePanel::Playlist
        } else {
            SidePanel::None
        };

        Self {
            player,
            renderer,
            playlist: Playlist::default(),
            donation,
            up_next: UpNextQueue::default(),
            history: History::load(),
            bookmarks: BookmarkStore::load(),
            notes: NoteStore::load(),
            thumbnails: ThumbnailCache::new(),
            scrobble: ScrobbleTracker::new(),
            remote,
            trim_job: None,
            convert_job: None,
            ab_repeat: AbRepeat::default(),
            sleep_timer: SleepTimer::new(),
            media_info: None,
            side_panel: side,
            active_modal: None,
            drop_hover: false,
            error_msg: None,
            success_msg: None,
            bug_report_title_input: String::new(),
            bug_report_desc_input: String::new(),
            bug_report_email_input: String::new(),
            streaming_panel: StreamingPanel::new(),
            trim_panel: TrimPanel::new(),
            converter_panel: ConverterPanel::new(),
            settings_panel,
            opensubs_panel: OpenSubtitlesPanel::new(),
            pip: PipWindow::new(),
            library_panel: LibraryPanel::default(),
            last_tick_pos: 0.0,
            played_seconds: 0.0,
            was_paused: false,
            pending_open: None,
            cover_texture: None,
            cover_checked_for: None,
            config,
        }
    }

    // ── Opening ───────────────────────────── ─────────────────────────────

    fn open_path(&mut self, path: PathBuf) {
        self.ab_repeat.clear(&self.player.mpv_handle());
        self.media_info = None;

        self.playlist.add(path.clone());
        self.playlist.set_current_by_path(&path);
        if let Err(e) = self.player.open(&path) {
            self.error_msg = Some(format!("Error: {}", e));
            return;
        }
        self.played_seconds = 0.0;
        self.last_tick_pos = 0.0;

        if let Some(entry) = self.history.get(&path) {
            if entry.should_resume() {
                let pos = entry.last_position;
                let _ = self.player.seek_absolute(pos);
            }
        }
    }

    fn ensure_cover_for_current(&mut self, current_file: Option<&Path>, ctx: &egui::Context) {
        let Some(file) = current_file else {
            self.cover_texture = None;
            self.cover_checked_for = None;
            return;
        };

        if file.to_string_lossy().starts_with("http") {
            self.cover_texture = None;
            self.cover_checked_for = Some(file.to_path_buf());
            return;
        }

        if self.cover_checked_for.as_deref() == Some(file) {
            return;
        }

        self.cover_checked_for = Some(file.to_path_buf());
        self.cover_texture =
            Self::find_cover_art_path(file).and_then(|cover| Self::load_cover_texture(ctx, &cover));
    }

    fn find_cover_art_path(media_path: &Path) -> Option<PathBuf> {
        const EXT: &[&str] = &["jpg", "jpeg", "png", "webp", "bmp"];
        const COMMON: &[&str] = &["cover", "folder", "front", "album", "artwork"];

        let dir = media_path.parent()?;
        let stem = media_path.file_stem()?.to_string_lossy().to_lowercase();

        for ext in EXT {
            let p = dir.join(format!("{stem}.{ext}"));
            if p.is_file() {
                return Some(p);
            }
        }

        for name in COMMON {
            for ext in EXT {
                let p = dir.join(format!("{name}.{ext}"));
                if p.is_file() {
                    return Some(p);
                }
            }
        }

        let entries = std::fs::read_dir(dir).ok()?;
        for entry in entries.flatten() {
            let p = entry.path();
            if !p.is_file() {
                continue;
            }
            let Some(ext) = p.extension().map(|e| e.to_string_lossy().to_lowercase()) else {
                continue;
            };
            if EXT.iter().any(|x| *x == ext) {
                return Some(p);
            }
        }

        None
    }

    fn load_cover_texture(ctx: &egui::Context, cover: &Path) -> Option<egui::TextureHandle> {
        let img = image::open(cover).ok()?;
        let img = img.thumbnail(900, 900).to_rgba8();
        let size = [img.width() as usize, img.height() as usize];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
        let tex_id = format!("cover:{}", cover.to_string_lossy());
        Some(ctx.load_texture(tex_id, color_image, egui::TextureOptions::LINEAR))
    }

    fn open_url(&mut self, req: StreamRequest) {
        let path = PathBuf::from(&req.url);
        self.playlist.add_url(req.url, req.title);
        self.playlist.set_current_by_path(&path);
        let _ = self.player.open(&path);
        self.played_seconds = 0.0;
    }

    fn apply_subtitle_style(&self) {
        let _ = self.player.set_sub_font_size(self.config.sub_font_size);
        let _ = self.player.set_sub_opacity(self.config.sub_opacity);
        let _ = self
            .player
            .set_sub_font_family(&self.config.sub_font_family);
        let _ = self.player.set_sub_bold(self.config.sub_bold);
        if let Some((r, g, b)) = parse_hex_rgb(&self.config.sub_color) {
            let _ = self.player.set_sub_color_rgb(r, g, b);
        }
    }

    fn open_file_dialog_single(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_directory(self.config.last_directory.clone().unwrap_or(".".into()))
            .add_filter("Video y Audio", MEDIA_EXTS)
            .add_filter("Todos", &["*"])
            .pick_file()
        {
            self.config.last_directory = path.parent().map(|p| p.to_path_buf());
            self.open_path(path);
        }
    }

    fn open_file_dialog_multiple(&mut self) {
        if let Some(paths) = rfd::FileDialog::new()
            .set_directory(self.config.last_directory.clone().unwrap_or(".".into()))
            .add_filter("Video y Audio", MEDIA_EXTS)
            .add_filter("Todos", &["*"])
            .pick_files()
        {
            let first = paths.first().cloned();
            if let Some(f) = &first {
                self.config.last_directory = f.parent().map(|p| p.to_path_buf());
            }
            self.playlist.add_many(paths);
            if let Some(p) = first {
                self.open_path(p);
            }
        }
    }

    fn open_karaoke_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Karaoke", &["cdg", "CDG", "mp3", "ogg", "wav"])
            .pick_file()
        {
            self.open_path(path);
        }
    }

    fn import_m3u(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Listas", &["m3u", "m3u8", "pls"])
            .pick_file()
        {
            let tracks = self.playlist.load_m3u(&path);
            if let Some(first) = tracks.first().cloned() {
                self.playlist.set_current_by_path(&first);
                let _ = self.player.open(&first);
            }
        }
    }

    fn export_m3u(&self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("M3U", &["m3u"])
            .set_file_name("playlist.m3u")
            .save_file()
        {
            let _ = self.playlist.export_m3u(&path);
        }
    }

    // ── Player events ──────────────────────── ────────────────────────

    fn process_player_events(&mut self) {
        events::process_player_events(self);
    }

    fn play_next_or_stop(&mut self) {
        if let Some(next) = self.up_next.pop_next() {
            self.open_path(next.path);
            return;
        }
        match self.config.repeat_mode {
            RepeatMode::One => {
                let file = self.player.state.lock().unwrap().current_file.clone();
                if let Some(f) = file {
                    let _ = self.player.open(&f);
                }
            }
            RepeatMode::All => {
                if let Some(next) = self.playlist.next() {
                    let path = next.path.clone();
                    self.playlist.current = self.playlist.current.map(|c| c + 1);
                    self.open_path(path);
                } else if let Some(first) = self.playlist.tracks.first().map(|t| t.path.clone()) {
                    self.playlist.current = Some(0);
                    self.open_path(first);
                }
            }
            RepeatMode::None => {
                if self.config.shuffle {
                    self.play_shuffle();
                } else if let Some(next) = self.playlist.next() {
                    let path = next.path.clone();
                    self.playlist.current = self.playlist.current.map(|c| c + 1);
                    self.open_path(path);
                }
            }
        }
    }

    fn play_shuffle(&mut self) {
        let len = self.playlist.tracks.len();
        if len <= 1 {
            return;
        }
        let current = self.playlist.current.unwrap_or(0);
        // Simple: pick random != current
        let idx = (current + 1 + (rand_usize() % (len - 1))) % len;
        let path = self.playlist.tracks[idx].path.clone();
        self.playlist.current = Some(idx);
        self.open_path(path);
    }

    fn save_history_entry(&mut self) {
        let s = self.player.state.lock().unwrap().clone();
        if let Some(f) = &s.current_file {
            if !f.to_string_lossy().starts_with("http") {
                self.history.update(f, &s.title, s.position, s.duration);
                self.history.save();
            }
        }
    }

    // ── Navigation ──────────────────────────── ────────────────────────────

    fn play_next(&mut self) {
        if let Some(next) = self.up_next.pop_next() {
            self.open_path(next.path);
            return;
        }
        if self.config.shuffle {
            return self.play_shuffle();
        }
        if let Some(next) = self.playlist.next() {
            let path = next.path.clone();
            self.playlist.current = self.playlist.current.map(|c| c + 1);
            self.open_path(path);
        }
    }

    fn play_prev(&mut self) {
        if let Some(prev) = self.playlist.prev() {
            let path = prev.path.clone();
            self.playlist.current = self.playlist.current.and_then(|c| c.checked_sub(1));
            self.open_path(path);
        }
    }

    // ── Remote control ────────────────────────── ──────────────────────────

    fn process_remote_commands(&mut self) {
        events::process_remote_commands(self);
    }

    // ──Sleep timer ─────────────────────────── ────────────────────────────

    fn tick_sleep_timer(&mut self, ctx: &egui::Context) {
        events::tick_sleep_timer(self, ctx);
    }

    // ── Background jobs ───────────────────────── ──────────────────────────

    fn poll_jobs(&mut self) {
        events::poll_jobs(self);
    }

    // ── Keyboard ───────────────────────────── ──────────────────────────────

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        actions::handle_keyboard(self, ctx);
    }

    // ── Menu ─────────────────────────────── ───────────────────────────────

    fn handle_menu_action(&mut self, ctx: &egui::Context, action: MenuAction) {
        actions::handle_menu_action(self, ctx, action);
    }

    /// Manages actions that come from contextual menus (right click).
    fn handle_context_action(
        &mut self,
        action: ContextAction,
        state: &crate::player::PlayerState,
        ctx: &egui::Context,
    ) {
        actions::handle_context_action(self, action, state, ctx);
    }

    fn toggle_side(&mut self, panel: SidePanel) {
        self.side_panel = if self.side_panel == panel {
            SidePanel::None
        } else {
            panel
        };
    }

    fn tick_scrobble(&mut self, pos: f64, paused: bool, duration: f64) {
        events::tick_scrobble(self, pos, paused, duration);
    }

    // ── Central area ─────────────────────────── ───────────────────────────

    fn draw_central(&mut self, ui: &mut egui::Ui, state: &crate::player::PlayerState) {
        if state.current_file.is_none() {
            return self.draw_empty_state(ui);
        }

        if let Some(renderer) = &self.renderer {
            let rect = ui.available_rect_before_wrap();
            let video_resp = ui.allocate_rect(rect, egui::Sense::click_and_drag());
            ui.painter().add(renderer.paint_callback(
                rect,
                ui.ctx().screen_rect(),
                ui.ctx().pixels_per_point(),
            ));

            // Right click context menu on the video
            let ca = video_context_menu(
                &video_resp,
                state,
                self.config.loudnorm,
                self.config.image_controls.deinterlace,
                self.config.image_controls.integer_scaling,
                self.config.language,
            );
            self.handle_context_action(ca, state, ui.ctx());

            if self.config.show_metrics_overlay {
                let text = format!(
                    "FPS {:.2} | Drop {} | HW {} | Buffer {:.2}s",
                    state.render_fps,
                    state.dropped_frames,
                    if state.hwdec_active { "ON" } else { "OFF" },
                    state.buffer_seconds
                );
                let font_size = self.config.metrics_overlay_font_size.clamp(9.0, 24.0);
                let alpha = (self.config.metrics_overlay_opacity.clamp(0.1, 1.0) * 255.0) as u8;
                let text_w = (text.chars().count() as f32 * font_size * 0.55).max(180.0);
                let bg_rect = egui::Rect::from_min_size(
                    rect.left_top() + egui::vec2(6.0, 6.0),
                    egui::vec2(text_w + 10.0, font_size + 12.0),
                );
                ui.painter().rect_filled(
                    bg_rect,
                    4.0,
                    Color32::from_rgba_unmultiplied(10, 12, 16, alpha),
                );
                ui.painter().text(
                    rect.left_top() + egui::vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    egui::FontId::proportional(font_size),
                    Color32::from_rgb(210, 220, 230),
                );
            }
        } else {
            // No renderer (audio only)
            let rect = ui.available_rect_before_wrap();
            let resp = ui.allocate_rect(rect, egui::Sense::hover());
            self.ensure_cover_for_current(state.current_file.as_deref(), ui.ctx());
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(28.0);
                    if let Some(tex) = &self.cover_texture {
                        let max_w = (ui.available_width() * 0.65).clamp(120.0, 360.0);
                        let max_h = (ui.available_height() * 0.55).clamp(120.0, 360.0);
                        let src = tex.size_vec2();
                        let scale = (max_w / src.x).min(max_h / src.y).min(1.0);
                        let size = egui::vec2(src.x * scale, src.y * scale);
                        ui.add(egui::Image::new((tex.id(), size)));
                    } else {
                        ui.label(egui::RichText::new("♪").size(64.0).color(theme::MUTED));
                    }
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new(&state.title)
                            .size(18.0)
                            .color(theme::TEXT),
                    );
                });
            });
            self.draw_audio_visualizer(ui, rect, state.position);
            let ca = video_context_menu(
                &resp,
                state,
                self.config.loudnorm,
                self.config.image_controls.deinterlace,
                self.config.image_controls.integer_scaling,
                self.config.language,
            );
            self.handle_context_action(ca, state, ui.ctx());
        }
    }

    fn draw_audio_visualizer(&self, ui: &mut egui::Ui, rect: egui::Rect, t: f64) {
        let bars = 48usize;
        let gap = 3.0f32;
        let pad = 24.0f32;
        let width = (rect.width() - pad * 2.0 - gap * (bars as f32 - 1.0)) / bars as f32;
        if width <= 1.0 {
            return;
        }

        let base_y = rect.bottom() - 26.0;
        for i in 0..bars {
            let phase = t as f32 * 2.8 + i as f32 * 0.33;
            let amp = (phase.sin().abs() * 0.65 + (phase * 0.7).cos().abs() * 0.35).clamp(0.0, 1.0);
            let h = 8.0 + amp * (rect.height() * 0.18).clamp(26.0, 72.0);
            let x = rect.left() + pad + i as f32 * (width + gap);
            let bar_rect =
                egui::Rect::from_min_max(egui::pos2(x, base_y - h), egui::pos2(x + width, base_y));
            let mix = i as f32 / (bars.saturating_sub(1) as f32).max(1.0);
            let color = blend(*theme::ACCENT, *theme::ACCENT2, mix);
            ui.painter().rect_filled(bar_rect, 2.0, color);
        }
    }

    fn draw_empty_state(&mut self, ui: &mut egui::Ui) {
        let bg = self.config.theme.bg_color();
        egui::Frame::none().fill(bg).show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                if self.drop_hover {
                    ui.label(
                        RichText::new(tr(self.config.language, "app.drop_here"))
                            .size(24.0)
                            .color(theme::ACCENT),
                    );
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(RichText::new("▶").size(72.0).color(theme::MUTED));
                        ui.add_space(14.0);
                        ui.label(
                            RichText::new(tr(self.config.language, "app.drag_or_open"))
                                .size(16.0)
                                .color(theme::MUTED),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(tr(self.config.language, "app.supported_formats"))
                                .size(11.0)
                                .color(Color32::from_rgb(55, 55, 70)),
                        );
                        ui.add_space(22.0);
                        if ui
                            .button(
                                RichText::new(tr(self.config.language, "app.open_file"))
                                    .size(14.0)
                                    .color(theme::TEXT),
                            )
                            .clicked()
                        {
                            self.open_file_dialog_single();
                        }
                    });
                }
            });
        });
    }

    // ── Side panel ────────────────────────── ───────────────────────────

    fn draw_side_panel(&mut self, ctx: &egui::Context, state: &crate::player::PlayerState) {
        panels::draw_side_panel(self, ctx, state);
    }

    // ── Floating windows ──────────────────────── ────────────────────────

    fn draw_floating_windows(&mut self, ctx: &egui::Context, state: &crate::player::PlayerState) {
        panels::draw_floating_windows(self, ctx, state);
    }
}

// ── eframe::App ─────────────────────────────── ────────────────────────────────

impl eframe::App for PlayerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_player_events();
        self.process_remote_commands();
        self.poll_jobs();
        self.settings_panel.tick_update_checks(&self.config);

        let state = self.player.state.lock().unwrap().clone();

        self.tick_scrobble(state.position, state.paused, state.duration);
        self.tick_sleep_timer(ctx);
        self.handle_keyboard(ctx);

        if (state.position - self.last_tick_pos).abs() >= 10.0 {
            self.save_history_entry();
        }

        // ── Top menu ──────────────────────── ─────────────────────────
        let mpv = self.player.mpv_handle();
        let menu_fill = self.config.theme.surface_color();
        let border_color = self.config.theme.surface2_color();
        egui::TopBottomPanel::top("menu_bar")
            .frame(
                egui::Frame::default()
                    .fill(menu_fill)
                    .inner_margin(egui::Margin::symmetric(16.0, 6.0)),
            )
            .show(ctx, |ui| {
                let ms = MenuState {
                    playlist_vis: self.side_panel == SidePanel::Playlist,
                    history_vis: self.side_panel == SidePanel::History,
                    bookmarks_vis: self.side_panel == SidePanel::Bookmarks,
                    notes_vis: self.side_panel == SidePanel::Notes,
                    chapters_vis: self.side_panel == SidePanel::Chapters,
                    info_vis: self.side_panel == SidePanel::MediaInfo,
                    theme_vis: self.side_panel == SidePanel::Theme,
                    karaoke_vis: self.side_panel == SidePanel::Karaoke,
                    subs_vis: self.side_panel == SidePanel::Subtitles,
                    upnext_vis: self.side_panel == SidePanel::UpNext,
                    perf_vis: self.side_panel == SidePanel::Performance,
                    library_vis: self.side_panel == SidePanel::Library,
                    codec_diag_vis: self.side_panel == SidePanel::CodecDiagnostics,
                    shuffle: self.config.shuffle,
                    loudnorm: self.config.loudnorm,
                    deinterlace: self.config.image_controls.deinterlace,
                    integer_scaling: self.config.image_controls.integer_scaling,
                    repeat: &self.config.repeat_mode,
                    current_speed: state.speed,
                };
                let a = draw_menu(ui, &ms, self.config.language, &mut self.ab_repeat, &mpv);

                // 1px bottom separator line
                let max_rect = ui.max_rect();
                ui.painter().line_segment(
                    [max_rect.left_bottom(), max_rect.right_bottom()],
                    egui::Stroke::new(1.0, border_color),
                );

                let ctx2 = ctx.clone();
                self.handle_menu_action(&ctx2, a);
            });

        // Donation
        egui::TopBottomPanel::bottom("donation").show(ctx, |ui| {
            if self.donation.show(ui) {
                self.config.show_donation_banner = false;
                self.config.save();
            }
        });

        // Controls
        let mut controls_action = controls::ControlsAction::None;
        egui::TopBottomPanel::bottom("controls")
            .frame(
                egui::Frame::default()
                    .fill(self.config.theme.bg_color())
                    .inner_margin(0.0),
            )
            .default_height(96.0)
            .show(ctx, |ui| {
                controls_action = controls::draw_controls(
                    ui,
                    &state,
                    &self.player,
                    &self.thumbnails,
                    &self.ab_repeat,
                    &self.config.repeat_mode,
                    self.config.shuffle,
                    self.config.audio_delay,
                    self.config.sub_delay,
                    &self.config.theme,
                );
            });

        match controls_action {
            controls::ControlsAction::ToggleRepeat => {
                self.config.repeat_mode = match self.config.repeat_mode {
                    RepeatMode::None => RepeatMode::All,
                    RepeatMode::All => RepeatMode::One,
                    RepeatMode::One => RepeatMode::None,
                };
                self.config.save();
            }
            controls::ControlsAction::ToggleShuffle => {
                self.config.shuffle = !self.config.shuffle;
                self.config.save();
            }
            controls::ControlsAction::Prev => self.play_prev(),
            controls::ControlsAction::Next => self.play_next(),
            controls::ControlsAction::None => {}
        }

        // side panel
        self.draw_side_panel(ctx, &state);

        // Video center
        let bg = self.config.theme.bg_color();
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(bg))
            .show(ctx, |ui| {
                self.draw_central(ui, &state);
            });

        // Windows
        self.draw_floating_windows(ctx, &state);

        if state.current_file.is_some() && !state.paused {
            ctx.request_repaint_after(std::time::Duration::from_millis(33));
        }
        if self.sleep_timer.enabled {
            ctx.request_repaint_after(std::time::Duration::from_secs(1));
        }
    }

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        self.save_history_entry();
        let s = self.player.state.lock().unwrap();
        self.config.volume = s.volume;
        self.config.muted = s.muted;
        self.config.speed = s.speed;
        self.config.show_playlist = self.side_panel == SidePanel::Playlist;
        drop(s);
        self.config.save();
        ThumbnailCache::clear_temp();

        if let Some(gl) = gl {
            if let Some(renderer) = &self.renderer {
                renderer.destroy_gl_resources(gl);
            }
        }
    }
}

// Simple pseudo-random (no extra dependencies)
fn rand_usize() -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize
}

fn parse_hex_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let hex = s.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

fn percent_encode_query_value(value: &str) -> String {
    let mut out = String::with_capacity(value.len() * 2);
    for b in value.bytes() {
        let keep = b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~');
        if keep {
            out.push(b as char);
        } else {
            out.push('%');
            out.push_str(&format!("{:02X}", b));
        }
    }
    out
}

fn build_bug_report_target(base: &str, title: &str, body: &str) -> String {
    let title = percent_encode_query_value(title);
    let body = percent_encode_query_value(body);
    if base.contains("{title}") || base.contains("{body}") {
        return base.replace("{title}", &title).replace("{body}", &body);
    }
    if base.starts_with("mailto:") {
        let sep = if base.contains('?') { '&' } else { '?' };
        return format!("{base}{sep}subject={title}&body={body}");
    }
    let sep = if base.contains('?') { '&' } else { '?' };
    format!("{base}{sep}title={title}&body={body}")
}

fn blend(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| -> u8 { (x as f32 + (y as f32 - x as f32) * t) as u8 };
    Color32::from_rgb(lerp(a.r(), b.r()), lerp(a.g(), b.g()), lerp(a.b(), b.b()))
}

const MEDIA_EXTS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "ts", "m2ts", "mpeg", "mpg", "ogv",
    "3gp", "rm", "rmvb", "divx", "vob", "cdg", "mp3", "flac", "ogg", "wav", "aac", "m4a", "opus",
    "wma", "ape", "mka", "ac3", "dts", "alac", "aiff",
];
