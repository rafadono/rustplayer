//! sleep_timer_panel.rs — Sleep timer.

use crate::sleep_timer::{SleepAction, SleepTimer};
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct SleepTimerPanel;

impl SleepTimerPanel {
    pub fn show(ui: &mut Ui, timer: &mut SleepTimer) {
        ui.vertical(|ui| {
            ui.label(RichText::new("Sleep Timer").color(TEXT).size(13.0));
            ui.add_space(8.0);

            if timer.enabled {
                // Active — show countdown
                if let Some(rem) = timer.remaining() {
                    let total = rem.as_secs();
                    let h = total / 3600;
                    let m = (total % 3600) / 60;
                    let s = total % 60;

                    let time_str = if h > 0 {
                        format!("{:02}:{:02}:{:02}", h, m, s)
                    } else {
                        format!("{:02}:{:02}", m, s)
                    };

                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new(format!("⏲ {}", time_str))
                                .size(28.0)
                                .color(ACCENT),
                        );
                    });
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(format!("Acción: {}", timer.action.label()))
                            .size(12.0)
                            .color(MUTED),
                    );
                }
                ui.add_space(8.0);
                if ui.button(RichText::new("Cancelar").color(DANGER)).clicked() {
                    timer.cancel();
                }
            } else {
                // Set timer
                let mut mins = timer.duration.as_secs() / 60;

                ui.label(RichText::new("Duración (minutos)").color(MUTED).size(11.0));
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut mins)
                            .clamp_range(1..=480)
                            .speed(1.0)
                            .suffix(" min"),
                    );
                    for &preset in &[5u64, 15, 30, 60, 90] {
                        if ui.small_button(format!("{}m", preset)).clicked() {
                            mins = preset;
                        }
                    }
                });
                timer.set_minutes(mins);

                ui.add_space(8.0);
                ui.label(RichText::new("Acción al terminar").color(MUTED).size(11.0));
                for action in &[SleepAction::Pause, SleepAction::Stop, SleepAction::Quit] {
                    ui.radio_value(&mut timer.action, action.clone(), action.label());
                }

                ui.add_space(10.0);
                if ui
                    .button(RichText::new("Iniciar timer").color(TEXT))
                    .clicked()
                {
                    timer.start();
                }
            }
        });
    }
}
