use leptos::*;
use rand::seq::SliceRandom;

use crate::{
    section::Section,
    typing::TypedText,
    Genre,
    MainMenu::{self, ReturnToMainMenu},
    Pages,
    Playlist::{self, PlaylistView},
};

#[component]
pub fn ViewAlbumPage(
    playlist: (
        ReadSignal<Playlist::Playlist>,
        WriteSignal<Playlist::Playlist>,
    ),
    set_page: WriteSignal<Pages>,
) -> impl IntoView {
    const INTERVAL: u64 = 112;
    let (shuffle, set_shuffle) = create_signal(false);

    view! {
        <div class="ml-8">
        <h1 class="hackertype text-2xl text-white">
            <TypedText
                text=playlist.0().title
                interval=INTERVAL * 2
            />
        </h1>
        <div class="flex ml-10">
            {move || playlist.0.with(|p| {
                let songs = match shuffle.get() {
                    true => {
                        let mut rng = rand::thread_rng();
                        let mut new_songs = p.songs.clone();
                        new_songs.shuffle(&mut rng);
                        new_songs
                    }
                    false => p.songs.clone()
                };
                songs.into_iter().map(|song| {
                    let song_copy = song.clone();
                    view! {
                        <div class="mx-4">
                            <div class="text-xl text-gray-100">
                                <TypedText
                                    text=song_copy.title.clone()
                                    interval=INTERVAL
                                    stop=true
                                />
                            </div>
                            <div class="text-lg text-gray-200 flex flex-col">
                                <div class="flex flex-row">
                                    <p class="text-gray-300 mr-2">Author: </p>
                                    <TypedText
                                        text=song_copy.author
                                        interval=INTERVAL
                                        stop=true
                                    />
                                </div>
                                <div class="flex flex-row">
                                    <p class="text-gray-300">Duration: </p>
                                    <TypedText
                                        text=song_copy.duration.to_string()
                                        interval=INTERVAL
                                        stop=true
                                    />
                                </div>
                                <div class="flex flex-row">
                                    <p class="text-gray-300">Genre: </p>
                                    <TypedText
                                        text=song_copy.genre.to_string()
                                        interval=INTERVAL
                                        stop=true
                                    />
                                </div>
                            </div>
                            <button
                                type="button"
                                class="rounded bg-gray-800"
                                on:click=move |_| {
                                    logging::log!("deleting song");
                                    playlist.1.update(|p| {
                                        p.songs.retain(|inner_song| &inner_song.title != &song.title);
                                    });
                                }
                            >
                                Delete {song_copy.title}
                            </button>
                        </div>
                    }
                }).collect_view()
            })}
        <button
            on:click=move |_| {set_shuffle.set(true)}
            class="rounded p-1 bg-gray-800"
        >
            Shuffle Playlist
        </button>
        </div>
        <ReturnToMainMenu set_page=set_page/>
        </div>
    }
}

#[component]
pub fn ViewAlbumListPage(
    playlists: Vec<(
        ReadSignal<Playlist::Playlist>,
        WriteSignal<Playlist::Playlist>,
    )>,
    delay: u64,
    set_page: WriteSignal<Pages>,
    set_playlist_buff: WriteSignal<(
        ReadSignal<Playlist::Playlist>,
        WriteSignal<Playlist::Playlist>,
    )>,
) -> impl IntoView {
    view! {
        {playlists.into_iter()
            .map(|p| view! {
                <div on:click=move |_| {
                    set_playlist_buff.set(p);
                    set_page.set(Pages::ViewAlbum)
                }>
                <PlaylistView
                    playlist=p.0()
                    delay=delay
                />
              </div>
            }).collect_view()
        }
        <ReturnToMainMenu
          set_page=set_page
        />
    }
}
