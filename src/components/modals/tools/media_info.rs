use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};
use rplayer::media_info::MediaInfo;

#[component]
pub fn MediaInfoTab(filename: String, media_info: Option<MediaInfo>) -> Element {
    let language = use_context::<Signal<Language>>();

    let info = media_info.clone().unwrap_or_default();
    rsx! {
        div { class: "info-table",
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_file\")}" } span { "{filename}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_container\")}" } span { "{info.format}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_resolution\")}" } span { "{info.resolution()}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_fps\")}" } span { "{info.fps:.3}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_video_codec\")}" } span { "{info.video_codec}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_audio_codec\")}" } span { "{info.audio_codec}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_audio_channels\")}" } span { "{info.channel_str()}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_video_bitrate\")}" } span { "{info.video_bitrate_str()}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_audio_bitrate\")}" } span { "{info.audio_bitrate_str()}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_file_size\")}" } span { "{info.size_str()}" } }
            div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.info_total_duration\")}" } span { "{rplayer::player::PlayerState::format_time(info.duration)}" } }
        }
    }
}
