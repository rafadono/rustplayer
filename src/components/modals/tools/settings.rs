use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};
use rplayer::updater::{UpdateChannel, UpdateInfo};

#[component]
pub fn SettingsTab(
    remote_running: bool,
    remote_port: u16,
    on_toggle_remote: EventHandler<()>,
    sleep_remaining_secs: Option<u64>,
    on_set_sleep_timer: EventHandler<u64>,
    on_cancel_sleep_timer: EventHandler<()>,
    show_metrics: bool,
    on_toggle_metrics: EventHandler<bool>,
    update_channel: UpdateChannel,
    on_change_update_channel: EventHandler<UpdateChannel>,
    auto_check_updates: bool,
    on_toggle_auto_check_updates: EventHandler<bool>,
    update_manifest_url_stable: String,
    update_manifest_url_beta: String,
    on_change_manifest_url: EventHandler<(UpdateChannel, String)>,
    update_status: String,
    update_info: Option<UpdateInfo>,
    on_check_updates: EventHandler<()>,
    on_install_update: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();

    let remote_description = tr(language(), "tools_modal.remote_description").replacen(
        "{}",
        &remote_port.to_string(),
        1,
    );
    let remote_start_label = tr(language(), "tools_modal.remote_start_button").replacen(
        "{}",
        &remote_port.to_string(),
        1,
    );
    let sleep_active_label = sleep_remaining_secs.map(|secs| {
        tr(language(), "tools_modal.sleep_timer_active").replacen(
            "{}",
            &rplayer::player::PlayerState::format_time(secs as f64),
            1,
        )
    });
    let stable_btn_class = if update_channel == UpdateChannel::Stable {
        "btn-icon active"
    } else {
        "btn-icon"
    };
    let beta_btn_class = if update_channel == UpdateChannel::Beta {
        "btn-icon active"
    } else {
        "btn-icon"
    };
    let update_notes = update_info
        .as_ref()
        .map(|i| i.notes.clone())
        .unwrap_or_default();
    let update_has_download = update_info
        .as_ref()
        .map(|i| !i.download_url.is_empty())
        .unwrap_or(false);
    rsx! {
        div { style: "padding: 4px;",
            h4 { class: "section-title", "{tr(language(), \"tools_modal.remote_section_title\")}" }
            p { style: "font-size: 13px; color: var(--text-muted); margin-bottom: 12px;",
                "{remote_description}"
            }
            button {
                class: "btn-primary",
                onclick: move |_| on_toggle_remote.call(()),
                if remote_running {
                    "{tr(language(), \"tools_modal.remote_stop_button\")}"
                } else {
                    "{remote_start_label}"
                }
            }

            h4 { class: "section-title", style: "margin-top: 24px;", "{tr(language(), \"tools_modal.sleep_timer_section_title\")}" }
            if let Some(active_label) = sleep_active_label {
                p { style: "font-size: 13px; color: var(--text-muted); margin-bottom: 8px;",
                    "{active_label}"
                }
                button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_cancel_sleep_timer.call(()), "{tr(language(), \"tools_modal.sleep_timer_cancel\")}" }
            } else {
                div { style: "display: flex; gap: 8px; margin-top: 8px;",
                    button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_set_sleep_timer.call(15), "15 min" }
                    button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_set_sleep_timer.call(30), "30 min" }
                    button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_set_sleep_timer.call(45), "45 min" }
                    button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_set_sleep_timer.call(60), "60 min" }
                }
            }

            h4 { class: "section-title", style: "margin-top: 24px;", "{tr(language(), \"menu.performance\")}" }
            label { style: "display: flex; align-items: center; gap: 8px; cursor: pointer;",
                input {
                    r#type: "checkbox",
                    checked: "{show_metrics}",
                    onchange: move |e| on_toggle_metrics.call(e.value() == "true")
                }
                span { "{tr(language(), \"tools_modal.metrics_toggle\")}" }
            }

            h4 { class: "section-title", style: "margin-top: 24px;", "{tr(language(), \"tools_modal.updates_section_title\")}" }
            label { style: "display: flex; align-items: center; gap: 8px; cursor: pointer; margin-bottom: 10px;",
                input {
                    r#type: "checkbox",
                    checked: "{auto_check_updates}",
                    onchange: move |e| on_toggle_auto_check_updates.call(e.value() == "true")
                }
                span { "{tr(language(), \"tools_modal.auto_check_updates_toggle\")}" }
            }
            div { class: "slider-row",
                span { "{tr(language(), \"tools_modal.update_channel_label\")}" }
                div { style: "display: flex; gap: 8px;",
                    button {
                        class: "{stable_btn_class}",
                        style: "border: 1px solid var(--border-color);",
                        onclick: move |_| on_change_update_channel.call(UpdateChannel::Stable),
                        "Stable"
                    }
                    button {
                        class: "{beta_btn_class}",
                        style: "border: 1px solid var(--border-color);",
                        onclick: move |_| on_change_update_channel.call(UpdateChannel::Beta),
                        "Beta"
                    }
                }
            }
            div { class: "control-group-col", style: "margin-top: 12px;",
                if update_channel == UpdateChannel::Stable {
                    input { class: "select-input", r#type: "text", value: "{update_manifest_url_stable}", onchange: move |e| on_change_manifest_url.call((UpdateChannel::Stable, e.value())) }
                } else {
                    input { class: "select-input", r#type: "text", value: "{update_manifest_url_beta}", onchange: move |e| on_change_manifest_url.call((UpdateChannel::Beta, e.value())) }
                }
            }
            div { style: "margin-top: 16px; display: flex; align-items: center; gap: 12px;",
                button {
                    class: "btn-primary",
                    onclick: move |_| on_check_updates.call(()),
                    "{tr(language(), \"tools_modal.check_updates_button\")}"
                }
                if !update_status.is_empty() {
                    span { style: "font-size: 13px; color: var(--text-muted);", "{update_status}" }
                }
            }
            if let Some(info) = update_info {
                div { class: "info-table", style: "margin-top: 16px;",
                    div { class: "info-row", span { class: "info-label", "{tr(language(), \"tools_modal.update_version\")}" } span { "{info.version}" } }
                    if !update_notes.is_empty() {
                        div { class: "info-row", span { class: "info-label", "Notes" } span { "{update_notes}" } }
                    }
                    if update_has_download {
                        div { style: "margin-top: 12px;",
                            button {
                                class: "btn-primary",
                                style: "background-color: var(--accent-color); color: #fff;",
                                onclick: move |_| on_install_update.call(()),
                                "{tr(language(), \"tools_modal.install_update_button\")}"
                            }
                        }
                    }
                }
            }
        }
    }
}
