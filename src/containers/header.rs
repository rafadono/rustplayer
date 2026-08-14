use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};
use rplayer::theme_manager::{ThemeColors, ThemePreset};
use std::path::PathBuf;

use crate::components::HeaderBar;

#[derive(Props, Clone, PartialEq)]
pub struct AppHeaderBarProps {
    do_load_file: EventHandler<PathBuf>,
}

#[component]
pub fn AppHeaderBar(props: AppHeaderBarProps) -> Element {
    let state = use_context::<crate::state::AppState>();

    let mut is_dark_mode = state.is_dark_mode;
    let mut config = state.config;
    let mut open_url_error = state.open_url_error;
    let mut show_open_url_modal = state.show_open_url_modal;
    let mut show_audio_modal = state.show_audio_modal;
    let mut show_video_modal = state.show_video_modal;
    let mut show_tools_modal = state.show_tools_modal;
    let mut show_playlist = state.show_playlist;
    let mut language = state.language;

    let do_load_file = props.do_load_file;
    let lang = language();

    let open_file_dialog = move |_| {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(
                tr(lang, "dialog.media_filter"),
                &["mp4", "mkv", "avi", "webm", "mp3", "flac", "ogg", "wav"],
            )
            .pick_file()
        {
            do_load_file.call(path);
        }
    };

    rsx! {
        HeaderBar {
            is_dark_mode: is_dark_mode(),
            on_toggle_theme: move |_| {
                is_dark_mode.toggle();
                let preset = if is_dark_mode() { ThemePreset::DarkBlue } else { ThemePreset::Light };
                config.write().theme = ThemeColors::from_preset(&preset);
                config.read().save();
            },
            on_open_file: open_file_dialog,
            on_open_url_modal: move |_| {
                open_url_error.set(String::new());
                show_open_url_modal.set(true);
            },
            on_open_audio_modal: move |_| show_audio_modal.set(true),
            on_open_video_modal: move |_| show_video_modal.set(true),
            on_open_tools_modal: move |_| show_tools_modal.set(true),
            on_toggle_playlist: move |_| show_playlist.toggle(),
            on_toggle_language: move |_| {
                let next = if language() == Language::Es { Language::En } else { Language::Es };
                language.set(next);
                config.write().language = next;
                config.read().save();
            },
        }
    }
}
