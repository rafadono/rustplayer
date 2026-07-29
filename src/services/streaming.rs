//! streaming.rs - Reproduction of URLs, radios and podcasts.

use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug, Clone, PartialEq)]
pub enum UrlType {
    DirectStream,
    YtDlp,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct StreamRequest {
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RadioStation {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct PodcastEpisode {
    pub title: String,
    pub url: String,
    pub pub_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PodcastFeed {
    pub title: String,
    pub episodes: Vec<PodcastEpisode>,
}

pub fn classify_url(url: &str) -> UrlType {
    let url = url.trim();
    if url.starts_with("rtmp://")
        || url.starts_with("rtsp://")
        || url.ends_with(".m3u8")
        || url.ends_with(".mpd")
        || url.ends_with(".pls")
        || url.ends_with(".m3u")
    {
        return UrlType::DirectStream;
    }

    if url.starts_with("http://") || url.starts_with("https://") {
        let lower = url.to_lowercase();
        let direct_exts = [
            ".mp4", ".mkv", ".avi", ".webm", ".mov", ".ts", ".mp3", ".flac", ".ogg", ".wav",
            ".aac", ".m4a", ".opus",
        ];
        if direct_exts.iter().any(|e| lower.ends_with(e)) {
            return UrlType::DirectStream;
        }
        if lower.contains("youtube.com")
            || lower.contains("youtu.be")
            || lower.contains("vimeo.com")
            || lower.contains("twitch.tv")
            || lower.contains("dailymotion.com")
            || lower.contains("soundcloud.com")
            || lower.contains("bandcamp.com")
        {
            return UrlType::YtDlp;
        }
        return UrlType::YtDlp;
    }

    UrlType::Unknown
}

pub fn ytdlp_available() -> bool {
    which::which("yt-dlp").is_ok()
}

pub fn ffmpeg_available() -> bool {
    which::which("ffmpeg").is_ok()
}

pub fn is_valid_url(url: &str) -> bool {
    let url = url.trim();
    !url.is_empty()
        && (url.starts_with("http://")
            || url.starts_with("https://")
            || url.starts_with("rtmp://")
            || url.starts_with("rtsp://")
            || url.starts_with("magnet:"))
}

pub fn default_radio_stations() -> Vec<RadioStation> {
    vec![
        RadioStation {
            name: "BBC World Service".to_string(),
            url: "http://stream.live.vc.bbcmedia.co.uk/bbc_world_service".to_string(),
        },
        RadioStation {
            name: "SomaFM Groove Salad".to_string(),
            url: "https://ice2.somafm.com/groovesalad-128-mp3".to_string(),
        },
        RadioStation {
            name: "Radio Paradise Main".to_string(),
            url: "https://stream.radioparadise.com/mp3-192".to_string(),
        },
    ]
}

pub fn fetch_podcast_feed(feed_url: &str) -> Result<PodcastFeed, String> {
    let body = ureq::get(feed_url)
        .call()
        .map_err(|e| format!("No se pudo descargar RSS: {}", e))?
        .into_string()
        .map_err(|e| format!("No se pudo leer RSS: {}", e))?;

    let mut reader = Reader::from_str(&body);
    reader.config_mut().trim_text(true);

    let mut feed_title = String::new();
    let mut episodes: Vec<PodcastEpisode> = Vec::new();
    let mut in_item = false;
    let mut current_title = String::new();
    let mut current_url = String::new();
    let mut current_date: Option<String> = None;
    let mut current_tag = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_tag = tag.clone();
                if tag == "item" {
                    in_item = true;
                    current_title.clear();
                    current_url.clear();
                    current_date = None;
                }
                if in_item && tag == "enclosure" {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"url" {
                            current_url = String::from_utf8_lossy(&attr.value).to_string();
                        }
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                if in_item && e.name().as_ref() == b"enclosure" {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"url" {
                            current_url = String::from_utf8_lossy(&attr.value).to_string();
                        }
                    }
                }
            }
            Ok(Event::Text(t)) => {
                let txt = t.decode().map(|c| c.to_string()).unwrap_or_default();
                if in_item {
                    if current_tag == "title" {
                        current_title = txt;
                    } else if current_tag == "pubDate" {
                        current_date = Some(txt);
                    } else if current_tag == "link" && current_url.is_empty() {
                        current_url = txt;
                    }
                } else if current_tag == "title" && feed_title.is_empty() {
                    feed_title = txt;
                }
            }
            Ok(Event::End(e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "item" {
                    in_item = false;
                    if !current_url.is_empty() {
                        episodes.push(PodcastEpisode {
                            title: if current_title.is_empty() {
                                "Sin titulo".to_string()
                            } else {
                                current_title.clone()
                            },
                            url: current_url.clone(),
                            pub_date: current_date.clone(),
                        });
                    }
                }
                current_tag.clear();
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("RSS invalido: {}", e)),
            _ => {}
        }
    }

    if episodes.is_empty() {
        return Err("No se encontraron episodios en el feed RSS".to_string());
    }

    Ok(PodcastFeed {
        title: if feed_title.is_empty() {
            "Podcast".to_string()
        } else {
            feed_title
        },
        episodes,
    })
}
