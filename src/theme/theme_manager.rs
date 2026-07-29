//! theme_manager.rs — Predefined color palettes and custom editor.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThemePreset {
    DarkBlue, // default
    DarkGreen,
    DarkPurple,
    DarkOrange,
    Light,
    LightWarm,
    Mocha,
    Solarized,
    Nord,
    Custom,
}

impl ThemePreset {
    pub fn label(&self) -> &str {
        match self {
            Self::DarkBlue => "Dark Blue (predeterminado)",
            Self::DarkGreen => "Dark Green",
            Self::DarkPurple => "Dark Purple",
            Self::DarkOrange => "Dark Orange",
            Self::Light => "Light",
            Self::LightWarm => "Light Warm",
            Self::Mocha => "Mocha",
            Self::Solarized => "Solarized Dark",
            Self::Nord => "Nord",
            Self::Custom => "Personalizado",
        }
    }

    pub fn all() -> &'static [ThemePreset] {
        &[
            ThemePreset::DarkBlue,
            ThemePreset::DarkGreen,
            ThemePreset::DarkPurple,
            ThemePreset::DarkOrange,
            ThemePreset::Light,
            ThemePreset::LightWarm,
            ThemePreset::Mocha,
            ThemePreset::Solarized,
            ThemePreset::Nord,
            ThemePreset::Custom,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub preset: ThemePreset,
    pub bg: [u8; 3],
    pub surface: [u8; 3],
    pub accent: [u8; 3],
    pub text: [u8; 3],
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::from_preset(&ThemePreset::DarkBlue)
    }
}

impl ThemeColors {
    pub fn from_preset(preset: &ThemePreset) -> Self {
        let (bg, surface, accent, text) = match preset {
            ThemePreset::DarkBlue => ([14, 14, 18], [22, 22, 28], [99, 179, 237], [220, 220, 230]),
            ThemePreset::DarkGreen => ([12, 16, 14], [18, 28, 22], [72, 199, 142], [210, 230, 215]),
            ThemePreset::DarkPurple => {
                ([14, 12, 20], [22, 18, 34], [160, 110, 240], [220, 215, 235])
            }
            ThemePreset::DarkOrange => {
                ([18, 14, 10], [28, 22, 16], [240, 140, 60], [235, 225, 210])
            }
            ThemePreset::Light => (
                [245, 245, 250],
                [255, 255, 255],
                [30, 100, 200],
                [20, 20, 30],
            ),
            ThemePreset::LightWarm => (
                [250, 248, 242],
                [255, 253, 248],
                [180, 80, 30],
                [50, 35, 20],
            ),
            ThemePreset::Mocha => ([24, 20, 18], [36, 30, 28], [225, 160, 120], [230, 215, 205]),
            ThemePreset::Solarized => ([0, 43, 54], [7, 54, 66], [38, 139, 210], [147, 161, 161]),
            ThemePreset::Nord => ([46, 52, 64], [59, 66, 82], [136, 192, 208], [216, 222, 233]),
            ThemePreset::Custom => ([14, 14, 18], [22, 22, 28], [99, 179, 237], [220, 220, 230]),
        };
        Self {
            preset: preset.clone(),
            bg,
            surface,
            accent,
            text,
        }
    }

    pub fn is_light(&self) -> bool {
        let lum = self.bg[0] as f32 * 0.299 + self.bg[1] as f32 * 0.587 + self.bg[2] as f32 * 0.114;
        lum > 128.0
    }
}

#[cfg(test)]
mod tests {
    use super::{ThemeColors, ThemePreset};

    #[test]
    fn presets_return_expected_lightness() {
        let dark = ThemeColors::from_preset(&ThemePreset::DarkBlue);
        let light = ThemeColors::from_preset(&ThemePreset::Light);
        assert!(!dark.is_light());
        assert!(light.is_light());
    }
}
