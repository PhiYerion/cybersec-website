use crate::album::playlist::Playlist;
use crate::song::{Genre, Song};

/// Create a test, placeholder list of [Playlist]s
///
/// # Returns
/// A [Vec] of four [Playlist]s with two [Song]s each
pub fn create_test_playlists() -> Vec<Playlist> {
    let mut playlists_base: Vec<Playlist> = vec![
        Playlist::new(
            "Playlist One".to_string(),
            Some("Artist One".to_string()),
            Genre::Pop,
        ),
        Playlist::new("Playlist Two".to_string(), Some("".to_string()), Genre::Pop),
        Playlist::new(
            "Playlist Three".to_string(),
            Some("Artist Three".to_string()),
            Genre::Pop,
        ),
        Playlist::new(
            "Playlist Four".to_string(),
            Some("Artist Four".to_string()),
            Genre::Pop,
        ),
    ];

    for playlist in &mut playlists_base {
        playlist.add_song(Song {
            title: "Song One".into(),
            duration: 320,
            author: "Song One's Artist".into(),
            genre: Genre::Pop,
        });
        playlist.add_song(Song {
            title: "Song Two".into(),
            duration: 3210,
            author: "Song Two's Artist".into(),
            genre: Genre::Pop,
        });
    }

    playlists_base
}
