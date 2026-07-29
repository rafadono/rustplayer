use dioxus::prelude::*;
use rplayer::bookmarks::Bookmark;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolsTab {
    Bookmarks,
    Trim,
    MediaInfo,
    Settings,
}

#[component]
pub fn ToolsModal(
    active_tab: ToolsTab,
    on_change_tab: EventHandler<ToolsTab>,
    on_close: EventHandler<()>,
    // Bookmarks
    bookmarks: Vec<Bookmark>,
    current_time: f64,
    on_add_bookmark: EventHandler<f64>,
    on_jump_bookmark: EventHandler<f64>,
    on_delete_bookmark: EventHandler<usize>,
    // Trim
    duration: f64,
    on_start_trim: EventHandler<()>,
    // Media Info
    filename: String,
    container: String,
    resolution: String,
    fps: f64,
    video_codec: String,
    audio_codec: String,
    bitrate: i64,
    duration_str: String,
) -> Element {
    let tabs = [
        (ToolsTab::Bookmarks, "🔖 Marcadores & Notas"),
        (ToolsTab::Trim, "✂️ Recorte sin Pérdida"),
        (ToolsTab::MediaInfo, "ℹ️ Ficha Técnica"),
        (ToolsTab::Settings, "⚙️ Red & Preferencias"),
    ];

    let mut start_time = use_signal(|| 0.0f64);
    let mut end_time = use_signal(|| duration);

    rsx! {
        div { class: "modal-overlay", onclick: move |_| on_close.call(()),
            div { class: "modal-card-large", onclick: move |e| e.stop_propagation(),
                div { class: "modal-header",
                    h3 { "⚙️ Centro de Herramientas y Ajustes" }
                    button { class: "btn-icon", onclick: move |_| on_close.call(()), "✕" }
                }

                div { class: "modal-layout-tabbed",
                    nav { class: "modal-sidebar-tabs",
                        for (tab, label) in tabs {
                            {
                                let is_active = active_tab == tab;
                                let class_name = if is_active { "tab-button active" } else { "tab-button" };
                                rsx! {
                                    button {
                                        key: "{label}",
                                        class: "{class_name}",
                                        onclick: move |_| on_change_tab.call(tab),
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }

                    main { class: "modal-tab-content",
                        match active_tab {
                            ToolsTab::Bookmarks => rsx! {
                                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",
                                    h4 { class: "section-title", style: "margin: 0;", "Marcadores del Archivo" }
                                    button { class: "btn-primary", onclick: move |_| on_add_bookmark.call(current_time), "➕ Añadir Marcador Actual" }
                                }
                                div { class: "tracks-list",
                                    for (idx, b) in bookmarks.iter().enumerate() {
                                        {
                                            let pos = b.position;
                                            let label = b.label.clone();
                                            rsx! {
                                                div { key: "{b.id}", class: "track-item",
                                                    div { style: "display: flex; gap: 8px; align-items: center;",
                                                        span { style: "font-family: monospace; color: var(--accent-color);", "{pos:.1}s" }
                                                        span { "{label}" }
                                                    }
                                                    div { style: "display: flex; gap: 8px;",
                                                        button { class: "btn-icon", onclick: move |_| on_jump_bookmark.call(pos), "▶ Saltar" }
                                                        button { class: "btn-icon", onclick: move |_| on_delete_bookmark.call(idx), "🗑️" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            ToolsTab::Trim => rsx! {
                                div { class: "control-group-col",
                                    h4 { class: "section-title", "Recortar Clip de Video sin Pérdida" }
                                    div { class: "slider-row",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { "Punto de Inicio" }
                                            span { class: "eq-val-label", "{start_time:.1}s" }
                                        }
                                        input {
                                            r#type: "range", min: "0", max: "{duration}", value: "{start_time}",
                                            oninput: move |e| { if let Ok(v) = e.value().parse::<f64>() { start_time.set(v); } }
                                        }
                                    }
                                    div { class: "slider-row",
                                        div { style: "display: flex; justify-content: space-between;",
                                            span { "Punto Final" }
                                            span { class: "eq-val-label", "{end_time:.1}s" }
                                        }
                                        input {
                                            r#type: "range", min: "0", max: "{duration}", value: "{end_time}",
                                            oninput: move |e| { if let Ok(v) = e.value().parse::<f64>() { end_time.set(v); } }
                                        }
                                    }
                                    div { style: "margin-top: 16px;",
                                        button { class: "btn-primary", onclick: move |_| on_start_trim.call(()), "✂️ Exportar Clip Recortado" }
                                    }
                                }
                            },
                            ToolsTab::MediaInfo => rsx! {
                                div { class: "info-table",
                                    div { class: "info-row", span { class: "info-label", "Archivo:" } span { "{filename}" } }
                                    div { class: "info-row", span { class: "info-label", "Contenedor:" } span { "{container}" } }
                                    div { class: "info-row", span { class: "info-label", "Resolución:" } span { "{resolution}" } }
                                    div { class: "info-row", span { class: "info-label", "Fotogramas (FPS):" } span { "{fps:.3}" } }
                                    div { class: "info-row", span { class: "info-label", "Códec Video:" } span { "{video_codec}" } }
                                    div { class: "info-row", span { class: "info-label", "Códec Audio:" } span { "{audio_codec}" } }
                                    div { class: "info-row", span { class: "info-label", "Tasa de Bits (Bitrate):" } span { "{bitrate / 1000} kbps" } }
                                    div { class: "info-row", span { class: "info-label", "Duración Total:" } span { "{duration_str}" } }
                                }
                            },
                            ToolsTab::Settings => rsx! {
                                div { style: "padding: 4px;",
                                    h4 { class: "section-title", "📱 Servidor de Control Remoto HTTP" }
                                    p { style: "font-size: 13px; color: var(--text-muted); margin-bottom: 12px;",
                                        "Controla la reproducción desde tu teléfono u otro navegador en la red local (http://localhost:8080)."
                                    }
                                    button { class: "btn-primary", "▶ Iniciar Servidor Remoto (Puerto 8080)" }

                                    h4 { class: "section-title", style: "margin-top: 24px;", "⏰ Temporizador de Apagado (Sleep Timer)" }
                                    div { style: "display: flex; gap: 8px; margin-top: 8px;",
                                        button { class: "btn-icon", style: "border: 1px solid var(--border-color);", "15 min" }
                                        button { class: "btn-icon", style: "border: 1px solid var(--border-color);", "30 min" }
                                        button { class: "btn-icon", style: "border: 1px solid var(--border-color);", "45 min" }
                                        button { class: "btn-icon", style: "border: 1px solid var(--border-color);", "60 min" }
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
