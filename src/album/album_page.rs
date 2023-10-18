use leptos::*;
use rand::seq::SliceRandom;

use crate::{album::Playlist, body::Pages, main_menu::ReturnToMainMenu, utils::TypedText};

#[component]
pub fn ViewAlbumPage(
    playlist: (ReadSignal<Playlist>, WriteSignal<Playlist>),
    set_page: WriteSignal<Pages>,
    current_page: ReadSignal<Pages>,
) -> impl IntoView {
    const INTERVAL: u64 = 112;
    let (shuffle, set_shuffle) = create_signal(false);

    view! {
        <div class="ml-8">
        <h1 class="hackertype text-2xl text-white">
            <TypedText
                text=playlist.0().title
                interval=INTERVAL * 2
                current_page=current_page
            />
        </h1>
        <div class="flex ml-10">
            {move || playlist.0.with(|p| {
                let songs = match shuffle.get() {
                    true => {
                        let mut new_songs = p.songs.clone();
                        new_songs.shuffle(&mut rand::thread_rng());
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
                                    current_page=current_page
                                />
                            </div>
                            <div class="text-lg text-gray-200 flex flex-col">
                                <div class="flex flex-row">
                                    <p class="text-gray-300 mr-2">Author: </p>
                                    <TypedText
                                        text=song_copy.author
                                        interval=INTERVAL
                                        stop=true
                                        current_page=current_page
                                    />
                                </div>
                                <div class="flex flex-row">
                                    <p class="text-gray-300">Duration: </p>
                                    <TypedText
                                        text=song_copy.duration.to_string()
                                        interval=INTERVAL
                                        stop=true
                                        current_page=current_page
                                    />
                                </div>
                                <div class="flex flex-row">
                                    <p class="text-gray-300">Genre: </p>
                                    <TypedText
                                        text=song_copy.genre.to_string()
                                        interval=INTERVAL
                                        stop=true
                                        current_page=current_page
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
        </div>
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
        <ReturnToMainMenu set_page=set_page current_page=current_page/>
        </div>
    }
}
