//! menu.rs - Main top menu.

use crate::config::RepeatMode;
use crate::i18n::{tr, Language};
use crate::ui::theme::*;
use egui::{RichText, Ui};

#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    OpenFile,
    OpenMultiple,
    OpenUrl,
    OpenKaraoke,
    ImportM3u,
    ExportM3u,
    Quit,

    TogglePause,
    Stop,
    PrevTrack,
    NextTrack,
    SeekForward5,
    SeekForward60,
    SeekBackward5,
    SeekBackward60,
    FrameStep,
    FrameBackStep,
    CycleRepeat,
    ToggleShuffle,
    CycleAbRepeat,
    SpeedUp,
    SpeedDown,
    SpeedReset,
    SetSpeed(u32),

    ToggleMute,
    VolumeUp,
    VolumeDown,
    ToggleLoudnorm,
    ToggleEqualizer,
    ToggleAudioTracks,
    ToggleSync,

    Screenshot,
    TogglePip,
    SetAspect(crate::config::AspectRatio),
    RotateCw,
    RotateCcw,
    FlipH,
    FlipV,
    ToggleDeinterlace,
    ToggleIntegerScaling,
    ResetImage,
    ToggleImageControls,

    ToggleSubtitles,
    ToggleSubtitlesDownload,

    OpenTrimPanel,
    OpenConverterPanel,
    ToggleSleepTimer,
    OpenRemoteUrl,

    TogglePlaylist,
    ToggleHistory,
    ToggleBookmarks,
    ToggleNotes,
    ToggleChapters,
    ToggleMediaInfo,
    ToggleKaraoke,
    ToggleTheme,
    ToggleUpNext,
    TogglePerformance,
    ToggleLibrary,
    ToggleCodecDiagnostics,
    ToggleSettings,

    ReportBug,
    About,
    None,
}

pub struct MenuState<'a> {
    pub playlist_vis: bool,
    pub history_vis: bool,
    pub bookmarks_vis: bool,
    pub notes_vis: bool,
    pub chapters_vis: bool,
    pub info_vis: bool,
    pub theme_vis: bool,
    pub karaoke_vis: bool,
    pub subs_vis: bool,
    pub upnext_vis: bool,
    pub perf_vis: bool,
    pub library_vis: bool,
    pub codec_diag_vis: bool,
    pub shuffle: bool,
    pub loudnorm: bool,
    pub deinterlace: bool,
    pub integer_scaling: bool,
    pub repeat: &'a RepeatMode,
    pub current_speed: f64,
}

