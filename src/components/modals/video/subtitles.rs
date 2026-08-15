use crate::components::TrackItem;
use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};
use rplayer::opensubtitles::SubResult;

#[component]
pub fn SubtitlesTab(
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
    sub_font_size: i64,
    sub_color: String,
    sub_pos: i64,
    on_change_sub_font_size: EventHandler<i64>,
    on_change_sub_color: EventHandler<String>,
    on_change_sub_pos: EventHandler<i64>,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let mut search_term = use_signal(|| opensubtitles_query.clone());

    rsx! {
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

        h4 { class: "section-title", "{tr(language(), \"video_modal.sub_style_title\")}" }
        div { class: "control-group-col", style: "margin-bottom: 24px;",
            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"video_modal.sub_font_size\")}" }
                    span { class: "eq-val-label", "{sub_font_size}" }
                }
                input {
                    r#type: "range", min: "10", max: "150", value: "{sub_font_size}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_sub_font_size.call(v); } }
                }
            }
            div { class: "slider-row",
                div { style: "display: flex; justify-content: space-between;",
                    span { "{tr(language(), \"video_modal.sub_pos\")}" }
                    span { class: "eq-val-label", "{sub_pos}%" }
                }
                input {
                    r#type: "range", min: "0", max: "100", value: "{sub_pos}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<i64>() { on_change_sub_pos.call(v); } }
                }
            }
            div { class: "slider-row", style: "display: flex; align-items: center; justify-content: space-between;",
                span { "{tr(language(), \"video_modal.sub_color\")}" }
                input {
                    r#type: "color", value: "{sub_color}",
                    oninput: move |e| { on_change_sub_color.call(e.value().to_string()); }
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
    }
}
