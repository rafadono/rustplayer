use std::fs;
use std::path::PathBuf;

use rplayer::{
    bookmarks::{Bookmark, BookmarkStore},
    history::History,
    notes::{Note, NoteStore},
    playlist::Playlist,
    config::{AspectRatio, Config},
    equalizer::Equalizer,
};

fn sandbox_path(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("rplayer_integration_tests")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn playlist_import_and_export_integration() {
    let dir = sandbox_path("playlist_import_export");
    let playlist_file = dir.join("demo.m3u");
    let content = "#EXTM3U\ntrack.mp3\n#EXTINF:-1,Artist - Track\nhttp://example.com/stream.mp3\n";
    fs::write(&playlist_file, content).unwrap();

    let mut playlist = Playlist::default();
    let loaded = playlist.load_m3u(&playlist_file);
    assert_eq!(loaded.len(), 2);
    assert_eq!(playlist.tracks[0].title, "track.mp3");
    assert_eq!(playlist.tracks[1].title, "Artist - Track");
    assert!(playlist.tracks[1]
        .path
        .to_string_lossy()
        .starts_with("http://example.com/stream.mp3"));

    let export_file = dir.join("exported.m3u");
    playlist.export_m3u(&export_file).unwrap();
    let exported = fs::read_to_string(&export_file).unwrap();
    assert!(exported.contains("#EXTINF:-1,Artist - Track"));
    assert!(exported.contains("track.mp3"));
}

#[test]
fn bookmarks_and_notes_integration_flow() {
    let mut bookmarks = BookmarkStore::default();
    let mut notes = NoteStore::default();
    let path = PathBuf::from("video.mp4");

    let bookmark = Bookmark::new(12.0, "chapter");
    let id = bookmark.id;
    bookmarks.add(&path, bookmark);
    bookmarks.update_label(&path, id, "intro".to_string());

    let note = Note::new(55.0, "important");
    notes.add(&path, note);

    let saved = bookmarks.get(&path);
    assert_eq!(saved.len(), 1);
    assert_eq!(saved[0].label, "intro");

    let exported_notes = notes.export_text(&path);
    assert!(exported_notes.contains("important"));
}

#[test]
fn history_playback_integration_flow() {
    let mut history = History::default();
    let path = PathBuf::from("movie.mkv");

    history.mark_play_start(&path, "movie", 240.0);
    history.update(&path, "movie", 30.0, 240.0);

    let entry = history.get(&path).expect("should have history entry");
    assert_eq!(entry.play_count, 1);
    assert_eq!(entry.last_position, 30.0);
    assert!(entry.should_resume());
}

#[test]
fn config_new_features_defaults_and_serialization() {
    let mut config = Config::default();

    assert_eq!(config.crop, 0.0);
    assert_eq!(config.deinterlace, false);
    assert_eq!(config.loudnorm, false);
    assert_eq!(config.sub_pos, 100);
    assert_eq!(config.aspect_ratio, AspectRatio::Auto);

    config.crop = 1.0;
    config.deinterlace = true;
    config.loudnorm = true;
    config.sub_pos = 90;
    config.aspect_ratio = AspectRatio::Ratio16_9;

    let json = serde_json::to_string(&config).unwrap();
    let loaded: Config = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.crop, 1.0);
    assert_eq!(loaded.deinterlace, true);
    assert_eq!(loaded.loudnorm, true);
    assert_eq!(loaded.sub_pos, 90);
    assert_eq!(loaded.aspect_ratio, AspectRatio::Ratio16_9);
}

#[test]
fn aspect_ratio_mpv_value_mapping() {
    assert_eq!(AspectRatio::Auto.to_mpv_value(), "-1");
    assert_eq!(AspectRatio::Ratio16_9.to_mpv_value(), "16/9");
    assert_eq!(AspectRatio::Ratio4_3.to_mpv_value(), "4/3");
    assert_eq!(AspectRatio::Ratio21_9.to_mpv_value(), "21/9");
    assert_eq!(AspectRatio::Ratio1_1.to_mpv_value(), "1/1");
    assert_eq!(AspectRatio::Custom(2.35).to_mpv_value(), "2.3500");
}

#[test]
fn loudnorm_af_chain_integration() {
    let eq = Equalizer::default();

    let chain_normal = eq.to_mpv_af_chain(false);
    assert!(!chain_normal.contains("lavfi=[loudnorm]"));

    let chain_loud = eq.to_mpv_af_chain(true);
    assert!(chain_loud.starts_with("lavfi=[loudnorm]"));
}
