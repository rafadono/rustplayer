use crate::components::TrackItem;
use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};
use rplayer::opensubtitles::SubResult;

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

    let mut search_term = use_signal(|| opensubtitles_query.clone());

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
                                h4 { class: "section-title", "{tr(language(), \"video_modal.embedded_tracks_title\")}" }
                                div { class: "tracks-list",
                                    for track in sub_tracks {
                                        {
                                            let is_sel = current_sub == Some(track.id);
                                            let class_name = if is_sel { "track-item active" } else { "track-item" };
                                            let track_id = track.id;
                                            rsx! {
                                                div {
                                                    key: "{track.id}",
                                                    class: "{class_name}",
                                                    onclick: move |_| on_select_sub.call(track_id),
                                                    span { "{track.title}" }
                                                    span { class: "eq-val-label", "{track.lang}" }
                                                }
                                            }
                                        }
                                    }
                                }
                                div { style: "display: flex; gap: 12px; align-items: center; margin-bottom: 16px;",
                                    button { class: "btn-primary", onclick: move |_| on_load_external_sub.call(()), "{tr(language(), \"video_modal.load_external_sub\")}" }
                                }
                                div { class: "slider-row", style: "margin-bottom: 24px;",
                                    div { style: "display: flex; justify-content: space-between;",
                                        span { "{tr(language(), \"video_modal.sub_delay_label\")}" }
                                        span { class: "eq-val-label", "{sub_delay:+.1}s" }
                                    }
                                    input {
                                        r#type: "range", min: "-5.0", max: "5.0", step: "0.1",
                                        value: "{sub_delay}",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse::<f64>() {
                                                on_change_sub_delay.call(v);
                                            }
                                        }
                                    }
                                }

                                h4 { class: "section-title", "{tr(language(), \"video_modal.opensubtitles_section_title\")}" }
                                div { style: "display: flex; gap: 8px; margin-top: 8px;",
                                    input {
                                        class: "select-input",
                                        style: "flex: 1;",
                                        r#type: "text",
                                        placeholder: "{tr(language(), \"video_modal.search_placeholder\")}",
                                        value: "{search_term}",
                                        oninput: move |e| search_term.set(e.value())
                                    }
                                    button { class: "btn-primary", onclick: move |_| on_search_opensubtitles.call(search_term()), "{tr(language(), \"video_modal.search_button\")}" }
                                }
                                if !sub_search_status.is_empty() {
                                    p { style: "font-size: 12px; color: var(--text-muted); margin-top: 8px;", "{sub_search_status}" }
                                }
                                div { class: "tracks-list", style: "margin-top: 8px;",
                                    for r in sub_search_results {
                                        {
                                            let result = r.clone();
                                            rsx! {
                                                div { key: "{r.file_id}", class: "track-item",
                                                    div { style: "display: flex; flex-direction: column;",
                                                        span { "{r.title}" }
                                                        span { class: "eq-val-label", "{r.language} · {r.release}" }
                                                    }
                                                    button { class: "btn-icon", onclick: move |_| on_download_sub.call(result.clone()), "{tr(language(), \"video_modal.download_button\")}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            VideoTab::Image => rsx! {
                                div { class: "control-group-col",
                                    div { class: "slider-row",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { "{tr(language(), \"video_modal.brightness_label\")}" }
                                            span { class: "eq-val-label", "{brightness}" }
                                        }
                                        input {
                                            r#type: "range", min: "-100", max: "100", value: "{brightness}",
                                            oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_brightness.call(v); } }
                                        }
                                    }

                                    div { class: "slider-row",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { "{tr(language(), \"video_modal.contrast_label\")}" }
                                            span { class: "eq-val-label", "{contrast}" }
                                        }
                                        input {
                                            r#type: "range", min: "-100", max: "100", value: "{contrast}",
                                            oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_contrast.call(v); } }
                                        }
                                    }

                                    div { class: "slider-row",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { "{tr(language(), \"video_modal.saturation_label\")}" }
                                            span { class: "eq-val-label", "{saturation}" }
                                        }
                                        input {
                                            r#type: "range", min: "-100", max: "100", value: "{saturation}",
                                            oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_saturation.call(v); } }
                                        }
                                    }

                                    div { class: "slider-row",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { "{tr(language(), \"video_modal.hue_label\")}" }
                                            span { class: "eq-val-label", "{hue}" }
                                        }
                                        input {
                                            r#type: "range", min: "-100", max: "100", value: "{hue}",
                                            oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_hue.call(v); } }
                                        }
                                    }

                                    div { class: "slider-row",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { "{tr(language(), \"video_modal.gamma_label\")}" }
                                            span { class: "eq-val-label", "{gamma}" }
                                        }
                                        input {
                                            r#type: "range", min: "-100", max: "100", value: "{gamma}",
                                            oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_gamma.call(v); } }
                                        }
                                    }

                                    div { style: "margin-top: 12px;",
                                        button { class: "btn-primary", style: "background: var(--bg-surface-hover); color: var(--text-main); border: 1px solid var(--border-color);",
                                            onclick: move |_| on_reset_image.call(()),
                                            "{tr(language(), \"video_modal.reset_defaults\")}"
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
