//! pip.rs — Picture-in-Picture mode.
//!
//! Opens an always-on floating auxiliary window with the video.
//! In eframe this is achieved with secondary viewports.

use crate::ui::theme::*;
use egui::{Color32, RichText};

pub struct PipWindow {
    pub open: bool,
}

impl PipWindow {
    pub fn new() -> Self {
        Self { open: false }
    }

    ///Shows the PiP viewport if `open == true`.
    /// It must be called from `PlayerApp::update`, within the context of egui.
    pub fn show_if_open(&mut self, ctx: &egui::Context, title: &str) {
        if !self.open {
            return;
        }

        // eframe 0.27 allows secondary viewports
        let pip_id = egui::ViewportId::from_hash_of("pip_window");
        let pip_builder = egui::ViewportBuilder::default()
            .with_title(format!("PiP — {}", title))
            .with_inner_size([320.0, 200.0])
            .with_always_on_top()
            .with_decorations(true)
            .with_resizable(true);

        ctx.show_viewport_immediate(pip_id, pip_builder, |ctx, _class| {
            egui::CentralPanel::default()
                .frame(egui::Frame::none().fill(Color32::BLACK))
                .show(ctx, |ui| {
                    // Actual rendering of video in a second window requires a
                    // second RenderContext of mpv pointing to this viewport.
                    // For now we show a placeholder while it is implemented.
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("▶").size(32.0).color(MUTED));
                            ui.add_space(6.0);
                            ui.label(RichText::new(title).size(11.0).color(MUTED));
                        });
                    });

                    // Close with ESC or X button
                    if ctx.input(|i| i.key_pressed(egui::Key::Escape))
                        || ctx.input(|i| i.viewport().close_requested())
                    {
                        self.open = false;
                    }
                });
        });
    }
}
