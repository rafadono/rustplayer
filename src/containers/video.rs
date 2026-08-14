use dioxus::prelude::*;
use rplayer::i18n::tr;
use rplayer::image_controls::ImageControls;
use rplayer::services::opensubtitles::{SubResult, SubSearchJob};

use crate::components::modals::VideoModal;

#[component]
pub fn AppVideoModal() -> Element {
    let state = use_context::<crate::state::AppState>();
    let mut show_video_modal = state.show_video_modal;
    let mut video_tab = state.video_tab;
    let mut config = state.config;
    let player_ref = state.player_ref;
    let mut sub_delay = state.sub_delay;
    let mut brightness = state.brightness;
    let mut contrast = state.contrast;
    let mut saturation = state.saturation;
    let mut hue = state.hue;
    let mut gamma = state.gamma;
    let mut sub_search = state.sub_search;
    let mut sub_search_status = state.sub_search_status;
    let mut sub_search_results = state.sub_search_results;
    let current_title = state.current_title;
    let playlist = state.playlist;

    // Derived
    let sub_tracks = state.sub_tracks.read().clone();
    let current_sub = *state.current_sub.read();
    let language = state.language;

    rsx! {
        VideoModal {
            active_tab: video_tab(),
            on_change_tab: move |tab| video_tab.set(tab),
            on_close: move |_| show_video_modal.set(false),

            sub_tracks: sub_tracks,
            current_sub: current_sub,
            sub_delay: sub_delay(),
            on_select_sub: move |id| {
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.set_sub_track(id); }
                }
            },
            on_load_external_sub: move |_| {
                let lang = language();
                dioxus::prelude::spawn(async move {
                    if let Some(path) = rfd::AsyncFileDialog::new()
                        .add_filter(tr(lang, "dialog.subtitles_filter"), &["srt", "vtt", "ass"])
                        .pick_file()
                        .await
                    {
                        if let Some(p_arc2) = player_ref.read().clone() {
                            if let Ok(p) = p_arc2.lock() {
                                let _ = p.add_sub_file(path.path());
                            }
                        }
                    }
                });
            },
            on_change_sub_delay: move |d| {
                sub_delay.set(d);
                config.write().sub_delay = d;
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.set_sub_delay(d); }
                }
            },
            opensubtitles_query: current_title(),
            on_search_opensubtitles: move |_query: String| {
                let path_opt = playlist.read().current.and_then(|i| playlist.read().tracks.get(i).cloned()).map(|t| t.path);
                if let Some(path) = path_opt {
                        sub_search_results.set(vec![]);
                        sub_search_status.set(tr(language(), "opensub.searching").to_string());
                        let job = SubSearchJob::search(&path, "es");
                        if job.is_none() {
                            sub_search_status.set(tr(language(), "opensub.not_configured").to_string());
                        }
                        sub_search.set(job);
                }
            },
            sub_search_status: sub_search_status(),
            sub_search_results: sub_search_results(),
            on_download_sub: move |result: SubResult| {
                let path_opt = playlist.read().current.and_then(|i| playlist.read().tracks.get(i).cloned()).map(|t| t.path);
                if let Some(path) = path_opt {
                    let dest_dir = path.parent().map(std::path::Path::to_path_buf).unwrap_or_default();
                    sub_search_status.set(tr(language(), "opensub.downloading").to_string());
                    sub_search.set(SubSearchJob::download(&result, &dest_dir));
                }
            },

            brightness: brightness(),
            contrast: contrast(),
            saturation: saturation(),
            hue: hue(),
            gamma: gamma(),
            on_change_brightness: move |b| {
                brightness.set(b);
                config.write().image_controls.brightness = b;
                config.read().save();
                let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                }
            },
            on_change_contrast: move |c| {
                contrast.set(c);
                config.write().image_controls.contrast = c;
                config.read().save();
                let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                }
            },
            on_change_saturation: move |s| {
                saturation.set(s);
                config.write().image_controls.saturation = s;
                config.read().save();
                let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                }
            },
            on_change_hue: move |h| {
                hue.set(h);
                config.write().image_controls.hue = h;
                config.read().save();
                let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                }
            },
            on_change_gamma: move |g| {
                gamma.set(g);
                config.write().image_controls.gamma = g;
                config.read().save();
                let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                }
            },
            on_reset_image: move |_| {
                brightness.set(0); contrast.set(0); saturation.set(0); hue.set(0); gamma.set(0);
                config.write().image_controls = ImageControls::default();
                config.read().save();
                let ic = ImageControls::default();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                }
            },
        }
    }
}
