//! config.rs — Persistent application configuration.

use crate::equalizer::Equalizer;
use crate::i18n::{tr, Language};
use crate::image_controls::ImageControls;
use crate::lastfm::LastFmConfig;
use crate::theme_manager::ThemeColors;
use crate::updater::UpdateChannel;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepeatMode {
    None,
    One,
    All,
}

impl RepeatMode {
    pub fn label(&self) -> &str {
        self.label_lang(Language::Es)
    }
    pub fn label_lang(&self, language: Language) -> &str {
        match self {
            Self::None => tr(language, "repeat.none"),
            Self::One => tr(language, "repeat.one"),
            Self::All => tr(language, "repeat.all"),
        }
    }
    pub fn icon(&self) -> &str {
        match self {
            Self::None => "OFF",
            Self::One => "1",
            Self::All => "ALL",
        }
    }
    pub fn next(&self) -> Self {
        match self {
            Self::None => Self::All,
            Self::All => Self::One,
            Self::One => Self::None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // Reproduction
    pub volume: i64,
    pub muted: bool,
    pub speed: f64,
    pub aspect_ratio: AspectRatio,
    pub repeat_mode: RepeatMode,
    pub shuffle: bool,

    // Audio/video sync
    pub audio_delay: f64,
    pub sub_delay: f64,

    // Image
    pub image_controls: ImageControls,

    // UI
    pub show_donation_banner: bool,
    pub show_playlist: bool,
    pub last_directory: Option<PathBuf>,
    pub theme: ThemeColors,

    // Features
    pub equalizer: Equalizer,
    pub remote_enabled: bool,
    pub remote_port: u16,
    pub lastfm: LastFmConfig,
    #[serde(default)]
    pub show_metrics_overlay: bool,
    #[serde(default = "default_metrics_overlay_opacity")]
    pub metrics_overlay_opacity: f32,
    #[serde(default = "default_metrics_overlay_font_size")]
    pub metrics_overlay_font_size: f32,
    #[serde(default = "default_update_channel")]
    pub update_channel: UpdateChannel,
    #[serde(default = "default_auto_check_updates")]
    pub auto_check_updates: bool,
    #[serde(default = "default_language")]
    pub language: Language,
    #[serde(default)]
    pub bug_report_url: String,

    // Subtitles
    pub sub_font_size: i64,
    pub sub_color: String,
    #[serde(default = "default_sub_opacity")]
    pub sub_opacity: f32,
    #[serde(default = "default_sub_font_family")]
    pub sub_font_family: String,
    #[serde(default)]
    pub sub_bold: bool,
    pub loudnorm: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AspectRatio {
    Auto,
    Ratio4_3,
    Ratio16_9,
    Ratio21_9,
    Ratio1_1,
    Custom(f64),
}

impl AspectRatio {
    pub fn label(&self) -> &str {
        match self {
            Self::Auto => "Auto",
            Self::Ratio4_3 => "4:3",
            Self::Ratio16_9 => "16:9",
            Self::Ratio21_9 => "21:9",
            Self::Ratio1_1 => "1:1",
            Self::Custom(_) => "Personalizado",
        }
    }
    pub fn to_mpv_value(&self) -> String {
        match self {
            Self::Auto => "-1".into(),
            Self::Ratio4_3 => "4/3".into(),
            Self::Ratio16_9 => "16/9".into(),
            Self::Ratio21_9 => "21/9".into(),
            Self::Ratio1_1 => "1/1".into(),
            Self::Custom(r) => format!("{:.4}", r),
        }
    }
    pub fn all() -> &'static [AspectRatio] {
        &[
            AspectRatio::Auto,
            AspectRatio::Ratio16_9,
            AspectRatio::Ratio4_3,
            AspectRatio::Ratio21_9,
            AspectRatio::Ratio1_1,
        ]
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            volume: 100,
            muted: false,
            speed: 1.0,
            aspect_ratio: AspectRatio::Auto,
            repeat_mode: RepeatMode::None,
            shuffle: false,
            audio_delay: 0.0,
            sub_delay: 0.0,
            image_controls: ImageControls::default(),
            show_donation_banner: true,
            show_playlist: true,
            last_directory: None,
            theme: ThemeColors::default(),
            equalizer: Equalizer::default(),
            remote_enabled: false,
            remote_port: 7890,
            lastfm: LastFmConfig::default(),
            show_metrics_overlay: false,
            metrics_overlay_opacity: default_metrics_overlay_opacity(),
            metrics_overlay_font_size: default_metrics_overlay_font_size(),
            update_channel: UpdateChannel::Stable,
            auto_check_updates: true,
            language: Language::Es,
            bug_report_url: String::new(),
            sub_font_size: 40,
            sub_color: "#FFFFFF".into(),
            sub_opacity: default_sub_opacity(),
            sub_font_family: default_sub_font_family(),
            sub_bold: false,
            loudnorm: false,
        }
    }
}

fn default_update_channel() -> UpdateChannel {
    UpdateChannel::Stable
}

fn default_language() -> Language {
    Language::Es
}

fn default_auto_check_updates() -> bool {
    true
}

fn default_metrics_overlay_opacity() -> f32 {
    0.72
}

fn default_metrics_overlay_font_size() -> f32 {
    11.0
}

fn default_sub_opacity() -> f32 {
    1.0
}

fn default_sub_font_family() -> String {
    "sans-serif".to_string()
}

impl Config {
    fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("rplayer").join("config.json"))
    }
    pub fn load() -> Self {
        let Some(path) = Self::path() else {
            return Self::default();
        };
        let Ok(data) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&data).unwrap_or_default()
    }
    pub fn save(&self) {
        let Some(path) = Self::path() else { return };
        if let Some(p) = path.parent() {
            let _ = std::fs::create_dir_all(p);
        }
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, data);
        }
    }
}
