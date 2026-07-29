//! lastfm.rs — Scrobbling to Last.fm.
//!
//! flow:
//!   1. auth::get_session(user, password) → session_key (save to config)
//!   2. When a track starts: now_playing(session_key, track)
//!   3. When playing >30s or >50%: scrobble(session_key, track, timestamp)
//!
//! To use: create your own API key at https://www.last.fm/api/account/create
//! and replace the values ​​in the api_key()/api_secret() functions below.

use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

// ──API Keys ───────────────────────────────── ─────────────────────────────────
//
// Resolution order:
//   1. RUSTPLAYER_LASTFM_KEY / RUSTPLAYER_LASTFM_SECRET environment variables
//   2. Compiled value (obfuscated with obfstr — does not appear in `binary strings`)
//
// To distribute with your own keys, define the environment variables
// BEFORE building with `cargo build --release`, or edit the values ​​of obfstr!()
// and recompile. The keys do NOT appear in plain text in the resulting binary.

fn api_key() -> String {
    std::env::var("RUSTPLAYER_LASTFM_KEY")
        .unwrap_or_else(|_| obfstr::obfstr!("REEMPLAZAR_CON_TU_API_KEY").to_string())
}

fn api_secret() -> String {
    std::env::var("RUSTPLAYER_LASTFM_SECRET")
        .unwrap_or_else(|_| obfstr::obfstr!("REEMPLAZAR_CON_TU_API_SECRET").to_string())
}

fn api_url() -> String {
    obfstr::obfstr!("https://ws.audioscrobbler.com/2.0/").to_string()
}

const MIN_SCROBBLE_SECS: f64 = 30.0;

// ── Settings ───────────────────────────────── ──────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LastFmConfig {
    pub enabled: bool,
    pub username: String,
    pub session_key: String,
}

// ── Track info ──────────────────────────────── ────────────────────────────────

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub artist: String,
    pub title: String,
    pub album: Option<String>,
}

impl TrackInfo {
    /// Extract artist/title from file name ("Artist - Title.mp3").
    pub fn from_filename(name: &str) -> Self {
        // Remove extension
        let base = std::path::Path::new(name)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| name.to_string());

        if let Some((artist, title)) = base.split_once(" - ") {
            Self {
                artist: artist.trim().to_string(),
                title: title.trim().to_string(),
                album: None,
            }
        } else {
            Self {
                artist: "Unknown".into(),
                title: base,
                album: None,
            }
        }
    }
}

// ── API signing ─────────────────────────────── ───────────────────────────────

/// Generates the MD5 signature required by Last.fm.
///params must NOT include "api_sig" or "format".
fn sign(params: &BTreeMap<&str, String>) -> String {
    let mut data = String::new();
    for (k, v) in params {
        data.push_str(k);
        data.push_str(v);
    }
    data.push_str(&api_secret());

    format!("{:x}", md5::compute(data.as_bytes()))
}

fn post(params: &mut BTreeMap<&str, String>) -> Result<String, String> {
    let key = api_key();
    params.insert("api_key", key);
    params.insert("format", "json".into());
    let sig = sign(params);
    params.insert("api_sig", sig);

    let body: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, urlencoded(v)))
        .collect::<Vec<_>>()
        .join("&");

    ureq::post(&api_url())
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .map_err(|e| e.to_string())?
        .into_string()
        .map_err(|e| e.to_string())
}

fn urlencoded(s: &str) -> String {
    // Simple Encoding Sufficient for Last.fm Fields
    s.chars()
        .flat_map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                vec![c]
            }
            ' ' => vec!['+'],
            other => format!("%{:02X}", other as u32).chars().collect(),
        })
        .collect()
}

// ──Now playing ───────────────────────────────── ─────────────────────────────────

