//! bookmarks_panel.rs - Bookmarks with context menu.

use crate::bookmarks::{Bookmark, BookmarkStore};
use crate::i18n::{tr, Language};
use crate::player::PlayerState;
use crate::ui::context_menu::{bookmark_context_menu, ContextAction};
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct BookmarksPanel;

impl BookmarksPanel {
    pub fn show(
        ui: &mut Ui,
        store: &mut BookmarkStore,
        state: &PlayerState,
        lang: Language,
    ) -> (Option<f64>, ContextAction) {
        let mut seek_to: Option<f64> = None;
        let mut ctx_action: ContextAction = ContextAction::None;
        let file = state.current_file.as_deref();

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(tr(lang, "panel.bookmarks_title"))
                        .color(TEXT)
                        .size(13.0),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(f) = file {
                        if ui
                            .small_button(
                                RichText::new(tr(lang, "panel.add"))
                                    .color(ACCENT)
                                    .size(11.0),
                            )
                            .on_hover_text(tr(lang, "panel.add_bookmark_hint"))
                            .clicked()
                        {
                            let bm =
                                Bookmark::new(state.position, tr(lang, "panel.bookmarks_title"));
                            store.add(f, bm);
                            store.save();
                        }
                    }
                });
            });
            ui.add_space(6.0);

            let Some(file) = file else {
                ui.label(
                    RichText::new(tr(lang, "panel.no_active_file"))
                        .color(MUTED)
                        .size(12.0),
                );
                return;
            };

            let file_key = file.to_string_lossy().to_string();
            let marks: Vec<Bookmark> = store.get(file).to_vec();

            if marks.is_empty() {
                ui.label(
                    RichText::new(tr(lang, "panel.no_bookmarks_for_file"))
                        .color(MUTED)
                        .size(12.0),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(tr(lang, "panel.use_b_or_plus"))
                        .color(MUTED)
                        .size(10.0),
                );
                return;
            }

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (i, bm) in marks.iter().enumerate() {
                        egui::Frame {
                            fill: *SURFACE,
                            inner_margin: egui::Margin::symmetric(8.0, 5.0),
                            rounding: egui::Rounding::same(4.0),
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let time_str = format_time(bm.position);
                                if ui
                                    .button(RichText::new(&time_str).size(11.0).color(ACCENT))
                                    .on_hover_text(tr(lang, "panel.go_to_bookmark"))
                                    .clicked()
                                {
                                    seek_to = Some(bm.position);
                                }

                                ui.label(RichText::new(&bm.label).size(12.0).color(TEXT));

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui
                                            .small_button(
                                                RichText::new("x").color(MUTED).size(10.0),
                                            )
                                            .on_hover_text(tr(lang, "panel.delete_bookmark"))
                                            .clicked()
                                        {
                                            ctx_action = ContextAction::DeleteBookmark(
                                                file_key.clone(),
                                                bm.id,
                                            );
                                        }
                                    },
                                );
                            });
                        });

                        let id = egui::Id::new(("bm_row", i));
                        if let Some(resp) = ui.ctx().read_response(id) {
                            let ca = bookmark_context_menu(
                                &resp,
                                &file_key,
                                bm.id,
                                bm.position,
                                &bm.label,
                                lang,
                            );
                            if ca != ContextAction::None {
                                ctx_action = ca;
                            }
                        }

                        ui.add_space(2.0);
                    }
                });
        });

        (seek_to, ctx_action)
    }
}

fn format_time(secs: f64) -> String {
    let h = secs as u64 / 3600;
    let m = (secs as u64 % 3600) / 60;
    let s = secs as u64 % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}
