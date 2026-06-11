//! controls.rs - Bottom bar with premium aesthetics and perfect alignment.

use crate::ab_repeat::AbRepeat;
use crate::config::RepeatMode;
use crate::player::{Player, PlayerState};
use crate::theme_manager::ThemeColors;
use crate::thumbnail::ThumbnailCache;
use egui::{Color32, Rect, RichText, Sense, Ui};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlsAction {
    None,
    ToggleRepeat,
    ToggleShuffle,
    Prev,
    Next,
}

pub fn draw_controls(
    ui: &mut Ui,
    state: &PlayerState,
    player: &Player,
    thumbs: &ThumbnailCache,
    ab: &AbRepeat,
    repeat: &RepeatMode,
    shuffle: bool,
    audio_delay: f64,
    sub_delay: f64,
    theme: &ThemeColors,
) -> ControlsAction {
    let fill = theme.surface_color();
    let border_color = theme.surface2_color();

    let mut action = ControlsAction::None;

    // Subtle top margin (1px separator line)
    let max_rect = ui.max_rect();
    ui.painter().line_segment(
        [max_rect.left_top(), max_rect.right_top()],
        egui::Stroke::new(1.0, border_color),
    );

    egui::Frame::none()
        .fill(fill)
        .inner_margin(egui::Margin::symmetric(24.0, 14.0))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                // Progress bar (Seekbar)
                draw_seekbar(ui, state, player, thumbs, ab, theme);
                ui.add_space(10.0);

                // Row of buttons and controls
                action = draw_buttons_row(
                    ui,
                    state,
                    player,
                    repeat,
                    shuffle,
                    audio_delay,
                    sub_delay,
                    theme,
                );
            });
        });

    action
}

fn draw_seekbar(
    ui: &mut Ui,
    state: &PlayerState,
    player: &Player,
    thumbs: &ThumbnailCache,
    ab: &AbRepeat,
    theme: &ThemeColors,
) {
    let ratio = state.progress_ratio();

    // Responsive height: thinner at rest, thicker on hover
    let desired = egui::vec2(ui.available_width(), 12.0);
    let (resp, painter) = ui.allocate_painter(desired, Sense::click_and_drag());
    let rect = resp.rect;

    let is_hovered = resp.hovered() || resp.dragged() || ui.rect_contains_pointer(rect);
    let track_h = if is_hovered { 6.0 } else { 4.0 };
    let track_rect = Rect::from_center_size(rect.center(), egui::vec2(rect.width(), track_h));

    // seekbar background
    painter.rect_filled(track_rect, 3.0, theme.surface2_color());

    // Region A-B if configured
    if let (Some(a), Some(b)) = (ab.a, ab.b) {
        if state.duration > 0.0 {
            let xa =
                track_rect.min.x + track_rect.width() * (a / state.duration).clamp(0.0, 1.0) as f32;
            let xb =
                track_rect.min.x + track_rect.width() * (b / state.duration).clamp(0.0, 1.0) as f32;
            painter.rect_filled(
                Rect::from_x_y_ranges(xa..=xb, track_rect.y_range()),
                0.0,
                theme.accent_color().linear_multiply(0.35),
            );
        }
    }

    // Part completed (progress)
    let fill_w = track_rect.width() * ratio;
    let filled_rect = Rect::from_min_size(track_rect.min, egui::vec2(fill_w, track_rect.height()));
    painter.rect_filled(filled_rect, 3.0, theme.accent_color());

    // Circular indicator (Thumb) - only visible if interacted
    let thumb_x = track_rect.min.x + fill_w;
    if is_hovered {
        painter.circle_filled(
            egui::pos2(thumb_x, track_rect.center().y),
            6.0,
            theme.text_color(),
        );
    }

    // Click/drag interaction to search position (Seek)
    if resp.clicked() || resp.dragged() {
        if let Some(pointer) = resp.interact_pointer_pos() {
            let t = ((pointer.x - track_rect.min.x) / track_rect.width()).clamp(0.0, 1.0);
            let target_time = t as f64 * state.duration;
            let _ = player.seek_absolute(target_time);
        }
    }

    // Time tooltip and thumbnail on hover
    if let Some(pointer) = resp.hover_pos() {
        let t = ((pointer.x - track_rect.min.x) / track_rect.width()).clamp(0.0, 1.0);
        let time_at_cursor = t as f64 * state.duration;

        // floating time text
        painter.text(
            egui::pos2(
                pointer
                    .x
                    .clamp(track_rect.min.x + 24.0, track_rect.max.x - 24.0),
                track_rect.min.y - 12.0,
            ),
            egui::Align2::CENTER_CENTER,
            PlayerState::format_time(time_at_cursor),
            egui::FontId::proportional(11.0),
            theme.text_color().gamma_multiply(0.85),
        );

        // Thumbnail preview
        if thumbs.is_ready() && thumbs.nearest(time_at_cursor).is_some() {
            let tw = 128.0f32;
            let th = 72.0f32;
            let tx = (pointer.x - tw / 2.0).clamp(track_rect.min.x, track_rect.max.x - tw);
            let ty = track_rect.min.y - th - 18.0;
            let thumb_rect = Rect::from_min_size(egui::pos2(tx, ty), egui::vec2(tw, th));
            painter.rect_filled(thumb_rect, 4.0, theme.surface_color());
            painter.rect_stroke(
                thumb_rect,
                4.0,
                egui::Stroke::new(1.0, theme.surface2_color()),
            );
        }
    }
}

