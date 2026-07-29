use dioxus::prelude::*;

#[component]
pub fn HeaderBar(
    is_dark_mode: bool,
    on_toggle_theme: EventHandler<()>,
    on_open_file: EventHandler<()>,
    on_open_audio_modal: EventHandler<()>,
    on_open_video_modal: EventHandler<()>,
    on_open_tools_modal: EventHandler<()>,
    on_toggle_playlist: EventHandler<()>,
) -> Element {
    let theme_icon = if is_dark_mode { "☀️" } else { "🌙" };

    rsx! {
        header { class: "header-bar",
            div { class: "header-title",
                svg {
                    width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "var(--accent-color)", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                    polygon { points: "5 3 19 12 5 21 5 3" }
                }
                span { style: "color: var(--accent-color); font-weight: 700; font-size: 16px;", "RPlayer" }
                span { style: "opacity: 0.5; font-size: 11px; margin-left: 4px;", "v0.5.0" }
            }

            div { class: "header-actions",
                button { class: "btn-primary", onclick: move |_| on_open_file.call(()),
                    svg { width: "15", height: "15", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" }
                    }
                    "Abrir"
                }

                button { class: "btn-icon", onclick: move |_| on_open_audio_modal.call(()), title: "Ecualizador, Pistas de Audio, Karaoke",
                    svg { width: "15", height: "15", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M2 10v4" } path { d: "M6 6v12" } path { d: "M10 3v18" } path { d: "M14 8v8" } path { d: "M18 5v14" } path { d: "M22 10v4" }
                    }
                    "Audio"
                }

                button { class: "btn-icon", onclick: move |_| on_open_video_modal.call(()), title: "Subtítulos, Ajustes de Imagen, OpenSubtitles",
                    svg { width: "15", height: "15", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        rect { x: "2", y: "4", width: "20", height: "16", rx: "2" }
                        path { d: "m10 9 5 3-5 3V9z" }
                    }
                    "Video"
                }

                button { class: "btn-icon", onclick: move |_| on_open_tools_modal.call(()), title: "Marcadores, Recorte, Ficha Técnica, Ajustes",
                    svg { width: "15", height: "15", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "3" }
                        path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
                    }
                    "Herramientas"
                }

                button { class: "btn-icon", onclick: move |_| on_toggle_playlist.call(()), title: "Lista de reproducción",
                    svg { width: "15", height: "15", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        line { x1: "8", y1: "6", x2: "21", y2: "6" } line { x1: "8", y1: "12", x2: "21", y2: "12" } line { x1: "8", y1: "18", x2: "21", y2: "18" }
                        line { x1: "3", y1: "6", x2: "3.01", y2: "6" } line { x1: "3", y1: "12", x2: "3.01", y2: "12" } line { x1: "3", y1: "18", x2: "3.01", y2: "18" }
                    }
                    "Playlist"
                }

                button {
                    class: "btn-icon",
                    onclick: move |_| on_toggle_theme.call(()),
                    title: if is_dark_mode { "Cambiar a Modo Claro" } else { "Cambiar a Modo Oscuro" },
                    "{theme_icon}"
                }
            }
        }
    }
}
