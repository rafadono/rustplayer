use crate::components::TrackItem;
use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};

#[component]
pub fn TracksTab(
    audio_tracks: Vec<TrackItem>,
    current_audio: Option<i64>,
    audio_delay: f64,
    on_select_audio: EventHandler<i64>,
    on_change_audio_delay: EventHandler<f64>,
    lastfm_connected: bool,
    lastfm_username: String,
    lastfm_status: String,
    on_lastfm_connect: EventHandler<(String, String)>,
    on_lastfm_disconnect: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let mut lastfm_user_input = use_signal(String::new);
    let mut lastfm_pass_input = use_signal(String::new);

    rsx! {
        h4 { class: "section-title", "{tr(language(), \"audio.tracks_available\")}" }
        div { class: "tracks-list",
            for track in audio_tracks {
                {
                    let is_sel = current_audio == Some(track.id);
                    let class_name = if is_sel { "track-item active" } else { "track-item" };
                    let track_id = track.id;
                    rsx! {
                        div {
                            key: "{track.id}",
                            class: "{class_name}",
                            onclick: move |_| on_select_audio.call(track_id),
                            span { "{track.title}" }
                            span { class: "eq-val-label", "{track.lang}" }
                        }
                    }
                }
            }
        }
        div { class: "slider-row", style: "margin-top: 16px;",
            div { style: "display: flex; justify-content: space-between;",
                span { "{tr(language(), \"audio.delay_label\")}" }
                span { class: "eq-val-label", "{audio_delay:+.1}s" }
            }
            input {
                r#type: "range", min: "-5.0", max: "5.0", step: "0.1",
                value: "{audio_delay}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<f64>() {
                        on_change_audio_delay.call(v);
                    }
                }
            }
        }
        h4 { class: "section-title", style: "margin-top: 24px;", "{tr(language(), \"audio.lastfm_section_title\")}" }
        p { style: "font-size: 13px; color: var(--text-muted); margin-bottom: 12px;",
            "{tr(language(), \"audio.lastfm_description\")}"
        }
        if lastfm_connected {
            div { style: "display: flex; align-items: center; gap: 12px;",
                span { "{tr(language(), \"audio.lastfm_connected_as\")}" strong { "{lastfm_username}" } }
                button { class: "btn-icon", onclick: move |_| on_lastfm_disconnect.call(()), "{tr(language(), \"audio.lastfm_disconnect\")}" }
            }
        } else {
            div { class: "control-group-col", style: "gap: 8px; max-width: 320px;",
                input {
                    class: "select-input",
                    r#type: "text",
                    placeholder: "{tr(language(), \"audio.lastfm_username_placeholder\")}",
                    value: "{lastfm_user_input}",
                    oninput: move |e| lastfm_user_input.set(e.value()),
                }
                input {
                    class: "select-input",
                    r#type: "password",
                    placeholder: "{tr(language(), \"audio.lastfm_password_placeholder\")}",
                    value: "{lastfm_pass_input}",
                    oninput: move |e| lastfm_pass_input.set(e.value()),
                }
                button {
                    class: "btn-primary",
                    onclick: move |_| on_lastfm_connect.call((lastfm_user_input(), lastfm_pass_input())),
                    "{tr(language(), \"audio.lastfm_connect_button\")}"
                }
                if !lastfm_status.is_empty() {
                    span { style: "font-size: 12px; color: var(--text-muted);", "{lastfm_status}" }
                }
            }
        }
    }
}
