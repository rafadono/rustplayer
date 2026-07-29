use dioxus::html::{DragData, HasFileData};
use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};

#[component]
pub fn VideoStage(
    has_file: bool,
    current_title: String,
    on_drop_file: EventHandler<String>,
    show_metrics: bool,
    render_fps: f64,
    dropped_frames: i64,
    hwdec_active: bool,
    buffer_seconds: f64,
    metrics_opacity: f32,
    metrics_font_size: f32,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let hwdec_label = if hwdec_active {
        tr(language(), "video.metrics_hwdec_on")
    } else {
        tr(language(), "video.metrics_hwdec_off")
    };
    let fps_label =
        tr(language(), "video.metrics_fps").replacen("{}", &format!("{:.1}", render_fps), 1);
    let dropped_label =
        tr(language(), "video.metrics_dropped").replacen("{}", &dropped_frames.to_string(), 1);
    let buffer_label =
        tr(language(), "video.metrics_buffer").replacen("{}", &format!("{:.1}", buffer_seconds), 1);

    rsx! {
        main {
            class: "video-container",
            ondragover: move |evt: Event<DragData>| evt.stop_propagation(),
            ondrop: move |evt: Event<DragData>| {
                if let Some(engine) = evt.files() {
                    for path in engine.files() {
                        on_drop_file.call(path);
                    }
                }
            },
            if !has_file {
                div { class: "empty-state",
                    div { class: "empty-icon", "🎬" }
                    h2 { style: "font-size: 18px; font-weight: 500;", "{tr(language(), \"video.no_file_loaded\")}" }
                    p { style: "font-size: 13px;", "{tr(language(), \"video.no_file_hint\")}" }
                }
            } else {
                div { id: "video-canvas", style: "width: 100%; height: 100%; position: relative;",
                    div { style: "position: absolute; top: 12px; left: 16px; background: rgba(0,0,0,0.6); padding: 4px 10px; border-radius: 4px; font-size: 12px; color: #fff;",
                        "{current_title}"
                    }
                    if show_metrics {
                        div {
                            style: "position: absolute; top: 12px; right: 16px; background: rgba(0,0,0,{metrics_opacity}); padding: 6px 10px; border-radius: 4px; font-size: {metrics_font_size}px; color: #fff; font-family: monospace; line-height: 1.4;",
                            div { "{fps_label}" }
                            div { "{dropped_label}" }
                            div { "{hwdec_label}" }
                            div { "{buffer_label}" }
                        }
                    }
                }
            }
        }
    }
}
