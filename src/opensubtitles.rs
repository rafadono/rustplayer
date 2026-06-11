//! opensubtitles.rs — Automatic subtitle download via OpenSubtitles.org v2 REST API.
//!
//! Endpoints used:
//!   POST https://api.opensubtitles.com/api/v1/login → token
//!   GET https://api.opensubtitles.com/api/v1/subtitles?imdb_id=...&languages=es
//!   GET direct download link
//!
//! Create a free account and API key at: https://www.opensubtitles.com/en/consumers
//! Override api_key() below.

use crossbeam_channel::{bounded, Receiver};
use log::debug;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::thread;

// ── API Keys and URLs ───────────────────────────── ──────────────────────────────
//
// Resolution order:
//   1. RUSTPLAYER_OPENSUBS_KEY environment variable
//   2. Compiled value (obfuscated — does not appear in `binary strings`)

fn api_key() -> String {
    std::env::var("RUSTPLAYER_OPENSUBS_KEY")
        .unwrap_or_else(|_| obfstr::obfstr!("REEMPLAZAR_CON_TU_OPENSUBTITLES_API_KEY").to_string())
}

fn app_name() -> String {
    obfstr::obfstr!("RPlayer v0.5.0-alpha").to_string()
}

fn base_url() -> String {
    obfstr::obfstr!("https://api.opensubtitles.com/api/v1").to_string()
}

#[derive(Debug, Clone)]
pub struct SubResult {
    pub language: String,
    pub title: String,
    pub hearing_impaired: bool,
    pub download_url: String,
    pub release: String,
    pub file_id: u32,
}

#[derive(Debug, Clone)]
pub enum SubSearchStatus {
    Searching,
    Results(Vec<SubResult>),
    Downloading,
    Done(PathBuf),
    Error(String),
}

pub struct SubSearchJob {
    pub rx: Receiver<SubSearchStatus>,
}

impl SubSearchJob {
    /// Searches for subtitles for the given file and language (e.g. "es", "en").
    pub fn search(file: &Path, language: &str) -> Option<Self> {
        if api_key() == obfstr::obfstr!("REEMPLAZAR_CON_TU_OPENSUBTITLES_API_KEY") {
            return None; // API key not configured, not spawning
        }

        let (tx, rx) = bounded(8);
        let filename = file
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let lang = language.to_string();
        let _ = tx.try_send(SubSearchStatus::Searching);

        thread::Builder::new()
            .name("opensubs-search".into())
            .spawn(move || match search_subtitles(&filename, &lang) {
                Ok(results) => {
                    let _ = tx.send(SubSearchStatus::Results(results));
                }
                Err(e) => {
                    let _ = tx.send(SubSearchStatus::Error(e));
                }
            })
            .ok()?;

        Some(SubSearchJob { rx })
    }

    /// Starts downloading the selected subtitle to the `dest_dir` directory.
    pub fn download(result: &SubResult, dest_dir: &Path) -> Option<Self> {
        let (tx, rx) = bounded(4);
        let file_id = result.file_id;
        let lang = result.language.clone();
        let dest = dest_dir.to_path_buf();
        let _ = tx.try_send(SubSearchStatus::Downloading);

        thread::Builder::new()
            .name("opensubs-download".into())
            .spawn(move || match download_file(file_id, &dest, &lang) {
                Ok(path) => {
                    let _ = tx.send(SubSearchStatus::Done(path));
                }
                Err(e) => {
                    let _ = tx.send(SubSearchStatus::Error(e));
                }
            })
            .ok()?;

        Some(SubSearchJob { rx })
    }
}

// ── API calls ──────────────────────────────── ─────────────────────────────────

#[derive(Deserialize)]
struct SearchResponse {
    data: Vec<SubData>,
}
#[derive(Deserialize)]
struct SubData {
    attributes: SubAttributes,
}
#[derive(Deserialize)]
struct SubAttributes {
    language: String,
    release: Option<String>,
    hearing_impaired: Option<bool>,
    #[serde(default)]
    files: Vec<SubFile>,
    feature_details: Option<FeatureDetails>,
}
#[derive(Deserialize)]
struct SubFile {
    file_id: u32,
}
#[derive(Deserialize)]
struct FeatureDetails {
    movie_name: Option<String>,
}

