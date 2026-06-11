//! equalizer_panel.rs - PEQ professional (6 filters).

use crate::equalizer::{EqFilterType, Equalizer};
use crate::player::Player;
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct EqualizerPanel;

impl EqualizerPanel {
    pub fn show(ui: &mut Ui, eq: &mut Equalizer, player: &Player, loudnorm: bool) -> bool {
        let mut changed = false;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("EQ Parametrico v1").color(TEXT).size(13.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.toggle_value(&mut eq.enabled, "Activar").changed() {
                        changed = true;
                    }
                });
            });

            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                for (label, preset) in [
                    ("Flat", Equalizer::preset_flat()),
                    ("Bass", Equalizer::preset_bass_boost()),
                    ("Vocal", Equalizer::preset_vocal()),
                    ("Cinema", Equalizer::preset_cinema()),
                    ("Rock", Equalizer::preset_rock()),
                ] {
                    if ui.small_button(label).clicked() {
                        *eq = preset;
                        changed = true;
                    }
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(RichText::new("Preamp (dB)").color(MUTED).size(11.0));
                if ui
                    .add_sized(
                        [180.0, 0.0],
                        egui::Slider::new(&mut eq.preamp_db, -18.0..=18.0).step_by(0.1),
                    )
                    .changed()
                {
                    changed = true;
                    eq.enabled = true;
                }
                if ui
                    .checkbox(&mut eq.anti_clipping, "Anti-clipping")
                    .changed()
                {
                    changed = true;
                    eq.enabled = true;
                }
            });

            ui.add_space(6.0);
            for (i, f) in eq.peq_filters.iter_mut().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("F{}", i + 1))
                                .color(ACCENT)
                                .size(11.0),
                        );
                        if ui.checkbox(&mut f.enabled, "ON").changed() {
                            changed = true;
                            eq.enabled = true;
                        }
                        egui::ComboBox::from_id_source(format!("peq_kind_{i}"))
                            .selected_text(f.kind.label())
                            .show_ui(ui, |ui| {
                                for kind in [
                                    EqFilterType::Peak,
                                    EqFilterType::LowShelf,
                                    EqFilterType::HighShelf,
                                    EqFilterType::HighPass,
                                    EqFilterType::LowPass,
                                ] {
                                    if ui.selectable_label(f.kind == kind, kind.label()).clicked() {
                                        f.kind = kind;
                                        changed = true;
                                        eq.enabled = true;
                                    }
                                }
                            });
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Freq").color(MUTED).size(10.0));
                        if ui
                            .add(
                                egui::DragValue::new(&mut f.freq_hz)
                                    .speed(1.0)
                                    .clamp_range(20.0..=20_000.0)
                                    .suffix(" Hz"),
                            )
                            .changed()
                        {
                            changed = true;
                            eq.enabled = true;
                        }

                        let gain_enabled =
                            !matches!(f.kind, EqFilterType::HighPass | EqFilterType::LowPass);
                        ui.add_enabled_ui(gain_enabled, |ui| {
                            ui.label(RichText::new("Gain").color(MUTED).size(10.0));
                            if ui
                                .add_sized(
                                    [130.0, 0.0],
                                    egui::Slider::new(&mut f.gain_db, -24.0..=24.0).step_by(0.1),
                                )
                                .changed()
                            {
                                changed = true;
                                eq.enabled = true;
                            }
                        });

                        ui.label(RichText::new("Q").color(MUTED).size(10.0));
                        if ui
                            .add_sized(
                                [100.0, 0.0],
                                egui::Slider::new(&mut f.q, 0.1..=10.0).logarithmic(true),
                            )
                            .changed()
                        {
                            changed = true;
                            eq.enabled = true;
                        }
                    });
                });
                ui.add_space(4.0);
            }

            ui.separator();
            ui.label(RichText::new("Presets de usuario").color(TEXT).size(11.0));
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut eq.preset_name_input)
                        .hint_text("Nombre preset")
                        .desired_width(160.0),
                );
                if ui.button("Guardar").clicked() {
                    let name = eq.preset_name_input.clone();
                    eq.save_user_preset(name);
                    changed = true;
                }
                if ui.button("Cargar").clicked() {
                    eq.load_selected_user_preset();
                    changed = true;
                }
                if ui.button("Eliminar").clicked() {
                    eq.delete_selected_user_preset();
                    changed = true;
                }
            });
            egui::ComboBox::from_id_source("peq_user_preset_select")
                .selected_text(
                    eq.selected_user_preset
                        .and_then(|i| eq.user_presets.get(i).map(|p| p.name.clone()))
                        .unwrap_or_else(|| "(ninguno)".to_string()),
                )
                .show_ui(ui, |ui| {
                    for (i, p) in eq.user_presets.iter().enumerate() {
                        if ui
                            .selectable_label(eq.selected_user_preset == Some(i), &p.name)
                            .clicked()
                        {
                            eq.selected_user_preset = Some(i);
                        }
                    }
                });
        });

        if changed {
            player.set_audio_filters(eq, loudnorm);
        }

        changed
    }
}
