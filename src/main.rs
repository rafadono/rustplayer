#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(
    dead_code,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::ptr_arg,
    clippy::collapsible_if,
    clippy::field_reassign_with_default,
    clippy::derivable_impls,
    clippy::needless_lifetimes,
    clippy::manual_ignore_case_cmp,
    clippy::manual_clamp,
    clippy::manual_split_once,
    clippy::map_identity,
    clippy::needless_return
)]

mod ab_repeat;
mod app;
mod bookmarks;
mod chapters;
mod config;
mod converter;
mod donation;
mod equalizer;
mod history;
mod i18n;
mod image_controls;
mod karaoke;
mod lastfm;
mod media_info;
mod notes;
mod opensubtitles;
mod player;
mod playlist;
mod remote;
mod renderer;
mod sleep_timer;
mod streaming;
mod theme_manager;
mod thumbnail;
mod trim;
mod ui;
mod up_next;
mod updater;

use eframe::egui;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    if std::env::args().any(|a| a == "--self-check") {
        return Ok(());
    }

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon-rp.png")).ok();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("RPlayer")
            .with_inner_size([1140.0, 700.0])
            .with_min_inner_size([560.0, 380.0])
            .with_icon(Arc::new(icon.unwrap_or_default())),
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "RPlayer",
        options,
        Box::new(|cc| Box::new(app::PlayerApp::new(cc))),
    )
}