fn search_subtitles(query: &str, lang: &str) -> Result<Vec<SubResult>, String> {
    let url = format!(
        "{}/subtitles?query={}&languages={}&per_page=20",
        base_url(),
        urlenc(query),
        lang
    );

    debug!("opensubtitles search: {}", url);

    let resp = ureq::get(&url)
        .set("Api-Key", &api_key())
        .set("User-Agent", &app_name())
        .call()
        .map_err(|e| e.to_string())?
        .into_string()
        .map_err(|e| e.to_string())?;

    let parsed: SearchResponse =
        serde_json::from_str(&resp).map_err(|e| format!("parse error: {e}"))?;

    let results = parsed
        .data
        .into_iter()
        .filter(|d| !d.attributes.files.is_empty())
        .map(|d| {
            let title = d
                .attributes
                .feature_details
                .and_then(|f| f.movie_name)
                .unwrap_or_else(|| query.to_string());
            let file_id = d.attributes.files[0].file_id;
            SubResult {
                language: d.attributes.language,
                title,
                hearing_impaired: d.attributes.hearing_impaired.unwrap_or(false),
                release: d.attributes.release.unwrap_or_default(),
                download_url: format!("{}/download", base_url()),
                file_id,
            }
        })
        .collect();

    Ok(results)
}

fn download_file(file_id: u32, dest_dir: &Path, lang: &str) -> Result<PathBuf, String> {
    // 1. POST to /download to negotiate the temporary download link
    let download_url = format!("{}/download", base_url());
    let request_body = serde_json::json!({
        "file_id": file_id
    });
    let request_body_str = serde_json::to_string(&request_body)
        .map_err(|e| format!("Error serializando JSON de descarga: {}", e))?;

    let resp = ureq::post(&download_url)
        .set("Api-Key", &api_key())
        .set("User-Agent", &app_name())
        .set("Content-Type", "application/json")
        .send_string(&request_body_str)
        .map_err(|e| format!("Error en POST de descarga: {}", e))?;

    let resp_str = resp
        .into_string()
        .map_err(|e| format!("Error al obtener cuerpo de respuesta: {}", e))?;

    #[derive(Deserialize)]
    struct DownloadResponse {
        link: String,
        file_name: Option<String>,
    }

    let dl_resp: DownloadResponse = serde_json::from_str(&resp_str)
        .map_err(|e| format!("Error parseando JSON de descarga: {}", e))?;

    // 2. GET the temporary link to retrieve the subtitle file bytes
    let file_resp = ureq::get(&dl_resp.link)
        .set("User-Agent", &app_name())
        .call()
        .map_err(|e| format!("Error descargando archivo de subtítulo: {}", e))?;

    let filename = dl_resp
        .file_name
        .unwrap_or_else(|| format!("subtitle_{}.srt", lang));
    let dest = dest_dir.join(&filename);

    let bytes = {
        let mut buf = Vec::new();
        use std::io::Read;
        file_resp
            .into_reader()
            .read_to_end(&mut buf)
            .map_err(|e| e.to_string())?;
        buf
    };

    std::fs::write(&dest, bytes).map_err(|e| e.to_string())?;
    debug!("subtítulo descargado: {}", dest.display());
    Ok(dest)
}

fn urlenc(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => vec![c],
            ' ' => vec!['+'],
            other => format!("%{:02X}", other as u32).chars().collect(),
        })
        .collect()
}

/// Returns true if the API key is configured.
pub fn is_configured() -> bool {
    api_key() != obfstr::obfstr!("REEMPLAZAR_CON_TU_OPENSUBTITLES_API_KEY")
}

///Common languages ​​​​with their ISO 639-1 code
pub fn common_languages() -> &'static [(&'static str, &'static str)] {
    &[
        ("es", "Español"),
        ("en", "Inglés"),
        ("fr", "Francés"),
        ("de", "Alemán"),
        ("pt", "Portugués"),
        ("it", "Italiano"),
        ("ja", "Japonés"),
        ("zh", "Chino"),
        ("ko", "Coreano"),
        ("ru", "Ruso"),
        ("ar", "Ãrabe"),
        ("nl", "Holandés"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opensubtitles_not_configured() {
        assert!(!is_configured());
    }
}
