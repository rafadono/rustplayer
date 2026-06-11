#![allow(
    dead_code,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::ptr_arg,
    clippy::collapsible_if,
    clippy::field_reassign_with_default,
    clippy::derivable_impls,
    clippy::needless_lifetimes,
    clippy::manual_ignore_case_cmp,
    clippy::manual_clamp,
    clippy::manual_split_once,
    clippy::map_identity,
    clippy::needless_return
)]

pub mod ab_repeat;
pub mod app;
pub mod bookmarks;
pub mod chapters;
pub mod config;
pub mod converter;
pub mod donation;
pub mod equalizer;
pub mod history;
pub mod i18n;
pub mod image_controls;
pub mod karaoke;
pub mod lastfm;
pub mod media_info;
pub mod notes;
pub mod opensubtitles;
pub mod player;
pub mod playlist;
pub mod remote;
pub mod renderer;
pub mod sleep_timer;
pub mod streaming;
pub mod theme_manager;
pub mod thumbnail;
pub mod trim;
pub mod ui;
pub mod up_next;
pub mod updater;
