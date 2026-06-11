//! image_controls.rs — Image controls: brightness, contrast, saturation, hue, gamma.
//! All properties are native to libmpv.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ImageControls {
    pub brightness: i64, // -100..100
    pub contrast: i64,   // -100..100
    pub saturation: i64, // -100..100
    pub hue: i64,        // -100..100
    pub gamma: i64,      // -100..100
    pub zoom: f64,       // 0.0 = normal, positive = zoom in
    pub pan_x: f64,      // -1.0..1.0
    pub pan_y: f64,      // -1.0..1.0
    pub rotation: i64,   // 0, 90, 180, 270
    pub flip_h: bool,
    pub flip_v: bool,
    pub deinterlace: bool,
    pub integer_scaling: bool,
}

impl Default for ImageControls {
    fn default() -> Self {
        Self {
            brightness: 0,
            contrast: 0,
            saturation: 0,
            hue: 0,
            gamma: 0,
            zoom: 0.0,
            pan_x: 0.0,
            pan_y: 0.0,
            rotation: 0,
            flip_h: false,
            flip_v: false,
            deinterlace: false,
            integer_scaling: false,
        }
    }
}

impl ImageControls {
    pub fn is_default(&self) -> bool {
        self.brightness == 0
            && self.contrast == 0
            && self.saturation == 0
            && self.hue == 0
            && self.gamma == 0
            && self.zoom.abs() < 0.001
            && self.pan_x.abs() < 0.001
            && self.pan_y.abs() < 0.001
            && self.rotation == 0
            && !self.flip_h
            && !self.flip_v
            && !self.deinterlace
            && !self.integer_scaling
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn apply(&self, mpv: &libmpv2::Mpv) {
        let _ = mpv.set_property("brightness", self.brightness);
        let _ = mpv.set_property("contrast", self.contrast);
        let _ = mpv.set_property("saturation", self.saturation);
        let _ = mpv.set_property("hue", self.hue);
        let _ = mpv.set_property("gamma", self.gamma);
        let _ = mpv.set_property("video-zoom", self.zoom);
        let _ = mpv.set_property("video-pan-x", self.pan_x);
        let _ = mpv.set_property("video-pan-y", self.pan_y);
        let _ = mpv.set_property("video-rotate", self.rotation);
        let _ = mpv.set_property("deinterlace", self.deinterlace);
        let unscaled = if self.integer_scaling { "yes" } else { "no" };
        let _ = mpv.set_property("video-unscaled", unscaled);

        // Flip via vf
        let vf = match (self.flip_h, self.flip_v) {
            (true, true) => "vflip,hflip",
            (true, false) => "hflip",
            (false, true) => "vflip",
            (false, false) => "",
        };
        let _ = mpv.set_property("vf", vf);
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 90) % 360;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation = ((self.rotation - 90) + 360) % 360;
    }
}
