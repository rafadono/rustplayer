use super::Player;

impl Player {
    pub fn set_audio_track(&self, id: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("aid", id)
    }

    pub fn set_sub_track(&self, id: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sid", id)
    }

    pub fn disable_subs(&self) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sid", "no")
    }

    pub fn add_sub_file(&self, path: &std::path::Path) -> Result<(), libmpv2::Error> {
        self.mpv
            .command("sub-add", &[&path.to_string_lossy(), "select"])
    }

    pub fn set_sub_font_size(&self, size: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sub-font-size", size)
    }

    pub fn set_sub_pos(&self, pos: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sub-pos", pos)
    }

    pub fn set_sub_color_rgb(&self, r: u8, g: u8, b: u8) -> Result<(), libmpv2::Error> {
        let color = format!("#{r:02X}{g:02X}{b:02X}");
        self.mpv.set_property("sub-color", color.as_str())
    }

    pub fn set_sub_opacity(&self, opacity: f32) -> Result<(), libmpv2::Error> {
        self.mpv
            .set_property("sub-opacity", opacity.clamp(0.0, 1.0) as f64)
    }

    pub fn set_sub_font_family(&self, family: &str) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sub-font", family)
    }

    pub fn set_sub_bold(&self, bold: bool) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sub-bold", bold)
    }

    pub fn set_audio_filters(
        &self,
        eq: &crate::equalizer::Equalizer,
        loudnorm: bool,
        karaoke_enabled: bool,
        karaoke_pitch: f64,
    ) {
        let mut af = eq.to_mpv_af_chain(loudnorm);
        let karaoke_af = crate::karaoke::to_mpv_af_chain(karaoke_enabled, karaoke_pitch);
        if !karaoke_af.is_empty() {
            if !af.is_empty() {
                af.push(',');
            }
            af.push_str(&karaoke_af);
        }
        let _ = self.mpv.set_property("af", af.as_str());
    }

    pub fn set_audio_delay(&self, delay: f64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("audio-delay", delay)
    }

    pub fn set_sub_delay(&self, delay: f64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("sub-delay", delay)
    }

    pub fn set_second_sub_track(&self, id: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("secondary-sid", id)
    }

    pub fn disable_second_subs(&self) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("secondary-sid", "no")
    }
}
