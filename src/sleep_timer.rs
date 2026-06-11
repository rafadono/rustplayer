//! sleep_timer.rs — Timer to pause or close playback automatically.

use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum SleepAction {
    Pause,
    Stop,
    Quit,
}

impl SleepAction {
    pub fn label(&self) -> &str {
        match self {
            Self::Pause => "Pausar",
            Self::Stop => "Detener",
            Self::Quit => "Cerrar aplicación",
        }
    }
}

pub struct SleepTimer {
    pub enabled: bool,
    pub action: SleepAction,
    pub duration: Duration,
    started_at: Option<Instant>,
    pub fired: bool,
}

impl SleepTimer {
    pub fn new() -> Self {
        Self {
            enabled: false,
            action: SleepAction::Pause,
            duration: Duration::from_secs(30 * 60), // 30 min by default
            started_at: None,
            fired: false,
        }
    }

    pub fn start(&mut self) {
        self.enabled = true;
        self.started_at = Some(Instant::now());
        self.fired = false;
    }

    pub fn cancel(&mut self) {
        self.enabled = false;
        self.started_at = None;
        self.fired = false;
    }

    /// Call each frame. Returns true when it should fire (only once).
    pub fn tick(&mut self) -> bool {
        if !self.enabled || self.fired {
            return false;
        }
        if let Some(start) = self.started_at {
            if start.elapsed() >= self.duration {
                self.fired = true;
                self.enabled = false;
                return true;
            }
        }
        false
    }

    pub fn remaining(&self) -> Option<Duration> {
        if !self.enabled {
            return None;
        }
        let elapsed = self.started_at?.elapsed();
        self.duration.checked_sub(elapsed)
    }

    pub fn set_minutes(&mut self, mins: u64) {
        self.duration = Duration::from_secs(mins * 60);
    }
}