/// Draw a modern, borderless control button using native egui widgets for perfect alignment
fn draw_control_button(
    ui: &mut Ui,
    icon: &str,
    size: f32,
    color: Color32,
    tooltip: &str,
) -> egui::Response {
    let button = egui::Button::new(RichText::new(icon).size(size).color(color)).frame(false);

    let resp = ui.add_sized([32.0, 32.0], button);

    if !tooltip.is_empty() {
        resp.on_hover_text(tooltip)
    } else {
        resp
    }
}

/// Draw the highlighted circular Play/Pause button
fn draw_play_button(
    ui: &mut Ui,
    is_paused: bool,
    accent_color: Color32,
    bg_color: Color32,
) -> egui::Response {
    let icon = if is_paused { "▶" } else { "⏸" };

    // Smart contrast for the icon based on the brightness of the background color
    let lum =
        bg_color.r() as f32 * 0.299 + bg_color.g() as f32 * 0.587 + bg_color.b() as f32 * 0.114;
    let icon_color = if lum > 128.0 {
        Color32::BLACK
    } else {
        Color32::WHITE
    };

    // Move the Play triangle a little to the right with a space to visually center it
    let text = if is_paused {
        format!(" {icon}")
    } else {
        icon.to_string()
    };

    let button = egui::Button::new(RichText::new(text).size(16.0).color(icon_color).strong())
        .fill(accent_color)
        .rounding(20.0); // Makes a 40x40 button a perfect circle

    let resp = ui.add_sized([40.0, 40.0], button);
    resp.on_hover_text("Play / Pausa [Espacio]")
}

