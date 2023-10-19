use crate::album::Playlist;
use crate::album::PlaylistView;
use crate::body::Pages;
use crate::main_menu::ReturnToMainMenu;
use crate::utils::PointerOnHover;
use leptos::*;

/// Page/view for the list of albums with links to the albums.
/// From here, the user can navigate to [AddSongPage](crate::album::AddSongPage), and
/// [ViewAlbumPage](crate::album::ViewAlbumPage). This page provides the functionality to edit albums. Creatation
/// is reserved for the [CrateAlbumPage](crate::album::CreateAlbumPage) in
/// the [MainMenu](crate::MainMenuPage)
///
/// # Arguments
/// * `playlists` - The [global](crate::body::Body) [Vec] of [ReadSignal]/[WriteSignal] [Playlist] pairs.
/// * `set_page` - The [global](crate::body::Body) [page](Pages)
/// * `set_playlist_buff` - The [global](body::Body) [ReadSignal]/[WriteSignal] [Playlist] pair
/// buffer, used as the selected playlist in this case.
#[component]
pub fn ViewAlbumListPage(
    playlists: Vec<(ReadSignal<Playlist>, WriteSignal<Playlist>)>,
    set_page: WriteSignal<Pages>,
    set_playlist_buff: WriteSignal<(ReadSignal<Playlist>, WriteSignal<Playlist>)>,
) -> impl IntoView {
    view! {
        {playlists.into_iter()
            .map(|p| view! {
                // # Per-album render
                <PointerOnHover>
                    <div
                        class="cursor-pointer"
                        on:click=move |_| {
                            set_playlist_buff.set(p);
                            set_page.set(Pages::ViewAlbum)
                    }>
                        <PlaylistView
                            playlist=p.0()
                        />
                    </div>
                </PointerOnHover>
            }).collect_view()
        }
        <div class="mt-[20vh]"/>
        <ReturnToMainMenu
          set_page=set_page
        />
    }
}
