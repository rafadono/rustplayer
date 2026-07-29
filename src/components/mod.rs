pub mod controls;
pub mod header;
pub mod modals;
pub mod playlist;
pub mod video;

#[derive(Clone, Debug, PartialEq)]
pub struct TrackItem {
    pub id: i64,
    pub title: String,
    pub lang: String,
    pub track_type: String,
}

pub use controls::PlayerControls;
pub use header::HeaderBar;
pub use modals::*;
pub use playlist::PlaylistPanel;
pub use video::VideoStage;
