//! theme_manager.rs — Predefined color palettes and custom editor.

use egui::Color32;
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

    pub fn bg_color(&self) -> Color32 {
        Color32::from_rgb(self.bg[0], self.bg[1], self.bg[2])
    }
    pub fn surface_color(&self) -> Color32 {
        Color32::from_rgb(self.surface[0], self.surface[1], self.surface[2])
    }
    pub fn accent_color(&self) -> Color32 {
        Color32::from_rgb(self.accent[0], self.accent[1], self.accent[2])
    }
    pub fn text_color(&self) -> Color32 {
        Color32::from_rgb(self.text[0], self.text[1], self.text[2])
    }

    pub fn surface2_color(&self) -> Color32 {
        // surface + 8 on each channel
        Color32::from_rgb(
            self.surface[0].saturating_add(8),
            self.surface[1].saturating_add(8),
            self.surface[2].saturating_add(8),
        )
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        let bg = self.bg_color();
        let surface = self.surface_color();
        let surface2 = self.surface2_color();
        let accent = self.accent_color();
        let text = self.text_color();

        unsafe {
            crate::ui::theme::ACTIVE_THEME = crate::ui::theme::ThemeStatic {
                surface,
                surface2,
                accent,
                accent2: Color32::from_rgb(
                    self.accent[0].saturating_add(30),
                    self.accent[1].saturating_add(30),
                    self.accent[2].saturating_add(30),
                ),
                text,
                muted: text.gamma_multiply(0.65),
                danger: Color32::from_rgb(220, 80, 80),
                success: Color32::from_rgb(72, 199, 142),
                warning: Color32::from_rgb(240, 180, 60),
            };
        }

        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        style.visuals.extreme_bg_color = bg;
        style.visuals.widgets.noninteractive.bg_fill = surface;
        style.visuals.widgets.inactive.bg_fill = surface2;
        style.visuals.widgets.hovered.bg_fill = surface2;
        style.visuals.widgets.active.bg_fill = accent;
        style.visuals.override_text_color = Some(text);
        style.visuals.selection.bg_fill = accent.gamma_multiply(0.4);

        // Light mode: change borders and shadows
        if self.is_light() {
            style.visuals.window_shadow = egui::epaint::Shadow {
                blur: 8.0,
                spread: 2.0,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 30),
                offset: egui::vec2(0.0, 2.0),
            };
        }

        ctx.set_style(style);
    }

    pub fn is_light(&self) -> bool {
        // bg relative luminance
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