pub fn now_playing(session_key: &str, track: &TrackInfo) {
    if session_key.is_empty() || api_key() == obfstr::obfstr!("REEMPLAZAR_CON_TU_API_KEY") {
        return;
    }

    let sk = session_key.to_string();
    let artist = track.artist.clone();
    let title = track.title.clone();
    let album = track.album.clone();

    thread::Builder::new()
        .name("lastfm-nowplaying".into())
        .spawn(move || {
            let mut params = BTreeMap::new();
            params.insert("method", "track.updateNowPlaying".to_string());
            params.insert("artist", artist);
            params.insert("track", title);
            params.insert("sk", sk);
            if let Some(a) = album {
                params.insert("album", a);
            }

            match post(&mut params) {
                Ok(_) => debug!("lastfm: nowplaying enviado"),
                Err(e) => error!("lastfm nowplaying error: {}", e),
            }
        })
        .ok();
}

// Scrobble ─────────────────────────────────

/// Scrobbling status manager for the current file.
pub struct ScrobbleTracker {
    pub track: Option<TrackInfo>,
    pub started_at: u64,
    pub played_secs: f64,
    pub scrobbled: bool,
    pub now_playing_sent: bool,
}

impl ScrobbleTracker {
    pub fn new() -> Self {
        Self {
            track: None,
            started_at: 0,
            played_secs: 0.0,
            scrobbled: false,
            now_playing_sent: false,
        }
    }

    pub fn start_track(&mut self, track: TrackInfo, session_key: &str) {
        self.track = Some(track.clone());
        self.started_at = unix_now();
        self.played_secs = 0.0;
        self.scrobbled = false;
        self.now_playing_sent = false;

        // Send now_playing immediately
        now_playing(session_key, &track);
        self.now_playing_sent = true;
    }

    /// Call periodically with accumulated played (not paused) seconds.
    /// Returns true if scrobble has just been completed.
    pub fn tick(&mut self, played_secs: f64, duration: f64, session_key: &str) -> bool {
        self.played_secs = played_secs;

        if self.scrobbled || session_key.is_empty() {
            return false;
        }

        let threshold = if duration > 0.0 {
            (duration * 0.5).min(240.0).max(MIN_SCROBBLE_SECS)
        } else {
            MIN_SCROBBLE_SECS
        };

        if played_secs >= threshold {
            if let Some(track) = &self.track {
                scrobble(session_key, track, self.started_at);
                self.scrobbled = true;
                return true;
            }
        }
        false
    }
}

fn scrobble(session_key: &str, track: &TrackInfo, timestamp: u64) {
    let sk = session_key.to_string();
    let artist = track.artist.clone();
    let title = track.title.clone();
    let album = track.album.clone();

    thread::Builder::new()
        .name("lastfm-scrobble".into())
        .spawn(move || {
            let mut params = BTreeMap::new();
            params.insert("method", "track.scrobble".to_string());
            params.insert("artist[0]", artist);
            params.insert("track[0]", title);
            params.insert("timestamp[0]", timestamp.to_string());
            params.insert("sk", sk);
            if let Some(a) = album {
                params.insert("album[0]", a);
            }

            match post(&mut params) {
                Ok(_) => debug!("lastfm: scrobble enviado"),
                Err(e) => error!("lastfm scrobble error: {}", e),
            }
        })
        .ok();
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Authenticates the user using credentials and returns the session_key.
pub fn get_session(username: &str, password: &str) -> Result<String, String> {
    if api_key() == obfstr::obfstr!("REEMPLAZAR_CON_TU_API_KEY") {
        return Err("API key no configurada".to_string());
    }

    let mut params = BTreeMap::new();
    params.insert("method", "auth.getMobileSession".to_string());
    params.insert("username", username.to_string());
    params.insert("password", password.to_string());

    let res_text = post(&mut params)?;

    #[derive(Deserialize)]
    struct SessionData {
        key: String,
    }
    #[derive(Deserialize)]
    struct MobileSessionResponse {
        session: Option<SessionData>,
        message: Option<String>,
    }

    let parsed: MobileSessionResponse =
        serde_json::from_str(&res_text).map_err(|e| format!("Error al parsear JSON: {}", e))?;

    if let Some(session) = parsed.session {
        Ok(session.key)
    } else if let Some(msg) = parsed.message {
        Err(msg)
    } else {
        Err("Error de autenticación desconocido".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_session_not_configured() {
        let res = get_session("user", "pass");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "API key no configurada");
    }
}
