use crate::components::VideoStage;
use dioxus::prelude::*;
use std::path::PathBuf;

#[derive(Props, Clone, PartialEq)]
pub struct AppVideoStageProps {
    drop_file_fn: EventHandler<PathBuf>,
}

#[component]
pub fn AppVideoStage(props: AppVideoStageProps) -> Element {
    let state = use_context::<crate::state::AppState>();

    let has_file = state.has_file;
    let current_title = state.current_title;
    let show_metrics = state.show_metrics;
    let render_fps = state.render_fps;
    let dropped_frames = state.dropped_frames;
    let hwdec_active = state.hwdec_active;
    let buffer_seconds = state.buffer_seconds;
    let config = state.config;

    let drop_file_fn = props.drop_file_fn;

    rsx! {
        VideoStage {
            has_file: has_file(),
            current_title: current_title.read().clone(),
            on_drop_file: move |path: String| drop_file_fn.call(PathBuf::from(path)),
            show_metrics: show_metrics(),
            render_fps: render_fps(),
            dropped_frames: dropped_frames(),
            hwdec_active: hwdec_active(),
            buffer_seconds: buffer_seconds(),
            metrics_opacity: config.read().metrics_overlay_opacity,
            metrics_font_size: config.read().metrics_overlay_font_size,
        }
    }
}
