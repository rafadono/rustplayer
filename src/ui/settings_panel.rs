//! settings_panel.rs - General settings.

use crate::config::{AspectRatio, Config};
use crate::i18n::{tr, Language};
use crate::ui::theme::*;
use crate::updater::{self, UpdateChannel, UpdateInfo};
use egui::{Color32, RichText, Ui};
use std::sync::mpsc::{self, Receiver};

pub struct SettingsPanel {
    pub lastfm_username_input: String,
    pub lastfm_password_input: String,
    pub lastfm_status_msg: String,
    pub remote_port_input: String,
    pub bug_report_url_input: String,
    pub update_status_msg: String,
    pub latest_update: Option<UpdateInfo>,
    pub update_check_in_progress: bool,
    pub auto_check_started: bool,
    pub update_rx: Option<Receiver<Result<UpdateInfo, String>>>,
    pub lastfm_rx: Option<Receiver<Result<String, String>>>,
    pub lastfm_login_in_progress: bool,
}

impl SettingsPanel {
    pub fn new(config: &Config) -> Self {
        Self {
            lastfm_username_input: config.lastfm.username.clone(),
            lastfm_password_input: String::new(),
            lastfm_status_msg: String::new(),
            remote_port_input: config.remote_port.to_string(),
            bug_report_url_input: config.bug_report_url.clone(),
            update_status_msg: String::new(),
            latest_update: None,
            update_check_in_progress: false,
            auto_check_started: false,
            update_rx: None,
            lastfm_rx: None,
            lastfm_login_in_progress: false,
        }
    }

    pub fn tick_update_checks(&mut self, config: &Config) {
        if config.auto_check_updates && !self.auto_check_started {
            self.auto_check_started = true;
            self.start_update_check(config.update_channel);
        }
        self.poll_update_result(config);
    }

    fn start_update_check(&mut self, channel: UpdateChannel) {
        if self.update_check_in_progress {
            return;
        }
        let (tx, rx) = mpsc::channel::<Result<UpdateInfo, String>>();
        self.update_rx = Some(rx);
        self.update_check_in_progress = true;
        self.update_status_msg = "Buscando actualizaciones...".into();
        std::thread::spawn(move || {
            let result = updater::check_for_updates(channel);
            let _ = tx.send(result);
        });
    }

    fn poll_update_result(&mut self, config: &Config) {
        let Some(rx) = &self.update_rx else {
            return;
        };
        let Ok(result) = rx.try_recv() else {
            return;
        };
        self.update_check_in_progress = false;
        self.update_rx = None;
        match result {
            Ok(info) => {
                let current = env!("CARGO_PKG_VERSION");
                if updater::is_newer(current, &info.version) {
                    self.update_status_msg = format!(
                        "Nueva version disponible: {} ({})",
                        info.version,
                        ch_label(config)
                    );
                    self.latest_update = Some(info);
                } else {
                    self.update_status_msg = format!("Sin updates. Actual: {}", current);
                    self.latest_update = None;
                }
            }
            Err(e) => {
                self.update_status_msg = e;
                self.latest_update = None;
            }
        }
    }

    fn start_lastfm_login(&mut self, user: String, pass: String) {
        if self.lastfm_login_in_progress {
            return;
        }
        let (tx, rx) = mpsc::channel::<Result<String, String>>();
        self.lastfm_rx = Some(rx);
        self.lastfm_login_in_progress = true;
        self.lastfm_status_msg = "Iniciando sesión...".into();
        std::thread::spawn(move || {
            let result = crate::lastfm::get_session(&user, &pass);
            let _ = tx.send(result);
        });
    }

    fn poll_lastfm_result(&mut self, config: &mut Config) -> bool {
        let Some(rx) = &self.lastfm_rx else {
            return false;
        };
        let Ok(result) = rx.try_recv() else {
            return false;
        };
        self.lastfm_login_in_progress = false;
        self.lastfm_rx = None;
        match result {
            Ok(session_key) => {
                self.lastfm_status_msg = "OK: Conectado".into();
                config.lastfm.session_key = session_key;
                config.lastfm.username = self.lastfm_username_input.clone();
                config.lastfm.enabled = true;
                true
            }
            Err(e) => {
                self.lastfm_status_msg = format!("Error: {}", e);
                config.lastfm.session_key = String::new();
                true
            }
        }
    }

