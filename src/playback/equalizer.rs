//! equalizer.rs - PEQ v1 professional.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EqFilterType {
    Peak,
    LowShelf,
    HighShelf,
    HighPass,
    LowPass,
}

impl EqFilterType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Peak => "Peak",
            Self::LowShelf => "Low Shelf",
            Self::HighShelf => "High Shelf",
            Self::HighPass => "High Pass",
            Self::LowPass => "Low Pass",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PeqFilter {
    pub enabled: bool,
    pub kind: EqFilterType,
    pub freq_hz: f32,
    pub gain_db: f32,
    pub q: f32,
}

impl Default for PeqFilter {
    fn default() -> Self {
        Self {
            enabled: false,
            kind: EqFilterType::Peak,
            freq_hz: 1000.0,
            gain_db: 0.0,
            q: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqProfile {
    pub preamp_db: f32,
    pub anti_clipping: bool,
    pub filters: Vec<PeqFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEqPreset {
    pub name: String,
    pub profile: EqProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Equalizer {
    pub enabled: bool,
    pub preamp_db: f32,
    pub anti_clipping: bool,
    pub peq_filters: Vec<PeqFilter>,
    #[serde(default)]
    pub user_presets: Vec<UserEqPreset>,
    #[serde(default)]
    pub selected_user_preset: Option<usize>,
    #[serde(default)]
    pub preset_name_input: String,
}

impl Default for Equalizer {
    fn default() -> Self {
        Self {
            enabled: false,
            preamp_db: 0.0,
            anti_clipping: true,
            peq_filters: vec![
                PeqFilter {
                    enabled: true,
                    kind: EqFilterType::LowShelf,
                    freq_hz: 90.0,
                    gain_db: 0.0,
                    q: 0.7,
                },
                PeqFilter {
                    enabled: true,
                    kind: EqFilterType::Peak,
                    freq_hz: 250.0,
                    gain_db: 0.0,
                    q: 1.0,
                },
                PeqFilter {
                    enabled: true,
                    kind: EqFilterType::Peak,
                    freq_hz: 700.0,
                    gain_db: 0.0,
                    q: 1.0,
                },
                PeqFilter {
                    enabled: true,
                    kind: EqFilterType::Peak,
                    freq_hz: 2200.0,
                    gain_db: 0.0,
                    q: 1.0,
                },
                PeqFilter {
                    enabled: true,
                    kind: EqFilterType::Peak,
                    freq_hz: 6000.0,
                    gain_db: 0.0,
                    q: 1.0,
                },
                PeqFilter {
                    enabled: true,
                    kind: EqFilterType::HighShelf,
                    freq_hz: 12000.0,
                    gain_db: 0.0,
                    q: 0.7,
                },
            ],
            user_presets: Vec::new(),
            selected_user_preset: None,
            preset_name_input: String::new(),
        }
    }
}

impl Equalizer {
    pub fn preset_flat() -> Self {
        Self::default()
    }

    pub fn preset_bass_boost() -> Self {
        let mut eq = Self::default();
        eq.enabled = true;
        eq.preamp_db = -2.0;
        eq.peq_filters[0].gain_db = 6.0;
        eq.peq_filters[1].gain_db = 3.0;
        eq
    }

    pub fn preset_vocal() -> Self {
        let mut eq = Self::default();
        eq.enabled = true;
        eq.preamp_db = -1.5;
        eq.peq_filters[0].gain_db = -2.0;
        eq.peq_filters[2].gain_db = 2.5;
        eq.peq_filters[3].gain_db = 3.5;
        eq.peq_filters[4].gain_db = 1.5;
        eq
    }

    pub fn preset_cinema() -> Self {
        let mut eq = Self::default();
        eq.enabled = true;
        eq.preamp_db = -1.0;
        eq.peq_filters[0].gain_db = 3.0;
        eq.peq_filters[4].gain_db = 2.0;
        eq.peq_filters[5].gain_db = 1.0;
        eq
    }

    pub fn preset_rock() -> Self {
        let mut eq = Self::default();
        eq.enabled = true;
        eq.preamp_db = -2.0;
        eq.peq_filters[0].gain_db = 4.0;
        eq.peq_filters[2].gain_db = -1.0;
        eq.peq_filters[3].gain_db = 2.0;
        eq.peq_filters[5].gain_db = 3.0;
        eq
    }

    pub fn to_profile(&self) -> EqProfile {
        EqProfile {
            preamp_db: self.preamp_db,
            anti_clipping: self.anti_clipping,
            filters: self.peq_filters.clone(),
        }
    }

    pub fn apply_profile(&mut self, p: &EqProfile) {
        self.preamp_db = p.preamp_db;
        self.anti_clipping = p.anti_clipping;
        self.peq_filters = p.filters.clone();
        while self.peq_filters.len() < 6 {
            self.peq_filters.push(PeqFilter::default());
        }
        self.peq_filters.truncate(6);
        self.enabled = true;
    }

    pub fn save_user_preset(&mut self, name: String) {
        let clean = name.trim().to_string();
        if clean.is_empty() {
            return;
        }
        let profile = self.to_profile();
        if let Some(existing) = self.user_presets.iter_mut().find(|p| p.name == clean) {
            existing.profile = profile;
            return;
        }
        self.user_presets.push(UserEqPreset {
            name: clean,
            profile,
        });
        self.selected_user_preset = Some(self.user_presets.len().saturating_sub(1));
    }

    pub fn load_selected_user_preset(&mut self) {
        if let Some(i) = self.selected_user_preset {
            if let Some(p) = self.user_presets.get(i).cloned() {
                self.apply_profile(&p.profile);
            }
        }
    }

    pub fn delete_selected_user_preset(&mut self) {
        if let Some(i) = self.selected_user_preset {
            if i < self.user_presets.len() {
                self.user_presets.remove(i);
            }
            self.selected_user_preset = if self.user_presets.is_empty() {
                None
            } else {
                Some(i.min(self.user_presets.len() - 1))
            };
        }
    }

    pub fn to_mpv_af_chain(&self, loudnorm: bool) -> String {
        let mut chain: Vec<String> = Vec::new();

        if loudnorm {
            chain.push("lavfi=[loudnorm]".to_string());
        }
        if !self.enabled {
            return chain.join(",");
        }

        if self.preamp_db.abs() > 0.01 {
            chain.push(format!("volume={:.2}dB", self.preamp_db));
        }

        for f in self.peq_filters.iter().filter(|f| f.enabled) {
            let q = f.q.clamp(0.1, 10.0);
            let freq = f.freq_hz.clamp(20.0, 20_000.0);
            let gain = f.gain_db.clamp(-24.0, 24.0);
            let filter = match f.kind {
                EqFilterType::Peak => {
                    format!(
                        "equalizer=f={:.1}:width_type=q:width={:.3}:gain={:.2}",
                        freq, q, gain
                    )
                }
                EqFilterType::LowShelf => {
                    format!("bass=g={:.2}:f={:.1}:w={:.3}", gain, freq, q)
                }
                EqFilterType::HighShelf => {
                    format!("treble=g={:.2}:f={:.1}:w={:.3}", gain, freq, q)
                }
                EqFilterType::HighPass => {
                    format!("highpass=f={:.1}", freq)
                }
                EqFilterType::LowPass => {
                    format!("lowpass=f={:.1}", freq)
                }
            };
            chain.push(filter);
        }

        if self.anti_clipping {
            chain.push("lavfi=[alimiter=limit=0.97]".to_string());
        }

        chain.join(",")
    }
}
