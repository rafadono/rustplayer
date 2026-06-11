use crate::ui::theme::*;
use crate::up_next::QueueItem;
use egui::{RichText, Ui};

#[derive(Debug, Clone)]
pub enum UpNextAction {
    PlayNow(usize),
    Dequeue(usize),
    Clear,
    None,
}

pub fn show(ui: &mut Ui, items: &[QueueItem]) -> UpNextAction {
    let mut action = UpNextAction::None;

    ui.horizontal(|ui| {
        ui.label(RichText::new("Cola Up Next").color(TEXT).size(13.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .small_button(RichText::new("Limpiar").color(MUTED).size(11.0))
                .clicked()
            {
                action = UpNextAction::Clear;
            }
        });
    });
    ui.add_space(6.0);

    if items.is_empty() {
        ui.label(
            RichText::new("Sin elementos en cola.")
                .color(MUTED)
                .size(12.0),
        );
        return action;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        for (idx, it) in items.iter().enumerate() {
            egui::Frame {
                fill: *SURFACE,
                inner_margin: egui::Margin::symmetric(8.0, 6.0),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .button(RichText::new("▶").color(ACCENT).size(12.0))
                        .on_hover_text("Reproducir ahora")
                        .clicked()
                    {
                        action = UpNextAction::PlayNow(idx);
                    }
                    ui.label(RichText::new(&it.title).color(TEXT).size(12.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .small_button(RichText::new("✕").color(MUTED).size(10.0))
                            .on_hover_text("Quitar")
                            .clicked()
                        {
                            action = UpNextAction::Dequeue(idx);
                        }
                    });
                });
            });
            ui.add_space(2.0);
        }
    });

    action
}
