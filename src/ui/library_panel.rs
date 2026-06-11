use crate::history::HistoryEntry;
use crate::ui::theme::*;
use egui::{RichText, Ui};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Default)]
pub struct LibraryPanel {
    pub folder_filter: String,
    pub ext_filter: String,
    pub min_duration_s: f64,
    pub max_age_days: i64,
    selected: HashSet<String>,
}

pub enum LibraryAction {
    AddSelectedToPlaylist(Vec<PathBuf>),
    AddSelectedToQueue(Vec<PathBuf>),
    RemoveSelectedFromIndex(Vec<PathBuf>),
    RescanSelected(Vec<PathBuf>),
    None,
}

impl LibraryPanel {
    pub fn show(&mut self, ui: &mut Ui, entries: &[HistoryEntry]) -> LibraryAction {
        let mut action = LibraryAction::None;
        ui.label(
            RichText::new("Biblioteca local (índice historial)")
                .color(TEXT)
                .size(13.0),
        );
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Carpeta");
            ui.text_edit_singleline(&mut self.folder_filter);
            ui.label("Tipo");
            ui.text_edit_singleline(&mut self.ext_filter);
        });
        ui.horizontal(|ui| {
            ui.label("Duración mín (s)");
            ui.add(egui::DragValue::new(&mut self.min_duration_s).speed(10.0));
            ui.label("Últimos días");
            ui.add(egui::DragValue::new(&mut self.max_age_days).speed(1.0));
        });
        ui.separator();

        let filtered: Vec<&HistoryEntry> = entries.iter().filter(|e| self.matches(e)).collect();

        ui.horizontal(|ui| {
            if ui.small_button("Playlist").clicked() {
                action = LibraryAction::AddSelectedToPlaylist(self.selected_paths());
            }
            if ui.small_button("Up Next").clicked() {
                action = LibraryAction::AddSelectedToQueue(self.selected_paths());
            }
            if ui.small_button("Quitar índice").clicked() {
                action = LibraryAction::RemoveSelectedFromIndex(self.selected_paths());
            }
            if ui.small_button("Re-scan parcial").clicked() {
                action = LibraryAction::RescanSelected(self.selected_paths());
            }
        });
        ui.add_space(4.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for e in filtered {
                let key = e.path.to_string_lossy().to_string();
                let mut checked = self.selected.contains(&key);
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut checked, "").changed() {
                        if checked {
                            self.selected.insert(key.clone());
                        } else {
                            self.selected.remove(&key);
                        }
                    }
                    let ext = e.path.extension().and_then(|x| x.to_str()).unwrap_or("-");
                    ui.label(
                        RichText::new(format!("[{}] {}", ext, e.title))
                            .size(11.0)
                            .color(TEXT),
                    );
                });
            }
        });

        action
    }

    fn matches(&self, e: &HistoryEntry) -> bool {
        if !self.folder_filter.trim().is_empty() {
            let f = self.folder_filter.to_lowercase();
            let p = e
                .path
                .parent()
                .map(|p| p.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            if !p.contains(&f) {
                return false;
            }
        }
        if !self.ext_filter.trim().is_empty() {
            let ext = e
                .path
                .extension()
                .and_then(|x| x.to_str())
                .unwrap_or("")
                .to_lowercase();
            if ext != self.ext_filter.trim().to_lowercase() {
                return false;
            }
        }
        if self.min_duration_s > 0.0 && e.duration < self.min_duration_s {
            return false;
        }
        if self.max_age_days > 0 {
            let age = chrono::Utc::now()
                .signed_duration_since(e.last_watched)
                .num_days();
            if age > self.max_age_days {
                return false;
            }
        }
        true
    }

    fn selected_paths(&self) -> Vec<PathBuf> {
        self.selected.iter().map(PathBuf::from).collect()
    }
}
