//! image_controls_panel.rs — Brightness, contrast, saturation, hue, gamma, zoom, rotate, flip.

use crate::i18n::{tr, Language};
use crate::image_controls::ImageControls;
use crate::player::Player;
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct ImageControlsPanel;

impl ImageControlsPanel {
    /// Returns true if something changed.
    pub fn show(ui: &mut Ui, ic: &mut ImageControls, player: &Player, lang: Language) -> bool {
        let mut changed = false;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Imagen").color(TEXT).size(13.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if !ic.is_default() {
                        if ui
                            .small_button(RichText::new("Resetear").color(MUTED).size(11.0))
                            .clicked()
                        {
                            ic.reset();
                            changed = true;
                        }
                    }
                });
            });
            ui.add_space(8.0);

            // ── Color sliders ───────────────────── ─────────────────────
            changed |= color_slider(ui, "Brillo", &mut ic.brightness);
            changed |= color_slider(ui, "Contraste", &mut ic.contrast);
            changed |= color_slider(ui, "Saturación", &mut ic.saturation);
            changed |= color_slider(ui, "Matiz", &mut ic.hue);
            changed |= color_slider(ui, "Gamma", &mut ic.gamma);

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(6.0);

            // ── Zoom and pan ──────────────────────── ────────────────────────
            ui.label(RichText::new("Zoom / Pan").color(MUTED).size(11.0));
            ui.horizontal(|ui| {
                ui.label(RichText::new("Zoom").color(TEXT).size(12.0));
                let old = ic.zoom;
                ui.add_sized(
                    [120.0, 0.0],
                    egui::Slider::new(&mut ic.zoom, -0.5..=2.0).step_by(0.05),
                );
                if (ic.zoom - old).abs() > 0.001 {
                    changed = true;
                }
                if ui.small_button("1:1").clicked() {
                    ic.zoom = 0.0;
                    changed = true;
                }
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new("Pan X").color(TEXT).size(12.0));
                let old = ic.pan_x;
                ui.add_sized(
                    [100.0, 0.0],
                    egui::Slider::new(&mut ic.pan_x, -1.0..=1.0).step_by(0.01),
                );
                if (ic.pan_x - old).abs() > 0.001 {
                    changed = true;
                }
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new("Pan Y").color(TEXT).size(12.0));
                let old = ic.pan_y;
                ui.add_sized(
                    [100.0, 0.0],
                    egui::Slider::new(&mut ic.pan_y, -1.0..=1.0).step_by(0.01),
                );
                if (ic.pan_y - old).abs() > 0.001 {
                    changed = true;
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(6.0);

            // ── Rotation and flip ───────────────────── ──────────────────────
            ui.label(RichText::new("Rotación / Flip").color(MUTED).size(11.0));
            ui.horizontal(|ui| {
                if ui.button(RichText::new("↺ -90°").size(12.0)).clicked() {
                    ic.rotate_ccw();
                    changed = true;
                }
                if ui.button(RichText::new("↻ +90°").size(12.0)).clicked() {
                    ic.rotate_cw();
                    changed = true;
                }
                ui.label(
                    RichText::new(format!("{}°", ic.rotation))
                        .color(MUTED)
                        .size(12.0),
                );
            });
            ui.horizontal(|ui| {
                if ui.toggle_value(&mut ic.flip_h, "↔ Flip H").changed() {
                    changed = true;
                }
                if ui.toggle_value(&mut ic.flip_v, "↕ Flip V").changed() {
                    changed = true;
                }
            });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui
                    .toggle_value(&mut ic.deinterlace, "Deinterlace")
                    .changed()
                {
                    changed = true;
                }
                ui.label(
                    RichText::new("(para contenido de TV)")
                        .color(MUTED)
                        .size(10.0),
                );
            });
            ui.horizontal(|ui| {
                if ui
                    .toggle_value(&mut ic.integer_scaling, tr(lang, "img.integer_scaling"))
                    .changed()
                {
                    changed = true;
                }
                ui.label(
                    RichText::new(tr(lang, "img.integer_scaling_hint"))
                        .color(MUTED)
                        .size(10.0),
                );
            });
        });

        if changed {
            player.apply_image_controls(ic);
        }

        changed
    }
}

fn color_slider(ui: &mut Ui, label: &str, val: &mut i64) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.add_sized(
            [70.0, 14.0],
            egui::Label::new(RichText::new(label).color(TEXT).size(12.0)),
        );
        let old = *val;
        let mut v = *val as f32;
        if ui
            .add_sized(
                [130.0, 0.0],
                egui::Slider::new(&mut v, -100.0..=100.0).step_by(1.0),
            )
            .changed()
        {
            *val = v as i64;
        }
        if *val != old {
            changed = true;
        }
        if *val != 0 {
            if ui
                .small_button(RichText::new("0").color(MUTED).size(10.0))
                .clicked()
            {
                *val = 0;
                changed = true;
            }
        }
    });
    changed
}
