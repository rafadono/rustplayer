//! opensubtitles_panel.rs — Search and download subtitles from OpenSubtitles.

use crate::opensubtitles::{
    common_languages, is_configured, SubResult, SubSearchJob, SubSearchStatus,
};
use crate::ui::theme::*;
use egui::{RichText, Ui};
use std::path::PathBuf;

pub struct OpenSubtitlesPanel {
    pub lang_idx: usize,
    pub search_query: String,
    pub results: Vec<SubResult>,
    pub status_msg: String,
    search_job: Option<SubSearchJob>,
    download_job: Option<SubSearchJob>,
    pub downloaded: Option<PathBuf>,
}

impl OpenSubtitlesPanel {
    pub fn new() -> Self {
        Self {
            lang_idx: 0,
            search_query: String::new(),
            results: Vec::new(),
            status_msg: String::new(),
            search_job: None,
            download_job: None,
            downloaded: None,
        }
    }

    /// Returns Some(path) when a downloaded subtitle is ready to load.
    pub fn show(&mut self, ui: &mut Ui, current_file: Option<&std::path::Path>) -> Option<PathBuf> {
        // Drain job results in background
        self.poll_jobs();

        let ready = self.downloaded.take();
        if ready.is_some() {
            return ready;
        }

        ui.vertical(|ui| {
            ui.label(RichText::new("Descargar subtítulos").color(TEXT).size(13.0));
            ui.add_space(6.0);

            if !is_configured() {
                ui.label(
                    RichText::new("⚠ API key no configurada.")
                        .color(WARNING)
                        .size(12.0),
                );
                ui.label(
                    RichText::new("Editar API_KEY en src/opensubtitles.rs")
                        .color(MUTED)
                        .size(11.0),
                );
                ui.label(
                    RichText::new("Crear cuenta en opensubtitles.com/consumers")
                        .color(MUTED)
                        .size(10.0),
                );
                return;
            }

            let langs = common_languages();
            let lang_label = langs[self.lang_idx].1;

            ui.horizontal(|ui| {
                // query
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("Título de la película / serie...")
                        .desired_width(ui.available_width() - 120.0),
                );

                // Language
                egui::ComboBox::from_id_source("sub_lang")
                    .selected_text(RichText::new(lang_label).size(11.0))
                    .width(80.0)
                    .show_ui(ui, |ui| {
                        for (i, (_, name)) in langs.iter().enumerate() {
                            ui.selectable_value(&mut self.lang_idx, i, *name);
                        }
                    });
            });

            // Auto-fill with file name
            if self.search_query.is_empty() {
                if let Some(f) = current_file {
                    if ui.small_button("Usar nombre del archivo").clicked() {
                        self.search_query = f
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_default();
                    }
                }
            }

            ui.add_space(6.0);

            let can_search = self.search_job.is_none() && !self.search_query.trim().is_empty();
            if ui
                .add_enabled(
                    can_search,
                    egui::Button::new(RichText::new("Buscar").color(TEXT).size(12.0)),
                )
                .clicked()
            {
                let lang = langs[self.lang_idx].0;
                if let Some(job) =
                    SubSearchJob::search(std::path::Path::new(&self.search_query), lang)
                {
                    self.search_job = Some(job);
                    self.status_msg = "Buscando...".into();
                    self.results.clear();
                }
            }

            if !self.status_msg.is_empty() {
                ui.label(RichText::new(&self.status_msg).color(MUTED).size(11.0));
            }

            ui.add_space(8.0);

            if !self.results.is_empty() {
                ui.label(
                    RichText::new(format!("{} resultados", self.results.len()))
                        .color(MUTED)
                        .size(11.0),
                );
                ui.add_space(4.0);

                egui::ScrollArea::vertical()
                    .max_height(260.0)
                    .show(ui, |ui| {
                        let mut download_idx: Option<usize> = None;

                        for (i, res) in self.results.iter().enumerate() {
                            let hi_label = if res.hearing_impaired { " [HI]" } else { "" };
                            egui::Frame {
                                fill: *SURFACE2,
                                inner_margin: egui::Margin::symmetric(8.0, 6.0),
                                rounding: egui::Rounding::same(4.0),
                                ..Default::default()
                            }
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "[{}]{} {}",
                                            res.language.to_uppercase(),
                                            hi_label,
                                            res.title
                                        ))
                                        .size(12.0)
                                        .color(TEXT),
                                    );
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            let can_dl = self.download_job.is_none();
                                            if ui
                                                .add_enabled(
                                                    can_dl,
                                                    egui::Button::new(
                                                        RichText::new("↓").color(ACCENT).size(13.0),
                                                    ),
                                                )
                                                .on_hover_text("Descargar")
                                                .clicked()
                                            {
                                                download_idx = Some(i);
                                            }
                                        },
                                    );
                                });
                                if !res.release.is_empty() {
                                    ui.label(RichText::new(&res.release).size(10.0).color(MUTED));
                                }
                            });
                            ui.add_space(2.0);
                        }

                        if let Some(idx) = download_idx {
                            let dest = current_file
                                .and_then(|f| f.parent())
                                .unwrap_or(std::path::Path::new("."));

                            if let Some(job) = SubSearchJob::download(&self.results[idx], dest) {
                                self.download_job = Some(job);
                                self.status_msg = "Descargando subtítulo...".into();
                            }
                        }
                    });
            }
        });

        None
    }

    fn poll_jobs(&mut self) {
        if let Some(job) = self.search_job.take() {
            let mut keep_job = true;
            while let Ok(status) = job.rx.try_recv() {
                match status {
                    SubSearchStatus::Results(r) => {
                        self.results = r;
                        self.status_msg = if self.results.is_empty() {
                            "Sin resultados".into()
                        } else {
                            String::new()
                        };
                        keep_job = false;
                    }
                    SubSearchStatus::Error(e) => {
                        self.status_msg = format!("Error: {}", e);
                        keep_job = false;
                    }
                    _ => {}
                }
            }
            if keep_job {
                self.search_job = Some(job);
            }
        }

        if let Some(job) = self.download_job.take() {
            let mut keep_job = true;
            while let Ok(status) = job.rx.try_recv() {
                match status {
                    SubSearchStatus::Done(path) => {
                        self.status_msg = format!("Descargado: {}", path.display());
                        self.downloaded = Some(path);
                        keep_job = false;
                    }
                    SubSearchStatus::Error(e) => {
                        self.status_msg = format!("Error al descargar: {}", e);
                        keep_job = false;
                    }
                    _ => {}
                }
            }
            if keep_job {
                self.download_job = Some(job);
            }
        }
    }
}
