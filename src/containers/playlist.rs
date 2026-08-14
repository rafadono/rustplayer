use crate::components::PlaylistPanel;
use dioxus::prelude::*;
use rplayer::i18n::tr;
use rplayer::playlist::Track;
use std::path::PathBuf;

#[derive(Props, Clone, PartialEq)]
pub struct AppPlaylistPanelProps {
    playlist_load_fn: EventHandler<PathBuf>,
}

#[component]
pub fn AppPlaylistPanel(props: AppPlaylistPanelProps) -> Element {
    let state = use_context::<crate::state::AppState>();

    let mut playlist = state.playlist;
    let mut show_playlist = state.show_playlist;
    let language = state.language;
    let playlist_load_fn = props.playlist_load_fn;
    let lang = language();

    rsx! {
        PlaylistPanel {
            tracks: playlist.read().tracks.clone(),
            current_index: playlist.read().current,
            on_select_item: move |idx| {
                if let Some(path) = playlist.read().tracks.get(idx).map(|t: &Track| t.path.clone()) {
                    playlist_load_fn.call(path);
                }
            },
            on_close: move |_| show_playlist.set(false),
            on_import: move |_| {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter(tr(lang, "dialog.playlist_filter"), &["m3u", "m3u8", "pls"])
                    .pick_file()
                {
                    playlist.write().load_m3u(&path);
                }
            },
            on_export: move |_| {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter(tr(lang, "dialog.playlist_filter"), &["m3u"])
                    .set_file_name("playlist.m3u")
                    .save_file()
                {
                    let _ = playlist.read().export_m3u(&path);
                }
            },
        }
    }
}
