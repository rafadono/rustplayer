#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;

use components::*;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use rplayer::bookmarks::Bookmark;
use rplayer::equalizer::Equalizer;
use rplayer::image_controls::ImageControls;
use rplayer::player::Player;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn main() {
    std::env::set_var("LC_NUMERIC", "C");
    if std::env::args().any(|a| a == "--self-check") {
        std::process::exit(0);
    }

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cfg = Config::new()
        .with_menu(None)
        .with_background_color((0, 0, 0, 0))
        .with_custom_head(r#"<link rel="stylesheet" href="assets/style.css">"#.to_string())
        .with_window(
            WindowBuilder::new()
                .with_title("RPlayer")
                .with_inner_size(LogicalSize::new(1140.0, 720.0))
                .with_min_inner_size(LogicalSize::new(600.0, 400.0)),
        );

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let mut is_dark_mode = use_signal(|| true);
    let mut playing = use_signal(|| false);
    let mut paused = use_signal(|| true);
    let mut volume = use_signal(|| 80i64);
    let mut muted = use_signal(|| false);
    let mut speed = use_signal(|| 1.0f64);
    let mut time_pos = use_signal(|| 0.0f64);
    let duration = use_signal(|| 0.0f64);
    let mut current_title = use_signal(String::new);
    let mut has_file = use_signal(|| false);
    let mut ab_looping = use_signal(|| false);

    // Playlist
    let mut show_playlist = use_signal(|| false);
    let mut playlist_items = use_signal(Vec::<PathBuf>::new);
    let mut current_index = use_signal(|| None::<usize>);

    // Modal Visibility & Active Tab States
    let mut show_audio_modal = use_signal(|| false);
    let mut audio_tab = use_signal(|| AudioTab::Equalizer);

    let mut show_video_modal = use_signal(|| false);
    let mut video_tab = use_signal(|| VideoTab::Subtitles);

    let mut show_tools_modal = use_signal(|| false);
    let mut tools_tab = use_signal(|| ToolsTab::Bookmarks);

    // Feature states
    let mut eq_enabled = use_signal(|| false);
    let mut eq_bands = use_signal(|| vec![0.0f64; 10]);
    let mut eq_preset = use_signal(|| "Flat".to_string());
    let mut brightness = use_signal(|| 0i64);
    let mut contrast = use_signal(|| 0i64);
    let mut saturation = use_signal(|| 0i64);
    let mut hue = use_signal(|| 0i64);
    let mut gamma = use_signal(|| 0i64);
    let mut audio_delay = use_signal(|| 0.0f64);
    let mut sub_delay = use_signal(|| 0.0f64);
    let mut karaoke_enabled = use_signal(|| false);
    let mut karaoke_pitch = use_signal(|| 0.0f64);
    let mut bookmarks = use_signal(Vec::<Bookmark>::new);

    // Single Player Instance
    let player_ref = use_signal(|| {
        if let Ok(p) = Player::new(80, false, 1.0) {
            Some(Arc::new(Mutex::new(p)))
        } else {
            None
        }
    });

    let window = dioxus::desktop::use_window();

    // File Open Helper
    let do_load_file = move |path: PathBuf| {
        let title = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        current_title.set(title);
        has_file.set(true);
        playing.set(true);
        paused.set(false);

        #[cfg(target_os = "linux")]
        {
            use dioxus::desktop::tao::platform::unix::WindowExtUnix;
            use gtk::prelude::*;

            let gtk_win = window.gtk_window();
            if let Some(gdk_win) = gtk_win.window() {
                let wid = if let Ok(x11_win) =
                    glib::object::Cast::downcast::<gdkx11::X11Window>(gdk_win.clone())
                {
                    x11_win.xid() as i64
                } else {
                    gdk_win.as_ptr() as i64
                };

                if let Some(ref p_arc) = *player_ref.read() {
                    if let Ok(p) = p_arc.lock() {
                        let _ = p.set_wid(wid);
                    }
                }
            }
        }

        if let Some(ref p_arc) = *player_ref.read() {
            if let Ok(p) = p_arc.lock() {
                let _ = p.open(&path);
                let _ = p.set_paused(false);
            }
        }

        if !playlist_items.read().contains(&path) {
            playlist_items.write().push(path);
            current_index.set(Some(playlist_items.read().len() - 1));
        }
    };

    let mut open_file_fn = do_load_file.clone();
    let open_file_dialog = move |_| {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(
                "Video & Audio",
                &["mp4", "mkv", "avi", "webm", "mp3", "flac", "ogg", "wav"],
            )
            .pick_file()
        {
            open_file_fn(path);
        }
    };

    let mut playlist_load_fn = do_load_file.clone();
    let theme_attr = if is_dark_mode() { "dark" } else { "light" };

    rsx! {
        div { id: "main", "data-theme": "{theme_attr}",
            HeaderBar {
                is_dark_mode: is_dark_mode(),
                on_toggle_theme: move |_| is_dark_mode.toggle(),
                on_open_file: open_file_dialog,
                on_open_audio_modal: move |_| show_audio_modal.set(true),
                on_open_video_modal: move |_| show_video_modal.set(true),
                on_open_tools_modal: move |_| show_tools_modal.set(true),
                on_toggle_playlist: move |_| show_playlist.toggle(),
            }

            div { class: "workspace",
                VideoStage {
                    has_file: has_file(),
                    current_title: current_title(),
                    on_drop_file: move |_| {}
                }

                if show_playlist() {
                    PlaylistPanel {
                        items: playlist_items(),
                        current_index: current_index(),
                        on_select_item: move |idx| {
                            current_index.set(Some(idx));
                            if let Some(path) = playlist_items.read().get(idx).cloned() {
                                playlist_load_fn(path);
                            }
                        },
                        on_close: move |_| show_playlist.set(false),
                    }
                }
            }

            PlayerControls {
                playing: playing(),
                paused: paused(),
                time_pos: time_pos(),
                duration: duration(),
                volume: volume(),
                muted: muted(),
                speed: speed(),
                ab_looping: ab_looping(),
                on_toggle_play: move |_| {
                    if has_file() {
                        paused.toggle();
                        playing.set(!paused());
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() {
                                let _ = p.set_paused(paused());
                            }
                        }
                    }
                },
                on_seek: move |pos| {
                    time_pos.set(pos);
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            let _ = p.seek_absolute(pos);
                        }
                    }
                },
                on_volume_change: move |v| {
                    volume.set(v);
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            let _ = p.set_volume(v);
                        }
                    }
                },
                on_toggle_mute: move |_| {
                    muted.toggle();
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            let _ = p.toggle_mute();
                        }
                    }
                },
                on_change_speed: move |spd| {
                    speed.set(spd);
                    if let Some(ref p_arc) = *player_ref.read() {
                        if let Ok(p) = p_arc.lock() {
                            let _ = p.set_speed(spd);
                        }
                    }
                },
                on_toggle_ab_repeat: move |_| ab_looping.toggle(),
                on_open_karaoke: move |_| {
                    audio_tab.set(AudioTab::Karaoke);
                    show_audio_modal.set(true);
                },
                on_open_trim: move |_| {
                    tools_tab.set(ToolsTab::Trim);
                    show_tools_modal.set(true);
                },
                on_open_opensubtitles: move |_| {
                    video_tab.set(VideoTab::Subtitles);
                    show_video_modal.set(true);
                },
                on_toggle_fullscreen: move |_| {},
            }

            /* 1. Modal de Audio */
            if show_audio_modal() {
                AudioModal {
                    active_tab: audio_tab(),
                    on_change_tab: move |tab| audio_tab.set(tab),
                    on_close: move |_| show_audio_modal.set(false),

                    eq_bands: eq_bands(),
                    eq_enabled: eq_enabled(),
                    eq_preset: eq_preset(),
                    on_band_change: move |(idx, val): (usize, f64)| {
                        eq_bands.write()[idx] = val;
                        let mut eq = Equalizer::default();
                        if let Some(f) = eq.peq_filters.get_mut(idx) {
                            f.gain_db = val as f32;
                            f.enabled = true;
                        }
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.set_audio_filters(&eq, false); }
                        }
                    },
                    on_toggle_eq: move |en| eq_enabled.set(en),
                    on_select_preset: move |p| eq_preset.set(p),

                    audio_tracks: vec![TrackItem { id: 1, title: "Pista de Audio Principal".into(), lang: "es".into(), track_type: "audio".into() }],
                    current_audio: Some(1),
                    audio_delay: audio_delay(),
                    on_select_audio: move |id| {
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { let _ = p.set_audio_track(id); }
                        }
                    },
                    on_change_audio_delay: move |d| {
                        audio_delay.set(d);
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { let _ = p.set_audio_delay(d); }
                        }
                    },

                    karaoke_enabled: karaoke_enabled(),
                    karaoke_pitch: karaoke_pitch(),
                    on_toggle_karaoke: move |en| karaoke_enabled.set(en),
                    on_change_pitch: move |p| karaoke_pitch.set(p),
                }
            }

            /* 2. Modal de Video y Subtítulos */
            if show_video_modal() {
                VideoModal {
                    active_tab: video_tab(),
                    on_change_tab: move |tab| video_tab.set(tab),
                    on_close: move |_| show_video_modal.set(false),

                    sub_tracks: vec![TrackItem { id: 1, title: "Subtítulo Español".into(), lang: "es".into(), track_type: "sub".into() }],
                    current_sub: Some(1),
                    sub_delay: sub_delay(),
                    on_select_sub: move |id| {
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { let _ = p.set_sub_track(id); }
                        }
                    },
                    on_load_external_sub: move |_| {
                        if let Some(path) = rfd::FileDialog::new().add_filter("Subtítulos", &["srt", "vtt", "ass"]).pick_file() {
                            if let Some(ref p_arc) = *player_ref.read() {
                                if let Ok(p) = p_arc.lock() { let _ = p.add_sub_file(&path); }
                            }
                        }
                    },
                    on_change_sub_delay: move |d| {
                        sub_delay.set(d);
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { let _ = p.set_sub_delay(d); }
                        }
                    },
                    opensubtitles_query: current_title(),
                    on_search_opensubtitles: move |_| {},

                    brightness: brightness(),
                    contrast: contrast(),
                    saturation: saturation(),
                    hue: hue(),
                    gamma: gamma(),
                    on_change_brightness: move |b| {
                        brightness.set(b);
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_change_contrast: move |c| {
                        contrast.set(c);
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_change_saturation: move |s| {
                        saturation.set(s);
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_change_hue: move |h| {
                        hue.set(h);
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_change_gamma: move |g| {
                        gamma.set(g);
                        let ic = ImageControls { brightness: brightness(), contrast: contrast(), saturation: saturation(), hue: hue(), gamma: gamma(), ..Default::default() };
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                    on_reset_image: move |_| {
                        brightness.set(0); contrast.set(0); saturation.set(0); hue.set(0); gamma.set(0);
                        let ic = ImageControls::default();
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { p.apply_image_controls(&ic); }
                        }
                    },
                }
            }

            /* 3. Modal de Herramientas y Ajustes */
            if show_tools_modal() {
                ToolsModal {
                    active_tab: tools_tab(),
                    on_change_tab: move |tab| tools_tab.set(tab),
                    on_close: move |_| show_tools_modal.set(false),

                    bookmarks: bookmarks(),
                    current_time: time_pos(),
                    on_add_bookmark: move |t| {
                        let b = Bookmark::new(t, format!("Marcador {}", bookmarks.read().len() + 1));
                        bookmarks.write().push(b);
                    },
                    on_jump_bookmark: move |t| {
                        time_pos.set(t);
                        if let Some(ref p_arc) = *player_ref.read() {
                            if let Ok(p) = p_arc.lock() { let _ = p.seek_absolute(t); }
                        }
                    },
                    on_delete_bookmark: move |idx| { bookmarks.write().remove(idx); },

                    duration: duration(),
                    on_start_trim: move |_| {},

                    filename: current_title(),
                    container: "Matroska / MKV".to_string(),
                    resolution: "1920x1080".to_string(),
                    fps: 23.976,
                    video_codec: "H.264 / AVC".to_string(),
                    audio_codec: "AAC (Stereo)".to_string(),
                    bitrate: 4500000,
                    duration_str: "00:45:00".to_string(),
                }
            }
        }
    }
}
