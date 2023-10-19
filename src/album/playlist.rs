use rand::Rng;

use crate::{
    body::Pages,
    section::{Section, TypedSection, TypedSectionProps, TypingConfig},
    song::genre::Genre,
    song::song::Song,
    utils::TypedText,
};
use leptos::*;

/// A simple playlist struct
#[derive(Clone, Debug)]
pub struct Playlist {
    pub title: String,
    pub author: Option<String>,
    pub songs: Vec<Song>,
    pub genre: Genre,
    pub image: String,
}

impl Playlist {
    /// Create a new playlist out of required feilds.
    /// This will select a random image and create an empty list of songs for your playlist.
    ///
    /// # Image Not Found
    /// This assumes that there are images {1,2,3,4,5}.jpg in /assets. If there are not, there will
    /// be a false link to one.
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

    /// Adds a song to the playlist by pushing it onto the vec
    pub fn add_song(self: &mut Self, song: Song) {
        self.songs.push(song);
    }
    /// Deletes a song by index
    pub fn delete_song(self: &mut Self, index: usize) {
        self.songs.remove(index);
    }
}

/// Creates a basic view of a [Playlist].
///
/// # Arguments
/// * `playlist` - [Playlist] to show
/// * `delay` - Delay in milliseconds before starting the view
#[component]
pub fn PlaylistView(playlist: Playlist) -> impl IntoView {
    logging::log!("Creating playlist view");

    // list that will later be displayed
    let mut songlist: Vec<String> = Vec::new();

    // Add formatted string to songlist
    for song in playlist.songs {
        logging::log!("Adding {} to {}", song.title, playlist.title);
        songlist.push(
            song.title + " by " + &song.author + "  " + &song.duration.to_string() + " seconds",
        );
    }

    view! {
        // # Playlist meta
        <div class="hackerfont flex px-20 pt-8">
            <img src=playlist.image class="w-[10vw] h-[10vw] mr-8" />
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
            // # Songs
            <div class = "ml-20">
                {songlist.into_iter().map(|s| {
                    view! {
                        <TypedText
                            text=s.clone()
                            interval=70
                            centered=false
                            stop=true
                            delay=s.len() as u64 * 10
                        />
                        <br/>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
