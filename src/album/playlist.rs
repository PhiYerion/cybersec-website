use rand::Rng;

use crate::{
    body::Pages,
    section::{Section, TypedSection, TypingConfig},
    song::genre::Genre,
    song::song::Song,
    utils::TypedText,
};
use leptos::*;

#[derive(Clone, Debug)]
pub struct Playlist {
    pub title: String,
    pub author: Option<String>,
    pub songs: Vec<Song>,
    pub genre: Genre,
    pub image: String,
}

impl Playlist {
    pub fn new(title: String, author: Option<String>, genre: Genre) -> Self {
        let mut rng = rand::thread_rng();
        let image = "/assets/".to_string() + &rng.gen_range(1..=5).to_string() + ".jpg";
        Self {
            title,
            author,
            songs: Vec::new(),
            genre,
            image,
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
pub fn PlaylistView(
    playlist: Playlist,
    delay: u64,
    current_page: ReadSignal<Pages>,
) -> impl IntoView {
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
            <img src=playlist.image class="w-[10vw] h-[10vw] mr-8" />
            <div class="text-white text-2xl pb-8">
                <TypedText
                    text=playlist.title + " "
                    interval=70
                    stop=true
                    current_page=current_page
                />
                <TypedText
                    text={ match playlist.author {
                        Some(author) => { "by ".to_string() + &author + " " },
                        None         => { String::new() }
                    }}
                    interval=70
                    stop=true
                    current_page=current_page
                />
                <TypedText
                    text=" Genre: ".to_string() + &playlist.genre.to_string() + " "
                    interval=70
                    stop=true
                    current_page=current_page
                />
            </div>
            <TypedSection
                base=songs_section
                current_page=current_page
            />
        </div>
    }
}
