//! media_info_panel.rs — Technical information of the current file.

use crate::media_info::MediaInfo;
use crate::player::PlayerState;
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct MediaInfoPanel;

impl MediaInfoPanel {
    pub fn show(ui: &mut Ui, info: &Option<MediaInfo>, state: &PlayerState) {
        ui.vertical(|ui| {
            ui.label(
                RichText::new("Información de medios")
                    .color(TEXT)
                    .size(13.0),
            );
            ui.add_space(8.0);

            let Some(info) = info else {
                ui.label(RichText::new("Sin archivo activo").color(MUTED).size(12.0));
                return;
            };

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    section(ui, "Archivo");
                    row(ui, "Nombre", &info.filename);
                    row(ui, "Formato", &info.format);
                    row(ui, "Tamaño", &info.size_str());
                    row(ui, "Duración", &PlayerState::format_time(info.duration));

                    if info.width > 0 {
                        ui.add_space(8.0);
                        section(ui, "Video");
                        row(ui, "Codec", &info.video_codec);
                        row(ui, "Resolución", &info.resolution());
                        row(
                            ui,
                            "FPS",
                            &if info.fps > 0.0 {
                                format!("{:.3}", info.fps)
                            } else {
                                "—".into()
                            },
                        );
                        row(ui, "Bitrate", &info.video_bitrate_str());
                        if !info.color_space.is_empty() {
                            row(ui, "Color", &info.color_space);
                        }
                    }

                    if !info.audio_codec.is_empty() {
                        ui.add_space(8.0);
                        section(ui, "Audio");
                        row(ui, "Codec", &info.audio_codec);
                        row(ui, "Canales", &info.channel_str());
                        row(ui, "Bitrate", &info.audio_bitrate_str());
                        if info.sample_rate > 0 {
                            row(ui, "Sample rate", &format!("{} Hz", info.sample_rate));
                        }
                    }

                    if info.sub_count > 0 {
                        ui.add_space(8.0);
                        section(ui, "Subtítulos");
                        row(ui, "Pistas", &info.sub_count.to_string());
                    }

                    // Real time status
                    if state.current_file.is_some() {
                        ui.add_space(8.0);
                        section(ui, "En tiempo real");
                        row(ui, "Posición", &PlayerState::format_time(state.position));
                        row(ui, "Velocidad", &format!("{:.2}×", state.speed));
                        row(ui, "Volumen", &format!("{}%", state.volume));
                    }
                });
        });
    }
}

fn section(ui: &mut Ui, title: &str) {
    ui.label(RichText::new(title).color(ACCENT).size(11.0).strong());
    ui.separator();
    ui.add_space(2.0);
}

fn row(ui: &mut Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.add_sized(
            [90.0, 16.0],
            egui::Label::new(RichText::new(key).color(MUTED).size(11.0)),
        );
        ui.label(RichText::new(value).color(TEXT).size(11.0));
    });
}
