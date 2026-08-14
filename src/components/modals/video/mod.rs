pub mod image;
pub mod subtitles;

use crate::components::TrackItem;
use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};
use rplayer::opensubtitles::SubResult;

use image::ImageTab;
use subtitles::SubtitlesTab;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VideoTab {
    Subtitles,
    Image,
}

#[component]
pub fn VideoModal(
    active_tab: VideoTab,
    on_change_tab: EventHandler<VideoTab>,
    on_close: EventHandler<()>,
    // Subtitles Props
    sub_tracks: Vec<TrackItem>,
    current_sub: Option<i64>,
    sub_delay: f64,
    on_select_sub: EventHandler<i64>,
    on_load_external_sub: EventHandler<()>,
    on_change_sub_delay: EventHandler<f64>,
    opensubtitles_query: String,
    on_search_opensubtitles: EventHandler<String>,
    sub_search_status: String,
    sub_search_results: Vec<SubResult>,
    on_download_sub: EventHandler<SubResult>,
    // Image Controls Props
    brightness: i64,
    contrast: i64,
    saturation: i64,
    hue: i64,
    gamma: i64,
    on_change_brightness: EventHandler<i64>,
    on_change_contrast: EventHandler<i64>,
    on_change_saturation: EventHandler<i64>,
    on_change_hue: EventHandler<i64>,
    on_change_gamma: EventHandler<i64>,
    on_reset_image: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let tabs = [
        (VideoTab::Subtitles, "video_modal.tab_subtitles"),
        (VideoTab::Image, "video_modal.tab_image"),
    ];

    rsx! {
        div { class: "modal-overlay", onclick: move |_| on_close.call(()),
            div { class: "modal-card-large", onclick: move |e| e.stop_propagation(),
                div { class: "modal-header",
                    h3 { "{tr(language(), \"video_modal.title\")}" }
                    button { class: "btn-icon", onclick: move |_| on_close.call(()), "✕" }
                }

                div { class: "modal-layout-tabbed",
                    nav { class: "modal-sidebar-tabs",
                        for (tab, label_key) in tabs {
                            {
                                let is_active = active_tab == tab;
                                let class_name = if is_active { "tab-button active" } else { "tab-button" };
                                rsx! {
                                    button {
                                        key: "{label_key}",
                                        class: "{class_name}",
                                        onclick: move |_| on_change_tab.call(tab),
                                        "{tr(language(), label_key)}"
                                    }
                                }
                            }
                        }
                    }

                    main { class: "modal-tab-content",
                        match active_tab {
                            VideoTab::Subtitles => rsx! {
                                SubtitlesTab {
                                    sub_tracks,
                                    current_sub,
                                    sub_delay,
                                    on_select_sub,
                                    on_load_external_sub,
                                    on_change_sub_delay,
                                    opensubtitles_query,
                                    on_search_opensubtitles,
                                    sub_search_status,
                                    sub_search_results,
                                    on_download_sub,
                                }
                            },
                            VideoTab::Image => rsx! {
                                ImageTab {
                                    brightness,
                                    contrast,
                                    saturation,
                                    hue,
                                    gamma,
                                    on_change_brightness,
                                    on_change_contrast,
                                    on_change_saturation,
                                    on_change_hue,
                                    on_change_gamma,
                                    on_reset_image,
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
