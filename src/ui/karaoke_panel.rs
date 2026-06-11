//! karaoke_panel.rs — Karaoke mode with .CDG files.

use crate::karaoke::{classify, list_cdg_files, KaraokeFileType, USAGE_HINT};
use crate::ui::theme::*;
use egui::{RichText, Ui};
use std::path::PathBuf;

pub struct KaraokePanel;

impl KaraokePanel {
    /// Returns Some(path) if the user wants to open a CDG or audio+CDG file.
    pub fn show(ui: &mut Ui, current_file: Option<&std::path::Path>) -> Option<PathBuf> {
        let mut open_path: Option<PathBuf> = None;

        ui.vertical(|ui| {
            ui.label(RichText::new("Karaoke (.CDG)").color(TEXT).size(13.0));
            ui.add_space(8.0);

            // Current file status
            if let Some(file) = current_file {
                let kind = classify(file);
                match &kind {
                    KaraokeFileType::AudioWithCdg(cdg) => {
                        ui.label(
                            RichText::new("✓ CDG detectado automáticamente")
                                .color(SUCCESS)
                                .size(12.0),
                        );
                        ui.label(
                            RichText::new(cdg.file_name().unwrap_or_default().to_string_lossy())
                                .color(MUTED)
                                .size(11.0),
                        );
                    }
                    KaraokeFileType::Cdg => {
                        ui.label(
                            RichText::new("Reproduciendo archivo CDG")
                                .color(ACCENT)
                                .size(12.0),
                        );
                    }
                    KaraokeFileType::Audio => {
                        ui.label(
                            RichText::new("Sin CDG asociado en la misma carpeta")
                                .color(MUTED)
                                .size(12.0),
                        );
                    }
                }

                // CDG files in the same directory
                if let Some(dir) = file.parent() {
                    let cdg_files = list_cdg_files(dir);
                    if !cdg_files.is_empty() {
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!("CDG en esta carpeta ({})", cdg_files.len()))
                                .color(MUTED)
                                .size(11.0),
                        );
                        egui::ScrollArea::vertical()
                            .max_height(120.0)
                            .show(ui, |ui| {
                                for cdg in &cdg_files {
                                    let name =
                                        cdg.file_name().unwrap_or_default().to_string_lossy();
                                    if ui
                                        .selectable_label(
                                            false,
                                            RichText::new(name.as_ref()).size(12.0).color(TEXT),
                                        )
                                        .on_hover_text("Abrir para karaoke")
                                        .clicked()
                                    {
                                        open_path = Some(cdg.clone());
                                    }
                                }
                            });
                    }
                }
            } else {
                ui.label(RichText::new("Sin archivo activo.").color(MUTED).size(12.0));
            }

            ui.add_space(8.0);

            // Open CDG manually
            if ui
                .button(
                    RichText::new("Abrir archivo .CDG...")
                        .size(12.0)
                        .color(MUTED),
                )
                .clicked()
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Karaoke CDG", &["cdg", "CDG"])
                    .add_filter("Audio MP3", &["mp3", "ogg", "wav", "flac"])
                    .pick_file()
                {
                    open_path = Some(path);
                }
            }

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(6.0);

            ui.label(RichText::new("Instrucciones").color(MUTED).size(11.0));
            ui.add_space(2.0);
            for line in USAGE_HINT.lines() {
                ui.label(RichText::new(line).color(MUTED).size(10.0));
            }
        });

        open_path
    }
}
