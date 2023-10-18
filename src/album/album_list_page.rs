use crate::album::Playlist;
use crate::album::PlaylistView;
use crate::body::Pages;
use crate::main_menu::ReturnToMainMenu;
use leptos::*;

#[component]
pub fn ViewAlbumListPage(
    playlists: Vec<(ReadSignal<Playlist>, WriteSignal<Playlist>)>,
    delay: u64,
    set_page: WriteSignal<Pages>,
    set_playlist_buff: WriteSignal<(ReadSignal<Playlist>, WriteSignal<Playlist>)>,
    current_page: ReadSignal<Pages>,
) -> impl IntoView {
    view! {
        {playlists.into_iter()
            .map(|p| view! {
                <div
                    class="cursor-pointer"
                    on:click=move |_| {
                        set_playlist_buff.set(p);
                        set_page.set(Pages::ViewAlbum)
                }>
                    <PlaylistView
                        playlist=p.0()
                        delay=delay
                        current_page=current_page
                    />
              </div>
            }).collect_view()
        }
        <ReturnToMainMenu
          set_page=set_page
          current_page=current_page
        />
    }
}
