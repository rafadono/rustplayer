use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};

#[component]
pub fn PlayerControls(
    playing: bool,
    paused: bool,
    time_pos: f64,
    duration: f64,
    volume: i64,
    muted: bool,
    speed: f64,
    ab_loop_active: bool,
    ab_loop_label: String,
    hover_thumb_uri: Option<String>,
    on_seek_preview: EventHandler<f64>,
    on_seek_preview_end: EventHandler<()>,
    on_toggle_play: EventHandler<()>,
    on_seek: EventHandler<f64>,
    on_volume_change: EventHandler<i64>,
    on_toggle_mute: EventHandler<()>,
    on_change_speed: EventHandler<f64>,
    on_toggle_ab_repeat: EventHandler<()>,
    on_open_karaoke: EventHandler<()>,
    on_open_trim: EventHandler<()>,
    on_open_opensubtitles: EventHandler<()>,
    on_toggle_fullscreen: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let format_time = |secs: f64| -> String {
        let s = secs as u64;
        let h = s / 3600;
        let m = (s % 3600) / 60;
        let sec = s % 60;
        if h > 0 {
            format!("{:02}:{:02}:{:02}", h, m, sec)
        } else {
            format!("{:02}:{:02}", m, sec)
        }
    };

    let pos_str = format_time(time_pos);
    let dur_str = format_time(duration);
    let seek_val = if duration > 0.0 {
        (time_pos / duration) * 100.0
    } else {
        0.0
    };

    let speeds = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 3.0, 4.0];
    let mut preview_pct = use_signal(|| None::<f64>);

    rsx! {
        footer { class: "controls-bar",
            div { class: "progress-bar-container", style: "position: relative;",
                if let (Some(uri), Some(pct)) = (hover_thumb_uri.as_ref(), preview_pct()) {
                    div {
                        style: "position: absolute; bottom: 100%; left: {pct}%; transform: translateX(-50%); margin-bottom: 8px; pointer-events: none;",
                        img { src: "{uri}", style: "width: 160px; border-radius: 4px; border: 1px solid var(--border-color); box-shadow: 0 2px 8px rgba(0,0,0,0.4);" }
                    }
                }
                span { class: "time-label", "{pos_str}" }
                input {
                    class: "seek-slider",
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "{seek_val}",
                    oninput: move |e| {
                        if let Ok(pct) = e.value().parse::<f64>() {
                            let target = (pct / 100.0) * duration;
                            on_seek.call(target);
                            preview_pct.set(Some(pct));
                            on_seek_preview.call(target);
                        }
                    },
                    onmouseup: move |_| {
                        preview_pct.set(None);
                        on_seek_preview_end.call(());
                    }
                }
                span { class: "time-label", "{dur_str}" }
            }

            div { class: "controls-row",
                div { class: "left-controls",
                    button {
                        class: "btn-icon",
                        onclick: move |_| on_toggle_mute.call(()),
                        if muted || volume == 0 { "🔇" } else { "🔊" }
                    }
                    input {
                        class: "volume-slider",
                        r#type: "range",
                        min: "0",
                        max: "100",
                        value: "{volume}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<i64>() {
                                on_volume_change.call(v);
                            }
                        }
                    }

                    select {
                        class: "select-input",
                        style: "padding: 2px 6px; font-size: 11px;",
                        value: "{speed}",
                        onchange: move |e| {
                            if let Ok(s) = e.value().parse::<f64>() {
                                on_change_speed.call(s);
                            }
                        },
                        for s in speeds {
                            option { value: "{s}", "{s}x" }
                        }
                    }
                }

                div { class: "center-controls", style: "display: flex; gap: 8px; align-items: center;",
                    button {
                        class: "btn-icon",
                        style: "border: 1px solid var(--border-color); padding: 4px 10px;",
                        onclick: move |_| {
                            let target = (time_pos - 10.0).max(0.0);
                            on_seek.call(target);
                        },
                        title: "{tr(language(), \"controls.seek_back_10\")}",
                        "⏪ -10s"
                    }
                    button {
                        class: "btn-primary",
                        style: "font-size: 15px; padding: 6px 18px;",
                        onclick: move |_| on_toggle_play.call(()),
                        if paused || !playing { "{tr(language(), \"controls.play\")}" } else { "{tr(language(), \"controls.pause\")}" }
                    }
                    button {
                        class: "btn-icon",
                        style: "border: 1px solid var(--border-color); padding: 4px 10px;",
                        onclick: move |_| {
                            let target = (time_pos + 10.0).min(duration);
                            on_seek.call(target);
                        },
                        title: "{tr(language(), \"controls.seek_fwd_10\")}",
                        "⏩ +10s"
                    }
                }

                div { class: "right-controls",
                    button {
                        class: if ab_loop_active { "btn-icon active" } else { "btn-icon" },
                        onclick: move |_| on_toggle_ab_repeat.call(()),
                        title: "{tr(language(), \"controls.ab_loop_tooltip\")}",
                        "🔂 {ab_loop_label}"
                    }

                    button {
                        class: "btn-icon",
                        onclick: move |_| on_open_karaoke.call(()),
                        title: "{tr(language(), \"menu.karaoke\")}",
                        "🎤"
                    }

                    button {
                        class: "btn-icon",
                        onclick: move |_| on_open_trim.call(()),
                        title: "{tr(language(), \"controls.trim_tooltip\")}",
                        "✂️"
                    }

                    button {
                        class: "btn-icon",
                        onclick: move |_| on_open_opensubtitles.call(()),
                        title: "{tr(language(), \"controls.search_subtitles_tooltip\")}",
                        "📜"
                    }

                    button {
                        class: "btn-icon",
                        onclick: move |_| on_toggle_fullscreen.call(()),
                        title: "{tr(language(), \"controls.fullscreen_tooltip\")}",
                        "⛶"
                    }
                }
            }
        }
    }
}
