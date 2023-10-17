use leptos::*;

mod MainMenu;
mod album;
mod section;
mod song;
mod typing;

use album::EditAlbum::CreateSongPage;
use album::Playlist;
use album::ViewAlbum::{ViewAlbumListPage, ViewAlbumPage};
use song::Genre;
use song::Song;
use typing::TypedText;
use MainMenu::MainMenuPage;

#[component]
fn Header() -> impl IntoView {
    logging::log!("Creating header.");
    view! {
      <nav style="background-color: #222" class="text-6xl flex justify-center items-center w-screen pt-12 pb-6">
        <h1 class="hackerfont text-center text-green-500">
          <TypedText centered=true interval=275 stop=false text="''Blazor App''".to_string() />
        </h1>
      </nav>
    }
}

#[component]
fn TyperButton(children: Children) -> impl IntoView {
    logging::log!("Creating button with typer effect");
    let (arrow, set_arrow) = create_signal(None);

    view! {
      <button
        on:mouseenter=move |_| {
            set_arrow(Some("-->"));
        }
        on:mouseleave=move |_| {
            set_arrow(None);
        }
      >
        <div class="flex flex-row items-center">
          <span class="whitespace-nowrap flex-shrink-0 text-2xl">{arrow}</span>
          {children()}
        </div>
      </button>
    }
}

pub enum Pages {
    MainMenu,
    ViewAlbumList,
    AddAlbum,
    EditAlbum,
    ViewAlbum,
}

#[component]
fn Body() -> impl IntoView {
    const CMD_INTERVAL: u64 = 80;

    logging::log!("Creating body element");

    let (cmd_text, set_cmd_text) = create_signal("msbuild ./vmlinuz-linux.sln");
    let (page, set_page) = create_signal(Pages::MainMenu);

    let mut playlists_base: Vec<Playlist::Playlist> = vec![
        Playlist::Playlist::new(
            "fjdkslaflk".to_string(),
            Some("fjdklsajkfdl".to_string()),
            Genre::Genre::Pop,
        ),
        Playlist::Playlist::new(
            "fjdkslaflk".to_string(),
            Some("fjdklsajkfdl".to_string()),
            Genre::Genre::Pop,
        ),
        Playlist::Playlist::new(
            "fjdkslaflk".to_string(),
            Some("fjdklsajkfdl".to_string()),
            Genre::Genre::Pop,
        ),
        Playlist::Playlist::new(
            "fjdkslaflk".to_string(),
            Some("fjdklsajkfdl".to_string()),
            Genre::Genre::Pop,
        ),
    ];
    for playlist in &mut playlists_base {
        playlist.add_song(Song::Song {
            title: "test".into(),
            duration: 320,
            author: "someone".into(),
            genre: Genre::Genre::Pop,
        });
        playlist.add_song(Song::Song {
            title: "testtwo".into(),
            duration: 3210,
            author: "fdskla".into(),
            genre: Genre::Genre::Pop,
        });
    }

    let playlists: Vec<_> = playlists_base
        .into_iter()
        .map(|p| create_signal(p))
        .collect();

    let (playlist_buff, set_playlist_buff) = create_signal(playlists[0]);

    view! {
      <div class="hackerfont text-gray-300">
        <p class="hackerfont text-lg text-white">
          <span class="text-red-500">
            "[root@csci.etsu.edu /usr/src/linux-6.5.6-gentoo]# "
          </span>
          <span>
            {move || {cmd_text.with(|cmd| {
              view! {<TypedText centered=false interval={CMD_INTERVAL} stop=true text={cmd.to_string()} />}
            })}}
          </span>
        </p>
        {move || {
            page.with(|p| { logging::log!("finding match statement"); match p {
                Pages::MainMenu => {
                    set_cmd_text("msbuild ./linux.sln");
                    logging::log!("Building MainMenu");
                    view! {
                        <MainMenuPage
                            delay=cmd_text().len() as u64 * CMD_INTERVAL + 20
                            set_page=set_page
                        />

                    }
                }
                Pages::ViewAlbumList => {
                    set_cmd_text("msbuild ./ViewAlbums.sln");
                    logging::log!("Building ViewAlbums");
                    view! {
                        <ViewAlbumListPage
                            playlists=playlists.clone()
                            delay=cmd_text().len() as u64 * CMD_INTERVAL + 20
                            set_page=set_page
                            set_playlist_buff=set_playlist_buff
                        />
                    }
                }
                Pages::ViewAlbum => {
                    set_cmd_text("ms bufjdik alsjkfl ild ./linux.sln");
                    view! {
                        <ViewAlbumPage
                            playlist=playlists[0]
                            set_page=set_page
                        />
                    }
                }
                _ => {
                    set_cmd_text("ms bufjdik alsjkfl ild ./linux.sln");
                    view! {
                        <ViewAlbumPage
                            playlist=playlists[0]
                            set_page=set_page
                        />
                    }
                }
            }})
        }}
      </div>
    }
}

#[component]
fn App() -> impl IntoView {
    extern crate console_error_panic_hook;
    use std::panic;

    panic::set_hook(Box::new(console_error_panic_hook::hook));
    view! {
      <html>
        <Header/>
        <Body/>
      </html>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
