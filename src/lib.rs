//! RPlayer library crate exposing domain modules.

pub mod playback;
pub mod services;
pub mod storage;
pub mod theme;
pub mod tools;

// Explicit re-exports for root-level module access without glob ambiguity
pub use playback::{
    ab_repeat, chapters, equalizer, image_controls, karaoke, player, sleep_timer, up_next,
};
pub use services::{lastfm, opensubtitles, remote, streaming, updater};
pub use storage::{bookmarks, config, history, notes, playlist};
pub use theme::{i18n, theme_manager};
pub use tools::{converter, media_info, thumbnail, trim};
