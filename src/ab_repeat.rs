//! ab_repeat.rs — Repetition between two points A and B.
//! mpv exposes ab-loop-a/ab-loop-b as direct properties.

#[derive(Debug, Clone, Default)]
pub struct AbRepeat {
    pub a: Option<f64>,
    pub b: Option<f64>,
    pub active: bool,
}

impl AbRepeat {
    pub fn set_a(&mut self, pos: f64, mpv: &libmpv2::Mpv) {
        self.a = Some(pos);
        let _ = mpv.set_property("ab-loop-a", pos);
        self.active = self.a.is_some() && self.b.is_some();
    }

    pub fn set_b(&mut self, pos: f64, mpv: &libmpv2::Mpv) {
        self.b = Some(pos);
        let _ = mpv.set_property("ab-loop-b", pos);
        self.active = self.a.is_some() && self.b.is_some();
    }

    pub fn clear(&mut self, mpv: &libmpv2::Mpv) {
        self.a = None;
        self.b = None;
        self.active = false;
        let _ = mpv.set_property("ab-loop-a", "no");
        let _ = mpv.set_property("ab-loop-b", "no");
    }

    /// Cycle A → B → clean with one key
    pub fn cycle(&mut self, pos: f64, mpv: &libmpv2::Mpv) {
        match (self.a, self.b) {
            (None, _) => self.set_a(pos, mpv),
            (Some(_), None) => self.set_b(pos, mpv),
            _ => self.clear(mpv),
        }
    }

    pub fn label(&self) -> String {
        match (self.a, self.b) {
            (None, _) => "A-B".into(),
            (Some(a), None) => format!("A={:.1}s → ?", a),
            (Some(a), Some(b)) => format!("A={:.1}s B={:.1}s", a, b),
        }
    }
}