    pub fn show(&mut self, ui: &mut Ui, config: &mut Config) -> bool {
        let mut changed = false;
        let lang = config.language;
        self.poll_update_result(config);
        changed |= self.poll_lastfm_result(config);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                section_header(ui, tr(lang, "settings.language"));
                ui.label(
                    RichText::new(tr(lang, "settings.ui_language"))
                        .color(MUTED)
                        .size(11.0),
                );
                let mut ui_lang = config.language;
                egui::ComboBox::from_id_source("ui_language")
                    .selected_text(ui_lang.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut ui_lang, Language::Es, Language::Es.label());
                        ui.selectable_value(&mut ui_lang, Language::En, Language::En.label());
                    });
                if ui_lang != config.language {
                    config.language = ui_lang;
                    changed = true;
                }

                ui.add_space(16.0);
                section_header(ui, "Last.fm");
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Activar scrobbling").color(TEXT).size(12.0));
                    changed |= ui.toggle_value(&mut config.lastfm.enabled, "").changed();
                });
                ui.label(RichText::new("Usuario").color(MUTED).size(11.0));
                if ui
                    .add(
                        egui::TextEdit::singleline(&mut self.lastfm_username_input)
                            .desired_width(220.0)
                            .hint_text("usuario de Last.fm"),
                    )
                    .lost_focus()
                {
                    config.lastfm.username = self.lastfm_username_input.clone();
                    changed = true;
                }
                ui.label(RichText::new("Contrasena (sesion)").color(MUTED).size(11.0));
                ui.add(
                    egui::TextEdit::singleline(&mut self.lastfm_password_input)
                        .desired_width(220.0)
                        .password(true)
                        .hint_text("contrasena"),
                );
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    let btn_label = if self.lastfm_login_in_progress {
                        "Conectando..."
                    } else {
                        "Conectar Last.fm"
                    };
                    if ui
                        .add_enabled(!self.lastfm_login_in_progress, egui::Button::new(btn_label))
                        .clicked()
                    {
                        let user = self.lastfm_username_input.clone();
                        let pass = self.lastfm_password_input.clone();
                        self.start_lastfm_login(user, pass);
                    }
                });
                if !self.lastfm_status_msg.is_empty() {
                    let color = if self.lastfm_status_msg.starts_with("OK") {
                        SUCCESS
                    } else {
                        DANGER
                    };
                    ui.label(
                        RichText::new(&self.lastfm_status_msg)
                            .color(color)
                            .size(11.0),
                    );
                }

                ui.add_space(16.0);
                section_header(ui, "Control remoto HTTP");
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Activar servidor").color(TEXT).size(12.0));
                    changed |= ui.toggle_value(&mut config.remote_enabled, "").changed();
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Puerto").color(MUTED).size(11.0));
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.remote_port_input)
                                .desired_width(70.0),
                        )
                        .lost_focus()
                    {
                        if let Ok(p) = self.remote_port_input.parse::<u16>() {
                            config.remote_port = p;
                            changed = true;
                        }
                    }
                });

                ui.add_space(16.0);
                section_header(ui, "Video");
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Aspecto arbitrario").color(MUTED).size(11.0));
                    let mut custom_aspect = match config.aspect_ratio {
                        AspectRatio::Custom(v) => v,
                        _ => 16.0 / 9.0,
                    };
                    if ui
                        .add(
                            egui::DragValue::new(&mut custom_aspect)
                                .speed(0.01)
                                .clamp_range(0.2..=5.0)
                                .prefix("W/H "),
                        )
                        .changed()
                    {
                        config.aspect_ratio = AspectRatio::Custom(custom_aspect);
                        changed = true;
                    }
                    if ui.button("Auto").clicked() {
                        config.aspect_ratio = AspectRatio::Auto;
                        changed = true;
                    }
                });

                ui.add_space(16.0);
                section_header(ui, "Subtitulos");
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Tamano").color(MUTED).size(11.0));
                    let mut sz = config.sub_font_size as f32;
                    if ui
                        .add_sized(
                            [150.0, 0.0],
                            egui::Slider::new(&mut sz, 16.0..=96.0).step_by(1.0),
                        )
                        .changed()
                    {
                        config.sub_font_size = sz as i64;
                        changed = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Opacidad").color(MUTED).size(11.0));
                    changed |= ui
                        .add_sized(
                            [150.0, 0.0],
                            egui::Slider::new(&mut config.sub_opacity, 0.0..=1.0).step_by(0.01),
                        )
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Fuente").color(MUTED).size(11.0));
                    egui::ComboBox::from_id_source("sub_font_family")
                        .selected_text(config.sub_font_family.clone())
                        .show_ui(ui, |ui| {
                            for fam in ["sans-serif", "serif", "monospace", "Arial", "Noto Sans"] {
                                if ui
                                    .selectable_label(config.sub_font_family == fam, fam)
                                    .clicked()
                                {
                                    config.sub_font_family = fam.to_string();
                                    changed = true;
                                }
                            }
                        });
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Color").color(MUTED).size(11.0));
                    let mut c = hex_to_color32(&config.sub_color).unwrap_or(Color32::WHITE);
                    if ui.color_edit_button_srgba(&mut c).changed() {
                        config.sub_color = color32_to_hex(c);
                        changed = true;
                    }
                    ui.label(RichText::new(&config.sub_color).color(MUTED).size(10.0));
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Negrita").color(MUTED).size(11.0));
                    changed |= ui.toggle_value(&mut config.sub_bold, "").changed();
                });

                ui.add_space(16.0);
                section_header(ui, "Rendimiento");
                changed |= ui
                    .checkbox(
                        &mut config.show_metrics_overlay,
                        "Mostrar overlay de metricas",
                    )
                    .changed();
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Opacidad overlay").color(MUTED).size(11.0));
                    changed |= ui
                        .add_sized(
                            [160.0, 0.0],
                            egui::Slider::new(&mut config.metrics_overlay_opacity, 0.1..=1.0)
                                .step_by(0.01),
                        )
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Tamano letra overlay")
                            .color(MUTED)
                            .size(11.0),
                    );
                    changed |= ui
                        .add_sized(
                            [160.0, 0.0],
                            egui::Slider::new(&mut config.metrics_overlay_font_size, 9.0..=24.0)
                                .step_by(1.0),
                        )
                        .changed();
                });

                ui.add_space(16.0);
                section_header(ui, tr(lang, "settings.bug_reports"));
                ui.label(
                    RichText::new(tr(lang, "settings.bug_reports_hint"))
                        .color(MUTED)
                        .size(11.0),
                );
                if ui
                    .add(
                        egui::TextEdit::singleline(&mut self.bug_report_url_input)
                            .desired_width(360.0)
                            .hint_text(tr(lang, "settings.bug_reports_placeholder")),
                    )
                    .lost_focus()
                {
                    config.bug_report_url = self.bug_report_url_input.trim().to_string();
                    changed = true;
                }

                ui.add_space(16.0);
                section_header(ui, "Actualizaciones");
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(tr(lang, "settings.auto_check_updates"))
                            .color(TEXT)
                            .size(12.0),
                    );
                    changed |= ui
                        .toggle_value(&mut config.auto_check_updates, "")
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Canal").color(MUTED).size(11.0));
                    let mut ch = config.update_channel;
                    egui::ComboBox::from_id_source("update_channel")
                        .selected_text(ch.label())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut ch, UpdateChannel::Stable, "Stable");
                            ui.selectable_value(&mut ch, UpdateChannel::Beta, "Beta");
                        });
                    if ch != config.update_channel {
                        config.update_channel = ch;
                        changed = true;
                    }
                });

                if ui
                    .add_enabled(
                        !self.update_check_in_progress,
                        egui::Button::new(tr(lang, "settings.check_updates_now")),
                    )
                    .clicked()
                {
                    self.start_update_check(config.update_channel);
                }
                if !self.update_status_msg.is_empty() {
                    ui.label(
                        RichText::new(&self.update_status_msg)
                            .size(10.0)
                            .color(MUTED),
                    );
                }
                if let Some(info) = &self.latest_update {
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(format!(
                            "{} {}",
                            tr(lang, "settings.update_available"),
                            info.version
                        ))
                        .size(11.0)
                        .color(SUCCESS),
                    );
                    if ui.button(tr(lang, "settings.install_update_now")).clicked() {
                        match std::env::current_exe() {
                            Ok(current_exe) => {
                                match updater::install_update_with_rollback(
                                    &info.download_url,
                                    &current_exe,
                                ) {
                                    Ok(()) => {
                                        self.update_status_msg =
                                            tr(lang, "settings.install_started").to_string();
                                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                                    }
                                    Err(e) => {
                                        self.update_status_msg = format!(
                                            "{}: {}",
                                            tr(lang, "settings.install_failed"),
                                            e
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                self.update_status_msg =
                                    format!("{}: {}", tr(lang, "settings.install_failed"), e);
                            }
                        }
                    }
                }
            });
        });

        changed
    }
}

fn ch_label(config: &Config) -> &'static str {
    match config.update_channel {
        UpdateChannel::Stable => "Stable",
        UpdateChannel::Beta => "Beta",
    }
}

fn section_header(ui: &mut Ui, title: &str) {
    ui.label(RichText::new(title).color(TEXT).size(13.0).strong());
    ui.separator();
    ui.add_space(6.0);
}

fn hex_to_color32(s: &str) -> Option<Color32> {
    let hex = s.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color32::from_rgb(r, g, b))
}

fn color32_to_hex(c: Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r(), c.g(), c.b())
}
