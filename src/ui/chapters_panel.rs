//! chapters_panel.rs — Chapter navigation.

use crate::chapters::Chapter;
use crate::player::PlayerState;
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct ChaptersPanel;

impl ChaptersPanel {
    /// Returns Some(time) if the user clicked a chapter.
    pub fn show(ui: &mut Ui, chapters: &[Chapter], current_idx: Option<usize>) -> Option<f64> {
        let mut seek_to: Option<f64> = None;

        ui.vertical(|ui| {
            ui.label(RichText::new("Capítulos").color(TEXT).size(13.0));
            ui.add_space(6.0);

            if chapters.is_empty() {
                ui.label(
                    RichText::new("Sin capítulos en este archivo.")
                        .color(MUTED)
                        .size(12.0),
                );
                return;
            }

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for ch in chapters {
                        let is_active = current_idx == Some(ch.index);
                        let label = format!(
                            "{:2}.  {}  — {}",
                            ch.index + 1,
                            ch.title,
                            PlayerState::format_time(ch.time),
                        );
                        if ui
                            .selectable_label(
                                is_active,
                                RichText::new(label).size(12.0).color(if is_active {
                                    ACCENT
                                } else {
                                    TEXT
                                }),
                            )
                            .clicked()
                        {
                            seek_to = Some(ch.time);
                        }
                        ui.add_space(1.0);
                    }
                });
        });

        seek_to
    }
}
