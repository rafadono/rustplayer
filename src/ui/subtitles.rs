//! subtitles.rs — Subtitle track selector and external file loading.

use crate::player::{MediaTrack, Player};
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct SubtitlePanel;

impl SubtitlePanel {
    pub fn show(ui: &mut Ui, tracks: &[MediaTrack], player: &Player) {
        ui.vertical(|ui| {
            ui.label(RichText::new("Subtítulos").color(TEXT).size(13.0));
            ui.add_space(6.0);

            if ui
                .selectable_label(
                    tracks.iter().all(|t| !t.selected),
                    RichText::new("Desactivar subtítulos").color(MUTED),
                )
                .clicked()
            {
                let _ = player.disable_subs();
            }

            ui.add_space(4.0);

            if tracks.is_empty() {
                ui.label(
                    RichText::new("Sin pistas de subtítulos")
                        .color(MUTED)
                        .size(12.0),
                );
            }

            for track in tracks {
                let label = if track.lang.is_empty() {
                    track.title.clone()
                } else {
                    format!("[{}] {}", track.lang.to_uppercase(), track.title)
                };

                if ui
                    .selectable_label(
                        track.selected,
                        RichText::new(label)
                            .color(if track.selected { ACCENT } else { TEXT })
                            .size(13.0),
                    )
                    .clicked()
                {
                    let _ = player.set_sub_track(track.id);
                }
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            if ui
                .button(
                    RichText::new("+ Cargar subtítulo externo...")
                        .size(12.0)
                        .color(MUTED),
                )
                .clicked()
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Subtítulos", &["srt", "ass", "ssa", "vtt", "sup", "pgs"])
                    .pick_file()
                {
                    let _ = player.add_sub_file(&path);
                }
            }
        });
    }
}
