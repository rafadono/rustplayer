//! converter_panel.rs — Format converter via ffmpeg.

use crate::converter::{ConvertJob, ConvertPreset};
use crate::streaming::ffmpeg_available;
use crate::ui::theme::*;
use egui::{RichText, Ui};
use std::path::Path;

pub struct ConverterPanel {
    pub preset: ConvertPreset,
    pub output: String,
    running: bool,
    status_msg: String,
}

impl ConverterPanel {
    pub fn new() -> Self {
        Self {
            preset: ConvertPreset::Mp4H264,
            output: String::new(),
            running: false,
            status_msg: String::new(),
        }
    }

    pub fn clear_job(&mut self) {
        self.running = false;
        self.status_msg.clear();
    }

    /// Returns ConvertJob if the user launched conversion.
    pub fn show(&mut self, ui: &mut Ui, file: Option<&Path>) -> Option<ConvertJob> {
        let mut new_job: Option<ConvertJob> = None;

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

            ui.label(RichText::new("Archivo").color(MUTED).size(11.0));
            ui.label(
                RichText::new(file.file_name().unwrap_or_default().to_string_lossy())
                    .color(TEXT)
                    .size(12.0),
            );
            ui.add_space(8.0);

            // Preset selector
            ui.label(RichText::new("Formato de salida").color(MUTED).size(11.0));
            egui::ComboBox::from_id_source("convert_preset")
                .selected_text(RichText::new(self.preset.label()).size(12.0).color(TEXT))
                .show_ui(ui, |ui| {
                    for preset in ConvertPreset::all() {
                        if ui
                            .selectable_label(self.preset == *preset, preset.label())
                            .clicked()
                        {
                            self.preset = preset.clone();
                            // Update output
                            self.output = ConvertJob::default_output(file, &self.preset)
                                .to_string_lossy()
                                .to_string();
                        }
                    }
                });

            ui.add_space(8.0);

            // Exit route
            if self.output.is_empty() {
                self.output = ConvertJob::default_output(file, &self.preset)
                    .to_string_lossy()
                    .to_string();
            }

            ui.label(RichText::new("Archivo de salida").color(MUTED).size(11.0));
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.output)
                        .desired_width(ui.available_width() - 70.0),
                );
                if ui.small_button("Elegir...").clicked() {
                    if let Some(p) = rfd::FileDialog::new()
                        .add_filter(self.preset.label(), &[self.preset.extension()])
                        .save_file()
                    {
                        self.output = p.to_string_lossy().to_string();
                    }
                }
            });

            ui.add_space(10.0);

            let can_run = !self.running && !self.output.is_empty();
            if ui
                .add_enabled(
                    can_run,
                    egui::Button::new(RichText::new("Convertir").color(TEXT)),
                )
                .clicked()
            {
                if let Some(job) = ConvertJob::start(file, self.output.clone().into(), &self.preset)
                {
                    self.running = true;
                    self.status_msg = "Convirtiendo... puede tardar varios minutos.".into();
                    new_job = Some(job);
                } else {
                    self.status_msg = "Error al iniciar ffmpeg".into();
                }
            }

            if !self.status_msg.is_empty() {
                ui.add_space(4.0);
                let color = if self.running { WARNING } else { MUTED };
                ui.label(RichText::new(&self.status_msg).color(color).size(12.0));
            }
        });

        new_job
    }
}
