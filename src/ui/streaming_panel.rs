//! streaming_panel.rs - Open URLs, radios and podcasts.

use crate::streaming::{
    classify_url, default_radio_stations, fetch_podcast_feed, is_valid_url, ytdlp_available,
    PodcastEpisode, RadioStation, StreamRequest, UrlType,
};
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub struct StreamingPanel {
    pub url_input: String,
    pub podcast_feed_input: String,
    pub podcast_title: String,
    pub podcast_items: Vec<PodcastEpisode>,
    pub radio_stations: Vec<RadioStation>,
    pub error: String,
}

impl StreamingPanel {
    pub fn new() -> Self {
        Self {
            url_input: String::new(),
            podcast_feed_input: String::new(),
            podcast_title: String::new(),
            podcast_items: Vec::new(),
            radio_stations: default_radio_stations(),
            error: String::new(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Option<StreamRequest> {
        let mut open: Option<StreamRequest> = None;

        ui.label(
            RichText::new("Streaming, radio y podcasts")
                .color(TEXT)
                .size(13.0),
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(RichText::new("URL directa").color(ACCENT).size(12.0));
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.url_input)
                        .hint_text("https://... / rtmp://... / m3u8")
                        .desired_width(ui.available_width() - 90.0),
                );
                if ui
                    .add_enabled(
                        is_valid_url(&self.url_input),
                        egui::Button::new(RichText::new("Abrir").color(TEXT)),
                    )
                    .clicked()
                {
                    let url = self.url_input.trim().to_string();
                    let kind = classify_url(&url);
                    if kind == UrlType::YtDlp && !ytdlp_available() {
                        self.error = "Requiere yt-dlp para esa URL".to_string();
                    } else {
                        self.error.clear();
                        open = Some(StreamRequest { url, title: None });
                    }
                }
            });
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label(RichText::new("Radios").color(ACCENT).size(12.0));
            for station in &self.radio_stations {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&station.name).color(TEXT).size(11.0));
                    if ui.small_button("Escuchar").clicked() {
                        open = Some(StreamRequest {
                            url: station.url.clone(),
                            title: Some(station.name.clone()),
                        });
                    }
                });
            }
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label(RichText::new("Podcasts (RSS)").color(ACCENT).size(12.0));
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.podcast_feed_input)
                        .hint_text("https://.../feed.xml")
                        .desired_width(ui.available_width() - 110.0),
                );
                if ui.button("Cargar").clicked() {
                    let url = self.podcast_feed_input.trim().to_string();
                    match fetch_podcast_feed(&url) {
                        Ok(feed) => {
                            self.podcast_title = feed.title;
                            self.podcast_items = feed.episodes;
                            self.error.clear();
                        }
                        Err(e) => {
                            self.error = e;
                            self.podcast_items.clear();
                        }
                    }
                }
            });

            if !self.podcast_title.is_empty() {
                ui.label(
                    RichText::new(&self.podcast_title)
                        .color(TEXT)
                        .size(11.0)
                        .strong(),
                );
            }
            egui::ScrollArea::vertical()
                .max_height(170.0)
                .show(ui, |ui| {
                    for ep in self.podcast_items.iter().take(80) {
                        ui.horizontal(|ui| {
                            let date = ep.pub_date.clone().unwrap_or_default();
                            let title = if date.is_empty() {
                                ep.title.clone()
                            } else {
                                format!("{}  ({})", ep.title, date)
                            };
                            ui.label(RichText::new(title).color(MUTED).size(10.5));
                            if ui.small_button("Reproducir").clicked() {
                                open = Some(StreamRequest {
                                    url: ep.url.clone(),
                                    title: Some(ep.title.clone()),
                                });
                            }
                        });
                    }
                });
        });

        if !self.error.is_empty() {
            ui.add_space(6.0);
            ui.label(RichText::new(&self.error).color(DANGER).size(11.0));
        }

        open
    }
}
