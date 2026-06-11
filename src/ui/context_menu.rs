//! context_menu.rs - Right-click context menus.

use crate::config::AspectRatio;
use crate::i18n::{tr, Language};
use crate::player::PlayerState;
use crate::ui::theme::*;
use egui::{RichText, Ui};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum ContextAction {
    TogglePause,
    Stop,
    SeekForward5,
    SeekForward60,
    SeekBackward5,
    SeekBackward60,
    PrevTrack,
    NextTrack,
    ToggleMute,
    VolumeUp,
    VolumeDown,
    ToggleLoudnorm,
    Screenshot,
    TogglePip,
    SetAspect(AspectRatio),
    RotateCw,
    RotateCcw,
    FlipH,
    FlipV,
    ToggleDeinterlace,
    ToggleIntegerScaling,
    ResetImage,
    PlayIndex(usize),
    RemoveFromPlaylist(usize),
    MoveUp(usize),
    MoveDown(usize),
    AddBookmarkAt(usize),
    EnqueueNext(usize),
    EnqueueLast(usize),
    OpenInExplorer(PathBuf),
    CopyPath(PathBuf),
    ClearPlaylist,
    OpenFromHistory(PathBuf),
    RemoveFromHistory(PathBuf),
    ClearHistory,
    SeekToBookmark(f64),
    DeleteBookmark(String, u64),
    RenameBookmark(String, u64, String),
    TogglePanel(Panel),
    OpenTrimPanel,
    OpenConverterPanel,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Panel {
    Equalizer,
    ImageControls,
    Sync,
    MediaInfo,
    Subtitles,
    SubtitlesDownload,
    AudioTracks,
}

pub fn video_context_menu(
    response: &egui::Response,
    state: &PlayerState,
    loudnorm: bool,
    deint: bool,
    integer_scaling: bool,
    lang: Language,
) -> ContextAction {
    let mut action = ContextAction::None;

    response.context_menu(|ui| {
        ui.set_min_width(220.0);

        let play_label = if state.paused {
            tr(lang, "ctx.play")
        } else {
            tr(lang, "ctx.pause")
        };
        menu_item(ui, play_label, || action = ContextAction::TogglePause);
        menu_item(ui, tr(lang, "ctx.stop"), || action = ContextAction::Stop);
        ui.separator();

        menu_item(ui, tr(lang, "ctx.prev_track"), || {
            action = ContextAction::PrevTrack
        });
        menu_item(ui, tr(lang, "ctx.next_track"), || {
            action = ContextAction::NextTrack
        });
        ui.separator();

        ui.menu_button(
            RichText::new(tr(lang, "ctx.jump")).size(12.0).color(TEXT),
            |ui| {
                menu_item(ui, tr(lang, "menu.seek_fwd_5"), || {
                    action = ContextAction::SeekForward5
                });
                menu_item(ui, tr(lang, "menu.seek_fwd_60"), || {
                    action = ContextAction::SeekForward60
                });
                ui.separator();
                menu_item(ui, tr(lang, "menu.seek_back_5"), || {
                    action = ContextAction::SeekBackward5
                });
                menu_item(ui, tr(lang, "menu.seek_back_60"), || {
                    action = ContextAction::SeekBackward60
                });
            },
        );
        ui.separator();

        let mute_label = if state.muted {
            tr(lang, "ctx.unmute")
        } else {
            tr(lang, "menu.mute")
        };
        menu_item(ui, mute_label, || action = ContextAction::ToggleMute);
        menu_item(ui, tr(lang, "menu.vol_up"), || {
            action = ContextAction::VolumeUp
        });
        menu_item(ui, tr(lang, "menu.vol_down"), || {
            action = ContextAction::VolumeDown
        });
        let ln_label = if loudnorm {
            format!("[x] {}", tr(lang, "menu.loudnorm"))
        } else {
            tr(lang, "menu.loudnorm").to_string()
        };
        menu_item(ui, &ln_label, || action = ContextAction::ToggleLoudnorm);
        ui.separator();

        ui.menu_button(
            RichText::new(tr(lang, "ctx.audio_and_subs"))
                .size(12.0)
                .color(TEXT),
            |ui| {
                menu_item(ui, tr(lang, "menu.audio_tracks"), || {
                    action = ContextAction::TogglePanel(Panel::AudioTracks)
                });
                menu_item(ui, tr(lang, "menu.subtitle_tracks"), || {
                    action = ContextAction::TogglePanel(Panel::Subtitles)
                });
                menu_item(ui, tr(lang, "menu.subtitle_download"), || {
                    action = ContextAction::TogglePanel(Panel::SubtitlesDownload)
                });
                menu_item(ui, tr(lang, "menu.sync"), || {
                    action = ContextAction::TogglePanel(Panel::Sync)
                });
                menu_item(ui, tr(lang, "menu.equalizer"), || {
                    action = ContextAction::TogglePanel(Panel::Equalizer)
                });
            },
        );
        ui.separator();

        menu_item(ui, tr(lang, "menu.screenshot"), || {
            action = ContextAction::Screenshot
        });
        menu_item(ui, tr(lang, "menu.pip"), || {
            action = ContextAction::TogglePip
        });

        ui.menu_button(
            RichText::new(tr(lang, "menu.aspect"))
                .size(12.0)
                .color(TEXT),
            |ui| {
                for ratio in AspectRatio::all() {
                    if ui.button(ratio.label()).clicked() {
                        action = ContextAction::SetAspect(ratio.clone());
                        ui.close_menu();
                    }
                }
            },
        );

        ui.menu_button(
            RichText::new(tr(lang, "ctx.image_and_video"))
                .size(12.0)
                .color(TEXT),
            |ui| {
                menu_item(ui, tr(lang, "menu.image_controls"), || {
                    action = ContextAction::TogglePanel(Panel::ImageControls)
                });
                ui.separator();
                menu_item(ui, tr(lang, "menu.rotate_cw"), || {
                    action = ContextAction::RotateCw
                });
                menu_item(ui, tr(lang, "menu.rotate_ccw"), || {
                    action = ContextAction::RotateCcw
                });
                menu_item(ui, tr(lang, "menu.flip_h"), || {
                    action = ContextAction::FlipH
                });
                menu_item(ui, tr(lang, "menu.flip_v"), || {
                    action = ContextAction::FlipV
                });
                ui.separator();
                let d_label = if deint {
                    format!("[x] {}", tr(lang, "menu.deinterlace"))
                } else {
                    tr(lang, "menu.deinterlace").to_string()
                };
                menu_item(ui, &d_label, || action = ContextAction::ToggleDeinterlace);
                let i_label = if integer_scaling {
                    format!("[x] {}", tr(lang, "menu.integer_scaling"))
                } else {
                    tr(lang, "menu.integer_scaling").to_string()
                };
                menu_item(ui, &i_label, || {
                    action = ContextAction::ToggleIntegerScaling
                });
                ui.separator();
                menu_item(ui, tr(lang, "menu.reset_image"), || {
                    action = ContextAction::ResetImage
                });
            },
        );
        ui.separator();

        menu_item(ui, tr(lang, "menu.trim"), || {
            action = ContextAction::OpenTrimPanel
        });
        menu_item(ui, tr(lang, "menu.convert"), || {
            action = ContextAction::OpenConverterPanel
        });
        ui.separator();

        menu_item(ui, tr(lang, "menu.media_info"), || {
            action = ContextAction::TogglePanel(Panel::MediaInfo)
        });
    });

    action
}

pub fn playlist_item_context_menu(
    response: &egui::Response,
    index: usize,
    path: &PathBuf,
    is_current: bool,
    is_first: bool,
    is_last: bool,
    lang: Language,
) -> ContextAction {
    let mut action = ContextAction::None;

    response.context_menu(|ui| {
        ui.set_min_width(200.0);

        let play_label = if is_current {
            tr(lang, "ctx.playing")
        } else {
            tr(lang, "ctx.play_now")
        };
        menu_item_enabled(ui, play_label, !is_current, || {
            action = ContextAction::PlayIndex(index)
        });
        ui.separator();

        menu_item_enabled(ui, tr(lang, "ctx.move_up"), !is_first, || {
            action = ContextAction::MoveUp(index)
        });
        menu_item_enabled(ui, tr(lang, "ctx.move_down"), !is_last, || {
            action = ContextAction::MoveDown(index)
        });
        ui.separator();

        menu_item(ui, tr(lang, "ctx.add_bookmark"), || {
            action = ContextAction::AddBookmarkAt(index)
        });
        menu_item(ui, tr(lang, "ctx.enqueue_next"), || {
            action = ContextAction::EnqueueNext(index)
        });
        menu_item(ui, tr(lang, "ctx.enqueue_last"), || {
            action = ContextAction::EnqueueLast(index)
        });
        ui.separator();

        let path_clone = path.clone();
        menu_item(ui, tr(lang, "ctx.copy_path"), || {
            action = ContextAction::CopyPath(path_clone.clone())
        });

        let path_clone2 = path.clone();
        menu_item(ui, tr(lang, "ctx.show_in_explorer"), || {
            action = ContextAction::OpenInExplorer(path_clone2.clone())
        });
        ui.separator();

        menu_item(ui, tr(lang, "ctx.remove_from_list"), || {
            action = ContextAction::RemoveFromPlaylist(index)
        });
        ui.separator();
        menu_item_danger(ui, tr(lang, "ctx.clear_list"), || {
            action = ContextAction::ClearPlaylist
        });
    });

    action
}

pub fn history_item_context_menu(
    response: &egui::Response,
    path: &PathBuf,
    lang: Language,
) -> ContextAction {
    let mut action = ContextAction::None;

    response.context_menu(|ui| {
        ui.set_min_width(200.0);

        let p = path.clone();
        menu_item(ui, tr(lang, "ctx.open"), || {
            action = ContextAction::OpenFromHistory(p.clone())
        });

        let p2 = path.clone();
        menu_item(ui, tr(lang, "ctx.show_in_explorer"), || {
            action = ContextAction::OpenInExplorer(p2.clone())
        });

        let p3 = path.clone();
        menu_item(ui, tr(lang, "ctx.copy_path"), || {
            action = ContextAction::CopyPath(p3.clone())
        });
        ui.separator();

        let p4 = path.clone();
        menu_item(ui, tr(lang, "ctx.remove_from_history"), || {
            action = ContextAction::RemoveFromHistory(p4.clone())
        });
        ui.separator();

        menu_item_danger(ui, tr(lang, "ctx.clear_history"), || {
            action = ContextAction::ClearHistory
        });
    });

    action
}

pub fn bookmark_context_menu(
    response: &egui::Response,
    file_key: &str,
    bm_id: u64,
    bm_pos: f64,
    bm_label: &str,
    lang: Language,
) -> ContextAction {
    let mut action = ContextAction::None;

    response.context_menu(|ui| {
        ui.set_min_width(180.0);

        menu_item(ui, tr(lang, "ctx.go_to_this_bookmark"), || {
            action = ContextAction::SeekToBookmark(bm_pos)
        });
        ui.separator();
        let k = file_key.to_string();
        let new_label = format!("{} (copia)", bm_label);
        menu_item(ui, tr(lang, "ctx.rename"), || {
            action = ContextAction::RenameBookmark(k.clone(), bm_id, new_label.clone())
        });
        let k2 = file_key.to_string();
        menu_item_danger(ui, tr(lang, "ctx.delete_bookmark"), || {
            action = ContextAction::DeleteBookmark(k2.clone(), bm_id)
        });
    });

    action
}

fn menu_item(ui: &mut Ui, label: &str, mut f: impl FnMut()) {
    if ui
        .button(RichText::new(label).size(12.0).color(TEXT))
        .clicked()
    {
        f();
        ui.close_menu();
    }
}

fn menu_item_enabled(ui: &mut Ui, label: &str, enabled: bool, mut f: impl FnMut()) {
    let color = if enabled { TEXT } else { MUTED };
    if ui
        .add_enabled(
            enabled,
            egui::Button::new(RichText::new(label).size(12.0).color(color)),
        )
        .clicked()
    {
        f();
        ui.close_menu();
    }
}

fn menu_item_danger(ui: &mut Ui, label: &str, mut f: impl FnMut()) {
    if ui
        .button(RichText::new(label).size(12.0).color(DANGER))
        .clicked()
    {
        f();
        ui.close_menu();
    }
}
