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

    let mut aspect_ratio = state.aspect_ratio;
    let mut crop = state.crop;
    let mut deinterlace = state.deinterlace;
    let mut sub_font_size = state.sub_font_size;
    let mut sub_color = state.sub_color;
    let mut sub_pos = state.sub_pos;

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
            sub_font_size: sub_font_size(),
            sub_color: sub_color(),
            sub_pos: sub_pos(),
            on_change_sub_font_size: move |s: i64| {
                sub_font_size.set(s);
                config.write().sub_font_size = s;
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.set_sub_font_size(s); }
                }
            },
            on_change_sub_color: move |c: String| {
                sub_color.set(c.clone());
                config.write().sub_color = c.clone();
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() {
                        let hex = c.trim_start_matches('#');
                        if hex.len() == 6 {
                            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
                            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
                            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
                            let _ = p.set_sub_color_rgb(r, g, b);
                        }
                    }
                }
            },
            on_change_sub_pos: move |pos: i64| {
                sub_pos.set(pos);
                config.write().sub_pos = pos;
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.set_sub_pos(pos); }
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
            aspect_ratio: aspect_ratio(),
            crop: crop(),
            deinterlace: deinterlace(),
            on_change_aspect_ratio: move |ar: rplayer::config::AspectRatio| {
                aspect_ratio.set(ar.clone());
                config.write().aspect_ratio = ar.clone();
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.set_aspect_ratio(&ar); }
                }
            },
            on_change_crop: move |c: f64| {
                crop.set(c);
                config.write().crop = c;
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.set_crop(c); }
                }
            },
            on_change_deinterlace: move |d: bool| {
                deinterlace.set(d);
                config.write().deinterlace = d;
                config.read().save();
                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() { let _ = p.set_deinterlace(d); }
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
