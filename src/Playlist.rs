/* use leptos::{component, IntoView, create_signal};

use crate::{Song::Song, Genre::Genre, section::{Section, TypingConfig, StrOption}};

pub struct Playlist<'a> {
    title:  &'a str,
    author: Option<&'a str>,
    songs:  Vec<Song<'a>>,
    genre:  Genre,
}

impl Playlist<'_> {
    pub fn add_song(self: &mut Self, song: Song) {
        self.songs.push(song);
    }
    pub fn delete_song(self: &mut Self, index: usize) {
        self.songs.remove(index);
    }
}

#[component]
pub fn View<'a>(playlist: &'a Playlist<'a>, delay: u64, cfg: &TypingConfig) -> impl IntoView {
    let s = stringify!("Author: ", playlist.author, " Genre: ", playlist.genre);
    let section = Section::new(StrOption::Static(playlist.title), StrOption::Static(s), delay, cfg); 
} */
