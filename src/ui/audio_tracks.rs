//! audio_tracks.rs — Audio track selector.

use crate::player::{MediaTrack, Player};
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct AudioTrackPanel;

impl AudioTrackPanel {
    pub fn show(ui: &mut Ui, tracks: &[MediaTrack], player: &Player) {
        ui.vertical(|ui| {
            ui.label(RichText::new("Pistas de audio").color(TEXT).size(13.0));
            ui.add_space(6.0);

            if tracks.is_empty() {
                ui.label(RichText::new("Sin pistas de audio").color(MUTED).size(12.0));
                return;
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
                    let _ = player.set_audio_track(track.id);
                }
            }
        });
    }
}
