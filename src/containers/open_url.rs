use crate::components::OpenUrlModal;
use crate::state::AppState;
use dioxus::prelude::*;
use rplayer::i18n::tr;
use rplayer::streaming;
use std::path::PathBuf;

#[component]
pub fn AppOpenUrlModal(open_url_fn: EventHandler<PathBuf>) -> Element {
    let state = use_context::<AppState>();
    let language = state.language;
    let mut show_open_url_modal = state.show_open_url_modal;
    let mut open_url_error = state.open_url_error;

    rsx! {
        OpenUrlModal {
            on_close: move |_| show_open_url_modal.set(false),
            error: open_url_error(),
            on_open: move |url: String| {
                if streaming::is_valid_url(&url) {
                    open_url_fn.call(PathBuf::from(url));
                    show_open_url_modal.set(false);
                } else {
                    open_url_error.set(tr(language(), "open_url_modal.invalid_url").to_string());
                }
            },
        }
    }
}
