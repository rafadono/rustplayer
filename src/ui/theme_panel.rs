//! theme_panel.rs - Theme picker and color editor.

use crate::theme_manager::{ThemeColors, ThemePreset};
use crate::ui::theme::*;
use egui::{Color32, RichText, Stroke, Ui};

pub struct ThemePanel;

impl ThemePanel {
    pub fn show(ui: &mut Ui, theme: &mut ThemeColors, ctx: &egui::Context) -> bool {
        let mut changed = false;

        ui.vertical(|ui| {
            ui.label(RichText::new("Tema").color(TEXT).size(13.0));
            ui.add_space(8.0);

            for preset in ThemePreset::all() {
                if preset == &ThemePreset::Custom {
                    continue;
                }
                let is_active = &theme.preset == preset;
                let preview = ThemeColors::from_preset(preset);

                ui.horizontal(|ui| {
                    color_dot(ui, preview.bg_color());
                    color_dot(ui, preview.surface_color());
                    color_dot(ui, preview.accent_color());

                    if ui
                        .selectable_label(
                            is_active,
                            RichText::new(preset.label())
                                .size(12.0)
                                .color(if is_active { ACCENT } else { TEXT }),
                        )
                        .clicked()
                        && !is_active
                    {
                        *theme = ThemeColors::from_preset(preset);
                        theme.apply(ctx);
                        changed = true;
                    }
                });
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);

            ui.label(RichText::new("Personalizado").color(MUTED).size(11.0));
            ui.label(
                RichText::new("Rueda de color + brillo")
                    .color(MUTED)
                    .size(10.0),
            );
            ui.add_space(4.0);

            let mut any_color_changed = false;
            any_color_changed |= color_wheel_row(ui, "Fondo", &mut theme.bg);
            any_color_changed |= color_wheel_row(ui, "Superficie", &mut theme.surface);
            any_color_changed |= color_wheel_row(ui, "Acento", &mut theme.accent);
            any_color_changed |= color_wheel_row(ui, "Texto", &mut theme.text);

            if any_color_changed {
                theme.preset = ThemePreset::Custom;
                theme.apply(ctx);
                changed = true;
            }
        });

        changed
    }
}

fn color_dot(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), 6.0, color);
    ui.painter().circle_stroke(
        rect.center(),
        6.0,
        egui::Stroke::new(0.5, Color32::from_rgb(80, 80, 80)),
    );
}

fn color_wheel_row(ui: &mut Ui, label: &str, rgb: &mut [u8; 3]) -> bool {
    let mut changed = false;
    let (mut h, mut s, mut v) = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);

    ui.group(|ui| {
        ui.label(RichText::new(label).color(TEXT).size(11.0));
        ui.add_space(4.0);

        let wheel_size = egui::vec2(120.0, 120.0);
        let (rect, response) = ui.allocate_exact_size(wheel_size, egui::Sense::click_and_drag());
        draw_color_wheel(ui, rect);

        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.5 - 2.0;

        if let Some(pos) = response.interact_pointer_pos() {
            if response.clicked() || response.dragged() {
                let dx = pos.x - center.x;
                let dy = pos.y - center.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= radius {
                    let mut hue = dy.atan2(dx).to_degrees();
                    if hue < 0.0 {
                        hue += 360.0;
                    }
                    h = hue / 360.0;
                    s = (dist / radius).clamp(0.0, 1.0);
                    changed = true;
                }
            }
        }

        let sel_angle = h * std::f32::consts::TAU;
        let sel_r = s * radius;
        let sel_pos = egui::pos2(
            center.x + sel_r * sel_angle.cos(),
            center.y + sel_r * sel_angle.sin(),
        );
        ui.painter()
            .circle_stroke(sel_pos, 5.0, Stroke::new(2.0, Color32::WHITE));
        ui.painter()
            .circle_stroke(sel_pos, 6.0, Stroke::new(1.0, Color32::BLACK));

        let mut new_v = v;
        if ui
            .add(egui::Slider::new(&mut new_v, 0.0..=1.0).text("Brillo"))
            .changed()
        {
            v = new_v;
            changed = true;
        }

        let preview = hsv_to_color32(h, s, v);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Vista previa").size(10.0).color(MUTED));
            color_dot(ui, preview);
        });
    });

    if changed {
        let c = hsv_to_color32(h, s, v);
        rgb[0] = c.r();
        rgb[1] = c.g();
        rgb[2] = c.b();
    }

    changed
}

fn draw_color_wheel(ui: &Ui, rect: egui::Rect) {
    let painter = ui.painter();
    let center = rect.center();
    let radius = rect.width().min(rect.height()) * 0.5 - 2.0;

    painter.circle_filled(center, radius + 1.0, Color32::from_rgb(28, 28, 35));

    let sat_steps = 18;
    let hue_steps = 120;
    let dot_radius = radius / sat_steps as f32 * 0.75;

    for si in 0..=sat_steps {
        let sat = si as f32 / sat_steps as f32;
        let r = sat * radius;

        if si == 0 {
            painter.circle_filled(center, dot_radius, Color32::WHITE);
            continue;
        }

        for hi in 0..hue_steps {
            let hue = hi as f32 / hue_steps as f32;
            let angle = hue * std::f32::consts::TAU;
            let pos = egui::pos2(center.x + r * angle.cos(), center.y + r * angle.sin());
            painter.circle_filled(pos, dot_radius, hsv_to_color32(hue, sat, 1.0));
        }
    }

    painter.circle_stroke(center, radius, Stroke::new(1.0, Color32::from_gray(180)));
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf.max(bf));
    let min = rf.min(gf.min(bf));
    let d = max - min;

    let h = if d == 0.0 {
        0.0
    } else if max == rf {
        (((gf - bf) / d) % 6.0) / 6.0
    } else if max == gf {
        (((bf - rf) / d) + 2.0) / 6.0
    } else {
        (((rf - gf) / d) + 4.0) / 6.0
    };

    let h = if h < 0.0 { h + 1.0 } else { h };
    let s = if max == 0.0 { 0.0 } else { d / max };
    let v = max;

    (h, s, v)
}

fn hsv_to_color32(h: f32, s: f32, v: f32) -> Color32 {
    let h6 = (h * 6.0).fract();
    let i = (h * 6.0).floor() as i32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - h6 * s);
    let t = v * (1.0 - (1.0 - h6) * s);

    let (r, g, b) = match i.rem_euclid(6) {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}
