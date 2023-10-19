use leptos::*;
use rand::seq::SliceRandom;

use crate::{
    album::Playlist, body::Pages, main_menu::ReturnToMainMenu, song::Song, utils::TypedText,
};

/// A page/view that has an interactive album display with the normal typed text.
/// There are options to [add](crate::album::AddSongPage) and remove songs to the
/// playlist here.
///
/// # Arguements
/// * `playlist` - [ReadSignal] for [Playlist] to view and edit
/// * `set_playlist` - [WriteSignal] for [Playlist] to edit, should be the same as [Playlist]
/// * `set_page` - [WriteSignal] for global [page](Pages)
#[component]
pub fn ViewAlbumPage(
    playlist: ReadSignal<Playlist>,
    set_playlist: WriteSignal<Playlist>,
    set_page: WriteSignal<Pages>,
) -> impl IntoView {
    const INTERVAL: u64 = 112;

    // Singal for if to shuffle or not, and also used as a signal to start a shuffle
    let (shuffle, set_shuffle) = create_signal(false);

    view! {
        <div class="ml-8">
            // # Title
            <h1 class="hackertype text-2xl text-white">
                <TypedText
                    text=playlist().title
                    interval=INTERVAL * 2
                />
            </h1>
            // # Song list
            <ListSongs
                playlist=playlist
                set_playlist=set_playlist
                interval=INTERVAL
                shuffle=shuffle
            />
            // # Album action buttons
            <div class="flex">
                <button
                    class="rounded p-1 bg-gray-800 mx-4 my-8"
                    on:click=move |_| {set_shuffle.set(true)}
                >
                    Shuffle Playlist
                </button>
                <button
                    class="rounded p-1 bg-gray-800 mx-4 my-8"
                    on:click=move |_| {set_page.set(Pages::CreateSong)}
                >
                    Add Song
                </button>
            </div>
            <ReturnToMainMenu set_page=set_page/>
        </div>
    }
}

/// Lists songs in a playlist with the option to locally shuffle the song list.
/// This is specifically for the [ViewAlbumPage] component.
///
/// # Arguements
/// * `playlist` - [ReadSignal] for [Playlist] to view and edit
/// * `set_playlist` - [WriteSignal] for [Playlist] to edit, should be the same as [Playlist]
/// * `interval` - Time inbetween characters typed in milliseconds
/// * `shuffle` - [ReadSignal] for whether or not to shuffle, and as a signal for when to shuffle
#[component]
fn ListSongs(
    playlist: ReadSignal<Playlist>,
    set_playlist: WriteSignal<Playlist>,
    interval: u64,
    shuffle: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="flex ml-10">
            {move || playlist.with(|p| {
                // Set up songs to be passed to [DisplaySong] and apply suffle if applicable
                let songs = match shuffle.get() {
                    true => {
                        let mut new_songs = p.songs.clone();
                        new_songs.shuffle(&mut rand::thread_rng());
                        new_songs
                    }
                    false => p.songs.clone()
                };
                // Call [DisplaySong] with appropriate arguements
                songs.into_iter().map(|song| {
                    view! {
                        <DisplaySong
                            song=song
                            interval=interval
                            set_playlist=set_playlist
                        />
                    }
                }).collect_view()
            })}
        </div>
    }
}

/// Displays a singular song for the [ViewAlbumPage] component.
/// This includes a delete button
/// # Arguements
/// * `song` - [song] to display
/// * `interval` - Time between typed characters in milliseconds
/// * `set_playlist` - Global set_playlist
#[component]
fn DisplaySong(song: Song, interval: u64, set_playlist: WriteSignal<Playlist>) -> impl IntoView {
    // We need an extra title for the delete button, and it has to be before the on:click feild.
    let title = song.title.clone();

    view! {
        <div class="mx-4">
            // # Title
            <div class="text-xl text-gray-100">
                <TypedText
                    text=song.title.clone()
                    interval=interval
                    stop=true
                />
            </div>
            // # Other
            <div class="text-lg text-gray-200 flex flex-col">
                // ## Author
                <div class="flex flex-row">
                    <p class="text-gray-300 mr-2">Author: </p>
                    <TypedText
                        text=song.author
                        interval=interval
                        stop=true
                    />
                </div>
                // ## Author
                <div class="flex flex-row">
                    <p class="text-gray-300">Duration: </p>
                    <TypedText
                        text=song.duration.to_string()
                        interval=interval
                        stop=true
                    />
                </div>
                // ## Author
                <div class="flex flex-row">
                    <p class="text-gray-300">Genre: </p>
                    <TypedText
                        text=song.genre.to_string()
                        interval=interval
                        stop=true
                    />
                </div>
            </div>
            // # Delete Button
            <button
                type="button"
                class="rounded bg-gray-800"
                on:click=move |_| {
                    logging::log!("deleting song");
                        set_playlist.update(|p| {
                            p.songs.retain(|inner_song| &inner_song.title != &song.title);
                        });
                    }
                >
                    Delete {title}
            </button>
        </div>
    }
}
