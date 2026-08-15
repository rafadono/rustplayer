use super::Player;

impl Player {
    pub fn set_aspect_ratio(
        &self,
        ratio: &crate::config::AspectRatio,
    ) -> Result<(), libmpv2::Error> {
        let val = ratio.to_mpv_value();
        self.mpv.set_property("video-aspect-override", val.as_str())
    }

    pub fn set_crop(&self, panscan: f64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("panscan", panscan)
    }

    pub fn set_deinterlace(&self, enable: bool) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("deinterlace", if enable { "yes" } else { "no" })
    }

    pub fn apply_image_controls(&self, ic: &crate::image_controls::ImageControls) {
        ic.apply(&self.mpv);
    }

    pub fn screenshot(&self) -> Result<(), libmpv2::Error> {
        self.mpv.command("screenshot", &["video"])
    }

    pub fn frame_step(&self) -> Result<(), libmpv2::Error> {
        self.mpv.command("frame-step", &[])
    }

    pub fn frame_back_step(&self) -> Result<(), libmpv2::Error> {
        self.mpv.command("frame-back-step", &[])
    }
}
