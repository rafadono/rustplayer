use egui::{Color32, RichText, Ui};

fn patreon_url() -> String {
    obfstr::obfstr!("https://patreon.com/TU_USUARIO").to_string()
}

pub struct DonationBanner {
    dismissed: bool,
}

impl DonationBanner {
    pub fn new(show: bool) -> Self {
        Self { dismissed: !show }
    }

    /// Returns true if it was closed in this frame (to save config).
    pub fn show(&mut self, ui: &mut Ui) -> bool {
        if self.dismissed {
            return false;
        }

        let mut closed = false;

        egui::Frame {
            fill: Color32::from_rgb(24, 24, 32),
            inner_margin: egui::Margin::symmetric(14.0, 8.0),
            ..Default::default()
        }
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("RPlayer es libre y sin publicidad.")
                        .color(Color32::from_rgb(180, 180, 195))
                        .size(13.0),
                );

                ui.add_space(6.0);

                if ui
                    .button(
                        RichText::new("Patreon")
                            .color(Color32::from_rgb(255, 102, 66))
                            .size(13.0),
                    )
                    .on_hover_text(patreon_url())
                    .clicked()
                {
                    let _ = open::that(patreon_url());
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .small_button(RichText::new("✕").color(Color32::from_rgb(110, 110, 120)))
                        .on_hover_text("No volver a mostrar")
                        .clicked()
                    {
                        self.dismissed = true;
                        closed = true;
                    }
                });
            });
        });

        closed
    }
}
