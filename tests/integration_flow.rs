use std::fs;
use std::path::PathBuf;

use rplayer::{
    bookmarks::{Bookmark, BookmarkStore},
    history::History,
    notes::{Note, NoteStore},
    playlist::Playlist,
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
