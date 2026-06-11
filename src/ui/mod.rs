pub mod audio_tracks;
pub mod bookmarks_panel;
pub mod chapters_panel;
pub mod codec_diagnostics_panel;
pub mod context_menu;
pub mod controls;
pub mod converter_panel;
pub mod equalizer_panel;
pub mod history_panel;
pub mod image_controls_panel;
pub mod karaoke_panel;
pub mod library_panel;
pub mod media_info_panel;
pub mod menu;
pub mod notes_panel;
pub mod opensubtitles_panel;
pub mod performance_panel;
pub mod pip;
pub mod playlist_panel;
pub mod settings_panel;
pub mod sleep_timer_panel;
pub mod streaming_panel;
pub mod subtitles;
pub mod sync_panel;
pub mod theme_panel;
pub mod trim_panel;
pub mod up_next_panel;

#[allow(static_mut_refs)]
pub mod theme {
    use egui::Color32;

    pub struct ThemeStatic {
        pub surface: Color32,
        pub surface2: Color32,
        pub accent: Color32,
        pub accent2: Color32,
        pub text: Color32,
        pub muted: Color32,
        pub danger: Color32,
        pub success: Color32,
        pub warning: Color32,
    }

    pub static mut ACTIVE_THEME: ThemeStatic = ThemeStatic {
        surface: Color32::from_rgb(22, 22, 28),
        surface2: Color32::from_rgb(30, 30, 40),
        accent: Color32::from_rgb(99, 179, 237),
        accent2: Color32::from_rgb(79, 209, 197),
        text: Color32::from_rgb(220, 220, 230),
        muted: Color32::from_rgb(100, 100, 115),
        danger: Color32::from_rgb(220, 80, 80),
        success: Color32::from_rgb(72, 199, 142),
        warning: Color32::from_rgb(240, 180, 60),
    };

    #[derive(Copy, Clone)]
    pub struct DynamicColor(pub fn() -> &'static Color32);

    impl std::ops::Deref for DynamicColor {
        type Target = Color32;
        fn deref(&self) -> &Self::Target {
            (self.0)()
        }
    }

    impl From<DynamicColor> for Color32 {
        fn from(d: DynamicColor) -> Self {
            *d
        }
    }

    pub const SURFACE: DynamicColor = DynamicColor(|| unsafe { &ACTIVE_THEME.surface });
    pub const SURFACE2: DynamicColor = DynamicColor(|| unsafe { &ACTIVE_THEME.surface2 });
    pub const ACCENT: DynamicColor = DynamicColor(|| unsafe { &ACTIVE_THEME.accent });
    pub const ACCENT2: DynamicColor = DynamicColor(|| unsafe { &ACTIVE_THEME.accent2 });
    pub const TEXT: DynamicColor = DynamicColor(|| unsafe { &ACTIVE_THEME.text });
    pub const MUTED: DynamicColor = DynamicColor(|| unsafe { &ACTIVE_THEME.muted });
    pub const DANGER: DynamicColor = DynamicColor(|| unsafe { &ACTIVE_THEME.danger });
    pub const SUCCESS: DynamicColor = DynamicColor(|| unsafe { &ACTIVE_THEME.success });
    pub const WARNING: DynamicColor = DynamicColor(|| unsafe { &ACTIVE_THEME.warning });
}
