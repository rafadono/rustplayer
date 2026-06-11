//! sync_panel.rs — Synchronization: audio delay, subtitles and secondary track.

use crate::player::{MediaTrack, Player};
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct SyncPanel;

impl SyncPanel {
    /// return (audio_delay_changed, sub_delay_changed)
    pub fn show(
        ui: &mut Ui,
        audio_delay: &mut f64,
        sub_delay: &mut f64,
        sub_tracks: &[MediaTrack],
        player: &Player,
    ) -> (bool, bool) {
        let mut audio_ch = false;
        let mut sub_ch = false;

        ui.vertical(|ui| {
            ui.label(RichText::new("Sincronización").color(TEXT).size(13.0));
            ui.add_space(8.0);

            // ──Audio delay ────────────────────── ───────────────────────
            ui.label(
                RichText::new("Retraso de audio (seg)")
                    .color(MUTED)
                    .size(11.0),
            );
            ui.horizontal(|ui| {
                let old = *audio_delay;
                ui.add_sized(
                    [130.0, 0.0],
                    egui::Slider::new(audio_delay, -5.0..=5.0)
                        .step_by(0.05)
                        .suffix("s"),
                );
                if (*audio_delay - old).abs() > 0.001 {
                    let _ = player.set_audio_delay(*audio_delay);
                    audio_ch = true;
                }
                if ui.small_button("0").clicked() {
                    *audio_delay = 0.0;
                    let _ = player.set_audio_delay(0.0);
                    audio_ch = true;
                }
            });
            ui.label(
                RichText::new("+ = audio adelantado · - = audio retrasado")
                    .color(MUTED)
                    .size(10.0),
            );

            ui.add_space(12.0);

            // ── Delayed subtitles ──────────────────── ─────────────────────
            ui.label(
                RichText::new("Retraso de subtítulos (seg)")
                    .color(MUTED)
                    .size(11.0),
            );
            ui.horizontal(|ui| {
                let old = *sub_delay;
                ui.add_sized(
                    [130.0, 0.0],
                    egui::Slider::new(sub_delay, -10.0..=10.0)
                        .step_by(0.05)
                        .suffix("s"),
                );
                if (*sub_delay - old).abs() > 0.001 {
                    let _ = player.set_sub_delay(*sub_delay);
                    sub_ch = true;
                }
                if ui.small_button("0").clicked() {
                    *sub_delay = 0.0;
                    let _ = player.set_sub_delay(0.0);
                    sub_ch = true;
                }
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(6.0);

            // ── Second subtitle track ──────────────────────────────
            ui.label(
                RichText::new("Segunda pista de subtítulos")
                    .color(MUTED)
                    .size(11.0),
            );
            ui.label(
                RichText::new("(para ver dos idiomas simultáneamente)")
                    .color(MUTED)
                    .size(10.0),
            );
            ui.add_space(4.0);

            if sub_tracks.is_empty() {
                ui.label(
                    RichText::new("Sin pistas disponibles.")
                        .color(MUTED)
                        .size(12.0),
                );
            } else {
                if ui
                    .selectable_label(
                        false,
                        RichText::new("Desactivar segunda pista")
                            .color(MUTED)
                            .size(12.0),
                    )
                    .clicked()
                {
                    let _ = player.disable_second_subs();
                }
                ui.add_space(2.0);
                for track in sub_tracks {
                    let label = if track.lang.is_empty() {
                        track.title.clone()
                    } else {
                        format!("[{}] {}", track.lang.to_uppercase(), track.title)
                    };
                    if ui
                        .selectable_label(false, RichText::new(label).color(TEXT).size(12.0))
                        .clicked()
                    {
                        let _ = player.set_second_sub_track(track.id);
                    }
                }
            }
        });

        (audio_ch, sub_ch)
    }
}
