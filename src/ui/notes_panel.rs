//! notes_panel.rs — Timestamped notes per file.

use crate::notes::{Note, NoteStore};
use crate::player::PlayerState;
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct NotesPanel;

impl NotesPanel {
    pub fn show(ui: &mut Ui, store: &mut NoteStore, state: &PlayerState) {
        let file = state.current_file.as_deref();

        ui.vertical(|ui| {
            ui.label(RichText::new("Notas").color(TEXT).size(13.0));
            ui.add_space(8.0);

            let Some(file) = file else {
                ui.label(RichText::new("Sin archivo activo").color(MUTED).size(12.0));
                return;
            };

            // Field for new note
            let mut new_text = String::new();
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut new_text)
                        .hint_text("Nueva nota...")
                        .desired_width(ui.available_width() - 60.0),
                );
                if ui
                    .add_enabled(
                        !new_text.trim().is_empty(),
                        egui::Button::new(RichText::new("Añadir").size(11.0)),
                    )
                    .clicked()
                {
                    store.add(file, Note::new(state.position, new_text.trim()));
                    store.save();
                }
            });

            ui.add_space(10.0);

            let notes: Vec<crate::notes::Note> = store.get(file).to_vec();

            if notes.is_empty() {
                ui.label(RichText::new("Sin notas aún.").color(MUTED).size(12.0));
                return;
            }

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let mut to_delete: Option<u64> = None;

                    for note in &notes {
                        egui::Frame {
                            fill: *SURFACE2,
                            inner_margin: egui::Margin::symmetric(8.0, 6.0),
                            rounding: egui::Rounding::same(4.0),
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "[{}]",
                                        PlayerState::format_time(note.position)
                                    ))
                                    .size(10.0)
                                    .color(ACCENT),
                                );
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui
                                            .small_button(RichText::new("✕").color(MUTED))
                                            .clicked()
                                        {
                                            to_delete = Some(note.id);
                                        }
                                    },
                                );
                            });
                            ui.label(RichText::new(&note.text).size(12.0).color(TEXT));
                        });
                        ui.add_space(2.0);
                    }

                    if let Some(id) = to_delete {
                        store.remove(file, id);
                        store.save();
                    }
                });

            ui.add_space(8.0);
            // Export notes
            if ui
                .small_button(
                    RichText::new("Exportar notas como texto")
                        .size(11.0)
                        .color(MUTED),
                )
                .clicked()
            {
                let text = store.export_text(file);
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Texto", &["txt"])
                    .set_file_name("notas.txt")
                    .save_file()
                {
                    let _ = std::fs::write(path, text);
                }
            }
        });
    }
}
