#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bootstrap;
pub mod components;
pub mod containers;
mod state;
mod tasks;

use dioxus::desktop::{Config as DesktopConfig, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use std::path::PathBuf;

fn main() {
    std::env::set_var("LC_NUMERIC", "C");
    if std::env::args().any(|a| a == "--self-check") {
        std::process::exit(0);
    }

    #[cfg(target_os = "linux")]
    {
        std::env::set_var("GDK_BACKEND", "x11");
        std::env::remove_var("WAYLAND_DISPLAY");
    }

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cfg = DesktopConfig::new()
        .with_menu(None)
        .with_background_color((0, 0, 0, 0))
        .with_custom_head(format!(
            "<style>{}</style>",
            include_str!("../assets/style.css")
        ))
        .with_window(
            WindowBuilder::new()
                .with_title("RPlayer")
                .with_inner_size(LogicalSize::new(1140.0, 720.0))
                .with_min_inner_size(LogicalSize::new(600.0, 400.0)),
        );

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let initial_config = rplayer::config::Config::load();
    let mut state = bootstrap::init_app_state(&initial_config);
    use_context_provider(|| state);
    use_context_provider(|| state.language);

    let logic = tasks::use_app_logic(state);

    rsx! {
        div {
            class: if (state.is_dark_mode)() { "app-container dark" } else { "app-container light" },

            if !(state.is_fullscreen)() {
                containers::AppHeaderBar {
                    do_load_file: logic.do_load_file,
                }
            }

            main { class: "main-content",
                containers::AppVideoStage {
                    drop_file_fn: logic.do_load_file,
                }

                if (state.show_playlist)() {
                    containers::AppPlaylistPanel {
                        playlist_load_fn: logic.do_load_file,
                    }
                }
            }

            if (state.has_file)() && !(state.is_fullscreen)() {
                containers::AppPlayerControls {}
            }

            // --- Modals ---
            if (state.show_audio_modal)() {
                containers::AppAudioModal {}
            }

            if (state.show_video_modal)() {
                containers::AppVideoModal {}
            }

            if (state.show_tools_modal)() {
                containers::AppToolsModal {
                    resume_history_fn: logic.do_load_file,
                }
            }

            if (state.show_open_url_modal)() {
                components::modals::OpenUrlModal {
                    on_close: move |_| state.show_open_url_modal.set(false),
                    on_open: move |url: String| {
                        if let Some(p_arc) = state.player_ref.read().clone() {
                            if let Ok(p) = p_arc.lock() {
                                if let Err(e) = p.open(&PathBuf::from(&url)) {
                                    state.open_url_error.set(e.to_string());
                                } else {
                                    state.show_open_url_modal.set(false);
                                    state.current_title.set(url);
                                    state.has_file.set(true);
                                    state.playing.set(true);
                                    state.paused.set(false);
                                }
                            }
                        }
                    },
                    error: (state.open_url_error)().clone(),
                }
            }


        }
    }
}
