//! playlist_panel.rs - Playlist with contextual menu per item.

use crate::i18n::{tr, Language};
use crate::playlist::{Playlist, Track};
use crate::ui::context_menu::{playlist_item_context_menu, ContextAction};
use crate::ui::theme::*;
use egui::{Color32, RichText, Ui};

pub fn draw_playlist(
    ui: &mut Ui,
    playlist: &Playlist,
    lang: Language,
) -> (Option<usize>, ContextAction) {
    let mut play_idx: Option<usize> = None;
    let mut ctx_action: ContextAction = ContextAction::None;
    let total = playlist.tracks.len();

    egui::Frame {
        fill: *SURFACE2,
        inner_margin: egui::Margin::symmetric(0.0, 0.0),
        ..Default::default()
    }
    .show(ui, |ui| {
        ui.vertical(|ui| {
            egui::Frame {
                fill: *SURFACE,
                inner_margin: egui::Margin::symmetric(12.0, 8.0),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(tr(lang, "panel.playlist_title"))
                            .color(TEXT)
                            .size(13.0),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{} {}", total, tr(lang, "panel.files_count")))
                                .color(MUTED)
                                .size(11.0),
                        );
                    });
                });
            });

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (i, track) in playlist.tracks.iter().enumerate() {
                        let is_current = playlist.current == Some(i);
                        let resp = draw_track_row(ui, track, i, is_current);

                        if resp.double_clicked() {
                            play_idx = Some(i);
                        }

                        let ca = playlist_item_context_menu(
                            &resp,
                            i,
                            &track.path,
                            is_current,
                            i == 0,
                            i == total.saturating_sub(1),
                            lang,
                        );
                        if ca != ContextAction::None {
                            ctx_action = ca;
                        }
                    }
                    if total == 0 {
                        ui.add_space(30.0);
                        ui.centered_and_justified(|ui| {
                            ui.label(
                                RichText::new(tr(lang, "panel.playlist_empty"))
                                    .color(MUTED)
                                    .size(12.0),
                            );
                        });
                    }
                });
        });
    });

    (play_idx, ctx_action)
}

fn draw_track_row(ui: &mut Ui, track: &Track, index: usize, is_current: bool) -> egui::Response {
    let fill = if is_current {
        Color32::from_rgb(35, 50, 70)
    } else if index % 2 == 0 {
        *SURFACE2
    } else {
        *SURFACE
    };

    egui::Frame {
        fill,
        inner_margin: egui::Margin::symmetric(12.0, 6.0),
        ..Default::default()
    }
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            if is_current {
                ui.label(RichText::new(">").size(10.0).color(ACCENT));
            } else {
                ui.label(
                    RichText::new(format!("{}", index + 1))
                        .size(10.0)
                        .color(MUTED),
                );
            }
            ui.add_space(4.0);
            let color = if is_current { ACCENT } else { TEXT };
            ui.label(RichText::new(&track.title).size(13.0).color(color));
        });
    })
    .response
}
