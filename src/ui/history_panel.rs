//! history_panel.rs - History with context menu.

use crate::history::{History, HistoryEntry};
use crate::i18n::{tr, Language};
use crate::player::PlayerState;
use crate::ui::context_menu::{history_item_context_menu, ContextAction};
use crate::ui::theme::*;
use egui::{Color32, RichText, Ui};
use std::path::PathBuf;

pub struct HistoryPanel;

impl HistoryPanel {
    pub fn show(
        ui: &mut Ui,
        hist: &mut History,
        state: &PlayerState,
        lang: Language,
    ) -> (Option<PathBuf>, ContextAction) {
        let mut open_path: Option<PathBuf> = None;
        let mut ctx_action: ContextAction = ContextAction::None;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(tr(lang, "panel.history_title"))
                        .color(TEXT)
                        .size(13.0),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .small_button(
                            RichText::new(tr(lang, "panel.clear_all"))
                                .color(MUTED)
                                .size(11.0),
                        )
                        .clicked()
                    {
                        hist.clear();
                        hist.save();
                    }
                });
            });
            ui.add_space(6.0);

            if hist.all_entries().is_empty() {
                ui.label(
                    RichText::new(tr(lang, "panel.no_recent_files"))
                        .color(MUTED)
                        .size(12.0),
                );
                return;
            }

            let entries: Vec<_> = hist.recent().take(50).collect();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for entry in entries {
                        let is_current = state
                            .current_file
                            .as_deref()
                            .map(|f| f == entry.path)
                            .unwrap_or(false);

                        let resp = draw_history_row(ui, entry, is_current);

                        if resp.double_clicked() {
                            open_path = Some(entry.path.clone());
                        }

                        let ca = history_item_context_menu(&resp, &entry.path, lang);
                        if ca != ContextAction::None {
                            ctx_action = ca;
                        }
                    }
                });
        });

        (open_path, ctx_action)
    }
}

fn draw_history_row(ui: &mut Ui, entry: &HistoryEntry, is_current: bool) -> egui::Response {
    let fill = if is_current {
        Color32::from_rgb(30, 46, 64)
    } else {
        *SURFACE
    };
    let progress = if entry.duration > 0.0 {
        (entry.last_position / entry.duration).clamp(0.0, 1.0) as f32
    } else {
        0.0
    };

    egui::Frame {
        fill,
        inner_margin: egui::Margin::symmetric(10.0, 6.0),
        rounding: egui::Rounding::same(4.0),
        ..Default::default()
    }
    .show(ui, |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let title = entry
                    .path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| entry.path.to_string_lossy().to_string());

                let color = if is_current { ACCENT } else { TEXT };
                ui.label(RichText::new(&title).size(12.0).color(color));
            });

            if progress > 0.0 {
                let (bar_rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 3.0),
                    egui::Sense::hover(),
                );
                ui.painter()
                    .rect_filled(bar_rect, 1.5, Color32::from_rgb(40, 40, 55));
                let fill_w = bar_rect.width() * progress;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(bar_rect.min, egui::vec2(fill_w, 3.0)),
                    1.5,
                    ACCENT,
                );
                ui.label(
                    RichText::new(format!("{:.0}%", progress * 100.0))
                        .size(10.0)
                        .color(MUTED),
                );
            }
        });
    })
    .response
}