fn draw_buttons_row(
    ui: &mut Ui,
    state: &PlayerState,
    player: &Player,
    repeat: &RepeatMode,
    shuffle: bool,
    audio_delay: f64,
    sub_delay: f64,
    theme: &ThemeColors,
) -> ControlsAction {
    let text_color = theme.text_color();
    let muted_color = theme.text_color().gamma_multiply(0.55);
    let accent_color = theme.accent_color();
    let bg_color = theme.bg_color();

    let mut action = ControlsAction::None;

    // Single horizontal row to ensure that egui aligns the vertical center of everything
    ui.horizontal(|ui| {
        // --- LEFT GROUP: Playback modes ---
        let repeat_color = if *repeat != RepeatMode::None {
            accent_color
        } else {
            muted_color
        };
        if draw_control_button(
            ui,
            repeat.icon(),
            13.0,
            repeat_color,
            &format!("{} - click para cambiar", repeat.label()),
        )
        .clicked()
        {
            action = ControlsAction::ToggleRepeat;
        }

        let sh_color = if shuffle { accent_color } else { muted_color };
        if draw_control_button(ui, "🔀", 13.0, sh_color, "Aleatorio").clicked() {
            action = ControlsAction::ToggleShuffle;
        }

        // --- CENTER GROUP: Main controls aligned to the center ---
        let center_x = ui.max_rect().center().x;
        // We calculate the space needed for 5 buttons + gaps
        let playback_w = 40.0 + 4.0 * 32.0 + 4.0 * 6.0;
        let playback_start_x = center_x - playback_w / 2.0;
        let current_x = ui.cursor().min.x;

        if playback_start_x > current_x {
            ui.add_space(playback_start_x - current_x);
        }

        // Former
        if draw_control_button(ui, "⏮", 14.0, text_color, "Anterior [P]").clicked() {
            action = ControlsAction::Prev;
        }

        // Go back 10s
        if draw_control_button(ui, "⏪", 14.0, text_color, "-10s").clicked() {
            let _ = player.seek_relative(-10.0);
        }

        // Play/Pause (Hero Button)
        if draw_play_button(ui, state.paused, accent_color, bg_color).clicked() {
            let _ = player.toggle_pause();
        }

        // Fast forward 10s
        if draw_control_button(ui, "⏩", 14.0, text_color, "+10s").clicked() {
            let _ = player.seek_relative(10.0);
        }

        // Following
        if draw_control_button(ui, "⏭", 14.0, text_color, "Siguiente [N]").clicked() {
            action = ControlsAction::Next;
        }

        // --- RIGHT GROUP: Time, speed and volume ---
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 1.Volume Control
            let mut vol = state.volume as f32;
            let vol_icon = if state.muted {
                "🔇"
            } else if state.volume > 60 {
                "🔊"
            } else {
                "🔉"
            };

            ui.add_space(4.0);
            ui.label(
                RichText::new(format!("{}%", state.volume))
                    .color(muted_color)
                    .size(11.0),
            );

            // Ultra-clean volume slider
            ui.spacing_mut().slider_width = 80.0;
            if ui
                .add(egui::Slider::new(&mut vol, 0.0..=150.0).show_value(false))
                .changed()
            {
                let _ = player.set_volume(vol as i64);
            }

            if draw_control_button(ui, vol_icon, 13.0, text_color, "Silenciar [M]").clicked() {
                let _ = player.toggle_mute();
            }

            ui.add_space(12.0);

            // 2. Modern and borderless Speed ​​Selector
            egui::ComboBox::from_id_source("speed_combo")
                .selected_text(
                    RichText::new(format!("{:.2}×", state.speed))
                        .color(muted_color)
                        .size(12.0),
                )
                .width(64.0)
                .show_ui(ui, |ui| {
                    for &sp in &[0.25f64, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 3.0] {
                        if ui
                            .selectable_label((state.speed - sp).abs() < 0.01, format!("{sp:.2}×"))
                            .clicked()
                        {
                            let _ = player.set_speed(sp);
                        }
                    }
                });

            ui.add_space(12.0);

            // 3.Video Duration
            ui.label(
                RichText::new(format!(
                    "{} / {}",
                    PlayerState::format_time(state.position),
                    PlayerState::format_time(state.duration),
                ))
                .color(muted_color)
                .size(12.0),
            );

            // 4. Audio Delay and Subtitle Indicators
            if audio_delay.abs() > 0.01 {
                ui.label(
                    RichText::new(format!("A{:+.1}s", audio_delay))
                        .size(10.0)
                        .color(*crate::ui::theme::WARNING),
                )
                .on_hover_text("Retraso de audio activo");
            }
            if sub_delay.abs() > 0.01 {
                ui.label(
                    RichText::new(format!("S{:+.1}s", sub_delay))
                        .size(10.0)
                        .color(*crate::ui::theme::WARNING),
                )
                .on_hover_text("Retraso de subtítulos activo");
            }
        });
    });

    action
}
