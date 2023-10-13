use leptos::*;

use crate::{section::Section, Playlist::{self}, Genre, MainMenu::ReturnToMainMenu, Pages};


#[component]
pub fn ViewAlbumView(
    playlists: Vec<Playlist::Playlist>, 
    delay: u64, 
    set_page: WriteSignal<Pages>
) -> impl IntoView {

    view! {
        {playlists.into_iter()
            .map(|p|
                Playlist::View(
                    Playlist::ViewProps { playlist: p, delay: delay }
            )).collect_view()
        }
        <ReturnToMainMenu
          set_page=set_page
        />
    }
}

