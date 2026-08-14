use dioxus::prelude::*;
use rplayer::i18n::{tr, Language};
use rplayer::notes::Note;

#[component]
pub fn NotesTab(
    notes: Vec<Note>,
    has_current_file: bool,
    on_add_note: EventHandler<String>,
    on_delete_note: EventHandler<u64>,
    on_export_notes: EventHandler<()>,
) -> Element {
    let language = use_context::<Signal<Language>>();
    let mut note_input = use_signal(String::new);

    rsx! {
        div { class: "control-group-col",
            div { style: "display: flex; gap: 8px; align-items: flex-start;",
                input {
                    class: "select-input",
                    style: "flex: 1;",
                    r#type: "text",
                    placeholder: "{tr(language(), \"tools_modal.notes_placeholder\")}",
                    value: "{note_input}",
                    disabled: !has_current_file,
                    oninput: move |e| note_input.set(e.value()),
                }
                button {
                    class: "btn-primary",
                    disabled: !has_current_file || note_input.read().trim().is_empty(),
                    onclick: move |_| {
                        let text = note_input.read().trim().to_string();
                        if !text.is_empty() {
                            on_add_note.call(text);
                            note_input.set(String::new());
                        }
                    },
                    "{tr(language(), \"tools_modal.notes_add_button\")}"
                }
            }
            if notes.is_empty() {
                div { style: "padding: 20px; text-align: center; color: var(--text-muted); font-size: 13px;",
                    "{tr(language(), \"tools_modal.notes_empty\")}"
                }
            }
            div { class: "tracks-list", style: "margin-top: 12px;",
                for note in notes.iter() {
                    {
                        let note_id = note.id;
                        let pos = note.position;
                        let text = note.text.clone();
                        rsx! {
                            div { key: "{note.id}", class: "track-item",
                                div { style: "display: flex; gap: 8px; align-items: center;",
                                    span { style: "font-family: monospace; color: var(--accent-color);", "{pos:.1}s" }
                                    span { "{text}" }
                                }
                                button { class: "btn-icon", onclick: move |_| on_delete_note.call(note_id), "🗑️" }
                            }
                        }
                    }
                }
            }
            if !notes.is_empty() {
                div { style: "margin-top: 12px;",
                    button { class: "btn-icon", style: "border: 1px solid var(--border-color);", onclick: move |_| on_export_notes.call(()), "{tr(language(), \"tools_modal.notes_export_button\")}" }
                }
            }
        }
    }
}
