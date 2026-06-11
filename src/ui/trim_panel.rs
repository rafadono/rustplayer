//! trim_panel.rs — Trim video without re-encoding (ffmpeg -c copy).

use crate::player::PlayerState;
use crate::streaming::ffmpeg_available;
use crate::trim::{default_output, TrimJob};
use crate::ui::theme::*;
use egui::{RichText, Ui};
use std::path::Path;

pub struct TrimPanel {
    pub start: f64,
    pub end: f64,
    pub output: String,
    running: bool,
    status_msg: String,
}

impl TrimPanel {
    pub fn new() -> Self {
        Self {
            start: 0.0,
            end: 0.0,
            output: String::new(),
            running: false,
            status_msg: String::new(),
        }
    }

    pub fn clear_job(&mut self) {
        self.running = false;
        self.status_msg.clear();
    }

    /// Returns a TrimJob if the user launched the trim.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        file: Option<&Path>,
        duration: f64,
        position: f64,
    ) -> Option<TrimJob> {
        let mut new_job: Option<TrimJob> = None;

        ui.vertical(|ui| {
            if !ffmpeg_available() {
                ui.label(
                    RichText::new("⚠ ffmpeg no encontrado en PATH")
                        .color(WARNING)
                        .size(12.0),
                );
                ui.label(
                    RichText::new("Instalar ffmpeg para usar esta función.")
                        .color(MUTED)
                        .size(11.0),
                );
                return;
            }

            let Some(file) = file else {
                ui.label(
                    RichText::new("Abrir un archivo primero.")
                        .color(MUTED)
                        .size(12.0),
                );
                return;
            };

            ui.horizontal(|ui| {
                ui.label(RichText::new("Inicio").color(MUTED).size(12.0));
                ui.add(
                    egui::DragValue::new(&mut self.start)
                        .speed(0.5)
                        .clamp_range(0.0..=duration)
                        .suffix("s"),
                );
                if ui.small_button("← Pos actual").clicked() {
                    self.start = position;
                }
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("Fin   ").color(MUTED).size(12.0));
                ui.add(
                    egui::DragValue::new(&mut self.end)
                        .speed(0.5)
                        .clamp_range(0.0..=duration)
                        .suffix("s"),
                );
                if ui.small_button("← Pos actual").clicked() {
                    self.end = position;
                }
            });

            // Trim Duration
            let clip_dur = (self.end - self.start).max(0.0);
            ui.label(
                RichText::new(format!("Duración: {}", PlayerState::format_time(clip_dur)))
                    .color(MUTED)
                    .size(11.0),
            );

            ui.add_space(8.0);

            // Exit route
            if self.output.is_empty() {
                let def = default_output(file, self.start, self.end);
                self.output = def.to_string_lossy().to_string();
            }
            ui.label(RichText::new("Archivo de salida").color(MUTED).size(11.0));
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.output)
                        .desired_width(ui.available_width() - 70.0),
                );
                if ui.small_button("Elegir...").clicked() {
                    if let Some(p) = rfd::FileDialog::new().save_file() {
                        self.output = p.to_string_lossy().to_string();
                    }
                }
            });

            ui.add_space(10.0);

            let can_run = !self.running && self.end > self.start && !self.output.is_empty();
            if ui
                .add_enabled(
                    can_run,
                    egui::Button::new(RichText::new("Recortar").color(TEXT)),
                )
                .clicked()
            {
                if let Some(job) =
                    TrimJob::start(file, self.start, self.end, self.output.clone().into())
                {
                    self.running = true;
                    self.status_msg = "Recortando...".into();
                    new_job = Some(job);
                } else {
                    self.status_msg = "Error al iniciar ffmpeg".into();
                }
            }

            if !self.status_msg.is_empty() {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(&self.status_msg)
                        .color(if self.running { WARNING } else { SUCCESS })
                        .size(12.0),
                );
            }
        });

        new_job
    }
}
