use crate::player::PlayerState;
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub fn show(ui: &mut Ui, state: &PlayerState, overlay_enabled: &mut bool) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Métricas de reproducción")
                .color(TEXT)
                .size(13.0),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .checkbox(overlay_enabled, "Overlay")
                .on_hover_text("Mostrar overlay sobre el video")
                .changed()
            {
                changed = true;
            }
        });
    });
    ui.separator();

    row(ui, "FPS renderizado", &format!("{:.2}", state.render_fps));
    row(ui, "Dropped frames", &format!("{}", state.dropped_frames));
    row(
        ui,
        "HW decode",
        if state.hwdec_active { "activo" } else { "off" },
    );
    row(ui, "Buffer (s)", &format!("{:.2}", state.buffer_seconds));

    changed
}

fn row(ui: &mut Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(key).color(MUTED).size(11.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(value).color(TEXT).size(12.0));
        });
    });
}
