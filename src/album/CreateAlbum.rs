use crate::{Genre::Genre, MainMenu::ReturnToMainMenu, Pages, Playlist};
use leptos::html::Input;
use leptos::*;
use web_sys::SubmitEvent;

#[component]
pub fn CreateAlbumView(
    set_page: WriteSignal<Pages>,
    playlists: ReadSignal<Vec<Playlist::Playlist>>,
    set_playlists: WriteSignal<Vec<Playlist::Playlist>>,
) -> impl IntoView {
    view! {


        <ReturnToMainMenu
          set_page=set_page
        />
    }
}
