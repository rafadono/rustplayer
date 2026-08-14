//! player/mod.rs — Complete libmpv wrapper.
//!
//! Displays playback commands, audio/subtitle tracks,
//! chapters, appearance, EQ and screenshots.

pub mod audio_ops;
pub mod events;
pub mod types;
pub mod video_ops;

pub use types::*;

use crossbeam_channel::{bounded, Receiver};
use std::sync::{Arc, Mutex};

pub struct Player {
    mpv: Arc<libmpv2::Mpv>,
    pub state: Arc<Mutex<PlayerState>>,
    event_rx: Receiver<PlayerEvent>,
}

impl Player {
    pub fn mpv(&self) -> Arc<libmpv2::Mpv> {
        Arc::clone(&self.mpv)
    }

    pub fn new(volume: i64, muted: bool, speed: f64) -> Result<Self, libmpv2::Error> {
        let mpv = Arc::new(libmpv2::Mpv::new()?);

        let _ = mpv.set_property("vo", "gpu,auto");
        let _ = mpv.set_property("hwdec", "vaapi,auto-safe");
        let _ = mpv.set_property("force-window", "no");
        let _ = mpv.set_property("keep-open", "always");
        mpv.set_property("audio-display", "no")?;
        mpv.set_property("hr-seek", "yes")?;
        mpv.set_property("cache", "yes")?;
        mpv.set_property("demuxer-max-bytes", "150MiB")?;
        mpv.set_property("volume", volume)?;
        mpv.set_property("mute", muted)?;
        mpv.set_property("speed", speed)?;

        // Screenshots go to ~/Images or ~/Pictures
        let ss_dir = dirs::picture_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_default())
            .join("RPlayer");
        let _ = std::fs::create_dir_all(&ss_dir);
        mpv.set_property("screenshot-directory", ss_dir.to_string_lossy().as_ref())?;
        mpv.set_property("screenshot-format", "png")?;

        let state = Arc::new(Mutex::new(PlayerState {
            volume,
            muted,
            speed,
            ..Default::default()
        }));

        let (tx, rx) = bounded::<PlayerEvent>(128);
        Self::spawn_monitor_thread(Arc::clone(&mpv), Arc::clone(&state), tx);

        Ok(Self {
            mpv,
            state,
            event_rx: rx,
        })
    }

    // ── Public API ──────────────────────────── ────────────────────────────

    pub fn drain_events(&self) -> Vec<PlayerEvent> {
        self.event_rx.try_iter().collect()
    }

    pub fn set_wid(&self, wid: i64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("wid", wid)
    }

    pub fn open(&self, path: &std::path::Path) -> Result<(), libmpv2::Error> {
        let s = path.to_string_lossy();
        self.mpv.command("loadfile", &[s.as_ref()])?;
        self.state.lock().unwrap().current_file = Some(path.to_path_buf());
        Ok(())
    }

    pub fn toggle_pause(&self) -> Result<(), libmpv2::Error> {
        let p: bool = self.mpv.get_property("pause").unwrap_or(false);
        self.mpv.set_property("pause", !p)
    }

    pub fn set_paused(&self, paused: bool) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("pause", paused)
    }

    pub fn seek_absolute(&self, secs: f64) -> Result<(), libmpv2::Error> {
        self.mpv
            .command("seek", &[&format!("{:.3}", secs), "absolute+keyframes"])
    }

    pub fn seek_exact(&self, secs: f64) -> Result<(), libmpv2::Error> {
        self.mpv
            .command("seek", &[&format!("{:.3}", secs), "absolute+exact"])
    }

    pub fn seek_relative(&self, secs: f64) -> Result<(), libmpv2::Error> {
        self.mpv
            .command("seek", &[&format!("{:.3}", secs), "relative+keyframes"])
    }

    pub fn set_volume(&self, vol: i64) -> Result<(), libmpv2::Error> {
        let v = vol.clamp(0, 150);
        self.mpv.set_property("volume", v)?;
        self.state.lock().unwrap().volume = v;
        Ok(())
    }

    pub fn toggle_mute(&self) -> Result<(), libmpv2::Error> {
        let m = !self.state.lock().unwrap().muted;
        self.mpv.set_property("mute", m)?;
        self.state.lock().unwrap().muted = m;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), libmpv2::Error> {
        self.mpv.command("stop", &[])
    }

    pub fn set_speed(&self, speed: f64) -> Result<(), libmpv2::Error> {
        self.mpv.set_property("speed", speed)
    }

    pub fn mpv_handle(&self) -> Arc<libmpv2::Mpv> {
        Arc::clone(&self.mpv)
    }
}

// ── Additional methods (v0.3) ────────────────────── ───────────────────────

impl Player {
    /// Gets the current position of the frame (for A-B and frame step).
    pub fn current_position(&self) -> f64 {
        self.state.lock().unwrap().position
    }

    /// Complete media information of the current file.
    pub fn media_info(&self) -> Option<crate::media_info::MediaInfo> {
        let s = self.state.lock().unwrap();
        let file = s.current_file.clone()?;
        drop(s);
        Some(crate::media_info::MediaInfo::from_mpv(&self.mpv, &file))
    }
}
