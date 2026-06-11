use crate::player::Player;
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub fn show(ui: &mut Ui, player: &Player) {
    ui.label(
        RichText::new("Diagnóstico de codecs")
            .color(TEXT)
            .size(13.0),
    );
    ui.add_space(6.0);

    let mpv = player.mpv_handle();

    let ffmpeg_version: String = mpv
        .get_property("ffmpeg-version")
        .unwrap_or_else(|_| "N/D".into());
    let hwdec_current: String = mpv
        .get_property("hwdec-current")
        .unwrap_or_else(|_| "N/D".into());
    let vo: String = mpv.get_property("vo").unwrap_or_else(|_| "N/D".into());
    let ao: String = mpv.get_property("ao").unwrap_or_else(|_| "N/D".into());

    row(ui, "FFmpeg", &ffmpeg_version);
    row(ui, "Video output (vo)", &vo);
    row(ui, "Audio output (ao)", &ao);
    row(ui, "HW decode actual", &hwdec_current);
    ui.separator();

    let video_codecs: String = mpv.get_property("decoder-list").unwrap_or_default();
    let audio_codecs: String = mpv.get_property("audio-device-list").unwrap_or_default();
    let demuxers: String = mpv.get_property("demuxer-lavf-list").unwrap_or_default();

    section(ui, "Decoders disponibles");
    block(ui, &video_codecs);
    section(ui, "Dispositivos de audio");
    block(ui, &audio_codecs);
    section(ui, "Demuxers (lavf)");
    block(ui, &demuxers);
}

fn row(ui: &mut Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.add_sized(
            [150.0, 16.0],
            egui::Label::new(RichText::new(key).color(MUTED).size(11.0)),
        );
        ui.label(RichText::new(value).color(TEXT).size(11.0));
    });
}

fn section(ui: &mut Ui, title: &str) {
    ui.add_space(4.0);
    ui.label(RichText::new(title).color(ACCENT).size(11.0).strong());
}

fn block(ui: &mut Ui, value: &str) {
    let txt = if value.trim().is_empty() {
        "N/D"
    } else {
        value
    };
    egui::ScrollArea::vertical()
        .max_height(110.0)
        .show(ui, |ui| {
            ui.code(txt);
        });
}