pub fn draw_menu(
    ui: &mut Ui,
    ms: &MenuState,
    lang: Language,
    ab: &mut crate::ab_repeat::AbRepeat,
    mpv: &libmpv2::Mpv,
) -> MenuAction {
    let mut action = MenuAction::None;

    // Apply local styles for a modern, polished look (flat rounded buttons/pills)
    let mut style = ui.style().as_ref().clone();

    // Comfortable button filling
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    // Space between menu items
    style.spacing.item_spacing = egui::vec2(6.0, 0.0);

    // Inactive buttons in the menu bar are transparent by default
    style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);

    // Hovered buttons have rounded surface2 bottom and smooth edges
    style.visuals.widgets.hovered.bg_fill = SURFACE2.gamma_multiply(0.9);
    style.visuals.widgets.hovered.weak_bg_fill = SURFACE2.gamma_multiply(0.9);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);

    // Active buttons have accent background
    style.visuals.widgets.active.bg_fill = ACCENT.gamma_multiply(0.85);
    style.visuals.widgets.active.weak_bg_fill = ACCENT.gamma_multiply(0.85);
    style.visuals.widgets.active.rounding = egui::Rounding::same(6.0);

    ui.set_style(style);

    egui::menu::bar(ui, |ui| {
        ui.menu_button(
            RichText::new(format!("📁  {}", tr(lang, "menu.file"))).color(TEXT),
            |ui| {
                btn(ui, tr(lang, "menu.open_file"), || {
                    action = MenuAction::OpenFile
                });
                btn(ui, tr(lang, "menu.open_multiple"), || {
                    action = MenuAction::OpenMultiple
                });
                btn(ui, tr(lang, "menu.open_url"), || {
                    action = MenuAction::OpenUrl
                });
                btn(ui, tr(lang, "menu.open_karaoke"), || {
                    action = MenuAction::OpenKaraoke
                });
                ui.separator();
                btn(ui, tr(lang, "menu.import_m3u"), || {
                    action = MenuAction::ImportM3u
                });
                btn(ui, tr(lang, "menu.export_m3u"), || {
                    action = MenuAction::ExportM3u
                });
                ui.separator();
                btn(ui, tr(lang, "menu.quit"), || action = MenuAction::Quit);
            },
        );

        ui.menu_button(
            RichText::new(format!("⏯  {}", tr(lang, "menu.playback"))).color(TEXT),
            |ui| {
                btn(ui, tr(lang, "menu.play_pause"), || {
                    action = MenuAction::TogglePause
                });
                btn(ui, tr(lang, "menu.stop"), || action = MenuAction::Stop);
                ui.separator();
                btn(ui, tr(lang, "menu.prev"), || action = MenuAction::PrevTrack);
                btn(ui, tr(lang, "menu.next"), || action = MenuAction::NextTrack);
                ui.separator();

                ui.menu_button(
                    RichText::new(tr(lang, "menu.jump")).size(12.0).color(TEXT),
                    |ui| {
                        btn(ui, tr(lang, "menu.seek_fwd_5"), || {
                            action = MenuAction::SeekForward5
                        });
                        btn(ui, tr(lang, "menu.seek_fwd_60"), || {
                            action = MenuAction::SeekForward60
                        });
                        ui.separator();
                        btn(ui, tr(lang, "menu.seek_back_5"), || {
                            action = MenuAction::SeekBackward5
                        });
                        btn(ui, tr(lang, "menu.seek_back_60"), || {
                            action = MenuAction::SeekBackward60
                        });
                    },
                );

                btn(ui, tr(lang, "menu.frame_fwd"), || {
                    action = MenuAction::FrameStep
                });
                btn(ui, tr(lang, "menu.frame_back"), || {
                    action = MenuAction::FrameBackStep
                });
                ui.separator();

                let repeat_label = format!(
                    "{}: {}",
                    tr(lang, "menu.repeat"),
                    ms.repeat.label_lang(lang)
                );
                btn(ui, &repeat_label, || action = MenuAction::CycleRepeat);

                let sh = if ms.shuffle { "[x]" } else { "[ ]" };
                btn(ui, &format!("{} {}", sh, tr(lang, "menu.shuffle")), || {
                    action = MenuAction::ToggleShuffle
                });
                btn(ui, tr(lang, "menu.ab_loop"), || {
                    action = MenuAction::CycleAbRepeat
                });
                ui.separator();

                ui.menu_button(
                    RichText::new(format!(
                        "{}: {:.2}x",
                        tr(lang, "menu.speed"),
                        ms.current_speed
                    ))
                    .size(12.0)
                    .color(TEXT),
                    |ui| {
                        for &(label, val) in &[
                            ("0.25x", 25u32),
                            ("0.50x", 50),
                            ("0.75x", 75),
                            ("1.00x [=]", 100),
                            ("1.25x", 125),
                            ("1.50x", 150),
                            ("2.00x", 200),
                            ("3.00x", 300),
                        ] {
                            let active = ((ms.current_speed * 100.0).round() as u32) == val;
                            let s = if active {
                                format!("[x] {}", label)
                            } else {
                                format!("[ ] {}", label)
                            };
                            btn(ui, &s, || action = MenuAction::SetSpeed(val));
                        }
                        ui.separator();
                        btn(ui, tr(lang, "menu.speed_up"), || {
                            action = MenuAction::SpeedUp
                        });
                        btn(ui, tr(lang, "menu.speed_down"), || {
                            action = MenuAction::SpeedDown
                        });
                        btn(ui, tr(lang, "menu.speed_reset"), || {
                            action = MenuAction::SpeedReset
                        });
                    },
                );
            },
        );

        ui.menu_button(
            RichText::new(format!("🔊  {}", tr(lang, "menu.audio"))).color(TEXT),
            |ui| {
                btn(ui, tr(lang, "menu.mute"), || {
                    action = MenuAction::ToggleMute
                });
                btn(ui, tr(lang, "menu.vol_up"), || {
                    action = MenuAction::VolumeUp
                });
                btn(ui, tr(lang, "menu.vol_down"), || {
                    action = MenuAction::VolumeDown
                });
                ui.separator();

                let ln = if ms.loudnorm {
                    format!("[x] {}", tr(lang, "menu.loudnorm"))
                } else {
                    format!("[ ] {}", tr(lang, "menu.loudnorm"))
                };
                btn(ui, &ln, || action = MenuAction::ToggleLoudnorm);
                ui.separator();

                btn(ui, tr(lang, "menu.audio_tracks"), || {
                    action = MenuAction::ToggleAudioTracks
                });
                btn(ui, tr(lang, "menu.equalizer"), || {
                    action = MenuAction::ToggleEqualizer
                });
                btn(ui, tr(lang, "menu.sync"), || {
                    action = MenuAction::ToggleSync
                });
            },
        );

        ui.menu_button(
            RichText::new(format!("📺  {}", tr(lang, "menu.video"))).color(TEXT),
            |ui| {
                btn(ui, tr(lang, "menu.screenshot"), || {
                    action = MenuAction::Screenshot
                });
                btn(ui, tr(lang, "menu.pip"), || action = MenuAction::TogglePip);
                ui.separator();

                ui.menu_button(
                    RichText::new(tr(lang, "menu.aspect"))
                        .size(12.0)
                        .color(TEXT),
                    |ui| {
                        for ratio in crate::config::AspectRatio::all() {
                            if ui.button(ratio.label()).clicked() {
                                action = MenuAction::SetAspect(ratio.clone());
                                ui.close_menu();
                            }
                        }
                    },
                );
                ui.separator();

                btn(ui, tr(lang, "menu.rotate_cw"), || {
                    action = MenuAction::RotateCw
                });
                btn(ui, tr(lang, "menu.rotate_ccw"), || {
                    action = MenuAction::RotateCcw
                });
                btn(ui, tr(lang, "menu.flip_h"), || action = MenuAction::FlipH);
                btn(ui, tr(lang, "menu.flip_v"), || action = MenuAction::FlipV);
                ui.separator();

                let deint = if ms.deinterlace {
                    format!("[x] {}", tr(lang, "menu.deinterlace"))
                } else {
                    format!("[ ] {}", tr(lang, "menu.deinterlace"))
                };
                btn(ui, &deint, || action = MenuAction::ToggleDeinterlace);
                let int_scale = if ms.integer_scaling {
                    format!("[x] {}", tr(lang, "menu.integer_scaling"))
                } else {
                    format!("[ ] {}", tr(lang, "menu.integer_scaling"))
                };
                btn(ui, &int_scale, || action = MenuAction::ToggleIntegerScaling);
                btn(ui, tr(lang, "menu.reset_image"), || {
                    action = MenuAction::ResetImage
                });
                ui.separator();

                btn(ui, tr(lang, "menu.image_controls"), || {
                    action = MenuAction::ToggleImageControls
                });
            },
        );

        ui.menu_button(
            RichText::new(format!("💬  {}", tr(lang, "menu.subtitles"))).color(TEXT),
            |ui| {
                toggle(ui, tr(lang, "menu.subtitle_tracks"), ms.subs_vis, || {
                    action = MenuAction::ToggleSubtitles
                });
                btn(ui, tr(lang, "menu.subtitle_download"), || {
                    action = MenuAction::ToggleSubtitlesDownload
                });
            },
        );

        ui.menu_button(
            RichText::new(format!("🔧  {}", tr(lang, "menu.tools"))).color(TEXT),
            |ui| {
                btn(ui, tr(lang, "menu.trim"), || {
                    action = MenuAction::OpenTrimPanel
                });
                btn(ui, tr(lang, "menu.convert"), || {
                    action = MenuAction::OpenConverterPanel
                });
                ui.separator();
                btn(ui, tr(lang, "menu.sleep"), || {
                    action = MenuAction::ToggleSleepTimer
                });
                btn(ui, tr(lang, "menu.remote"), || {
                    action = MenuAction::OpenRemoteUrl
                });
            },
        );

        ui.menu_button(
            RichText::new(format!("👁  {}", tr(lang, "menu.view"))).color(TEXT),
            |ui| {
                toggle(ui, tr(lang, "menu.playlist"), ms.playlist_vis, || {
                    action = MenuAction::TogglePlaylist
                });
                toggle(ui, tr(lang, "menu.history"), ms.history_vis, || {
                    action = MenuAction::ToggleHistory
                });
                toggle(ui, tr(lang, "menu.bookmarks"), ms.bookmarks_vis, || {
                    action = MenuAction::ToggleBookmarks
                });
                toggle(ui, tr(lang, "menu.notes"), ms.notes_vis, || {
                    action = MenuAction::ToggleNotes
                });
                toggle(ui, tr(lang, "menu.chapters"), ms.chapters_vis, || {
                    action = MenuAction::ToggleChapters
                });
                ui.separator();
                toggle(ui, tr(lang, "menu.media_info"), ms.info_vis, || {
                    action = MenuAction::ToggleMediaInfo
                });
                toggle(ui, tr(lang, "menu.karaoke"), ms.karaoke_vis, || {
                    action = MenuAction::ToggleKaraoke
                });
                toggle(ui, tr(lang, "menu.theme"), ms.theme_vis, || {
                    action = MenuAction::ToggleTheme
                });
                toggle(ui, tr(lang, "menu.up_next"), ms.upnext_vis, || {
                    action = MenuAction::ToggleUpNext
                });
                toggle(ui, tr(lang, "menu.performance"), ms.perf_vis, || {
                    action = MenuAction::TogglePerformance
                });
                toggle(ui, tr(lang, "menu.library"), ms.library_vis, || {
                    action = MenuAction::ToggleLibrary
                });
                toggle(ui, tr(lang, "menu.codec_diag"), ms.codec_diag_vis, || {
                    action = MenuAction::ToggleCodecDiagnostics
                });
                ui.separator();
                btn(ui, tr(lang, "menu.settings"), || {
                    action = MenuAction::ToggleSettings
                });
            },
        );

        ui.menu_button(
            RichText::new(format!("❓  {}", tr(lang, "menu.help"))).color(MUTED),
            |ui| {
                btn(ui, tr(lang, "menu.report_bug"), || {
                    action = MenuAction::ReportBug
                });
                btn(ui, tr(lang, "menu.about"), || action = MenuAction::About);
            },
        );

        // Draw state A-B if configured (marked point A) as a fancy pill
        if ab.a.is_some() {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let tag_bg = SURFACE2.gamma_multiply(0.60);
                let text_color = if ab.active { ACCENT2 } else { WARNING };

                egui::Frame::none()
                    .fill(tag_bg)
                    .rounding(12.0)
                    .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            ui.label(RichText::new("🔁").size(10.0));
                            ui.label(
                                RichText::new(ab.label())
                                    .color(text_color)
                                    .size(11.0)
                                    .strong(),
                            );

                            // Clear loop button with interactive hover
                            let close_btn =
                                egui::Button::new(RichText::new("✕").size(10.0).color(MUTED))
                                    .frame(false);
                            if ui.add(close_btn).on_hover_text("Limpiar A-B").clicked() {
                                ab.clear(mpv);
                            }
                        });
                    });
            });
        }
    });

    action
}

fn btn(ui: &mut Ui, label: &str, mut f: impl FnMut()) {
    if ui.button(RichText::new(label).size(12.0)).clicked() {
        f();
        ui.close_menu();
    }
}

fn toggle(ui: &mut Ui, label: &str, active: bool, f: impl FnMut()) {
    let s = if active {
        format!("[x] {}", label)
    } else {
        format!("[ ] {}", label)
    };
    btn(ui, &s, f);
}
