#![feature(iter_collect_into)]

use crate::{
    section::{Section, TypedSection, TypingConfig},
    song::Genre::Genre,
    song::Song::Song,
    typing::TypedText,
};
use leptos::*;

#[derive(Clone, Debug)]
pub struct Playlist {
    pub title: String,
    pub author: Option<String>,
    pub songs: Vec<Song>,
    pub genre: Genre,
}

impl Playlist {
    pub fn new(title: String, author: Option<String>, genre: Genre) -> Self {
        Self {
            title,
            author,
            songs: Vec::new(),
            genre,
        }
    }

    pub fn add_song(self: &mut Self, song: Song) {
        self.songs.push(song);
    }
    pub fn delete_song(self: &mut Self, index: usize) {
        self.songs.remove(index);
    }
}

#[component]
pub fn PlaylistView(playlist: Playlist, delay: u64) -> impl IntoView {
    logging::log!("Creating playlist view");

    let mut songlist = String::new();
    let song_cfg: &'static TypingConfig = &TypingConfig {
        header_interval: 50,
        header_classes: "text-xl text-gray-200",
        text_interval: 5,
        text_classes: "text-bg text-gray-300",
    };

    for song in playlist.songs {
        logging::log!("Adding {} to {}", song.title, playlist.title);
        songlist += &(song.title
            + " by "
            + &song.author
            + "  "
            + &song.duration.to_string()
            + " seconds, ");
    }

    let songs_section = Section::new(String::new(), songlist, delay, &song_cfg);

    view! {
        <div class="hackerfont flex px-20 pt-8">
            <div class="text-white text-2xl pb-8">
                <TypedText
                    text=playlist.title + " "
                    interval=70
                    stop=true
                />
                <TypedText
                    text={ match playlist.author {
                        Some(author) => { "by ".to_string() + &author + " " },
                        None         => { String::new() }
                    }}
                    interval=70
                    stop=true
                />
                <TypedText
                    text=" Genre: ".to_string() + &playlist.genre.to_string() + " "
                    interval=70
                    stop=true
                />
            </div>
            <TypedSection
                base=songs_section
            />
        </div>
    }
}
