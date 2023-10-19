use leptos::*;

use crate::album::{AddSongPage, CreateAlbumPage, ViewAlbumListPage, ViewAlbumPage};
use crate::main_menu::MainMenuPage;
use crate::utils::{create_test_playlists, TypedText};

/// Enum defining all possible views/pages the body could be rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pages {
    MainMenu,
    ViewAlbumList,
    AddAlbum,
    CreateSong,
    ViewAlbum,
}

/// Handles switching views/pages and manages state between views/pages.
///
/// # "Global State"
/// The body instantiates and handles global state, but all read/write operations are done by
/// pages/views that the body passes these values to.
///
/// ##### "Global" Variables
/// * `(set_)cmd_text` - reactive signal for the command line at the top of the body
/// * `(set_)page` - reactive signal for which page is currently active
/// * `(set_)reactive_playlists` - the master list of all playlists
#[component]
pub fn Body() -> impl IntoView {
    const CMD_INTERVAL: u64 = 80;

    logging::log!("Creating body element");

    // # Create our 'global' signals
    let (cmd_text, set_cmd_text) = create_signal("msbuild ./vmlinuz-linux.sln");
    let (page, set_page) = create_signal(Pages::MainMenu);

    // # Set up our 'global' playlists
    // Here are our base playlist list with some example playlists
    let playlists_base = create_test_playlists();

    // Now we are converting this into a list of reactive playlists
    let playlists: Vec<_> = playlists_base
        .into_iter() // <= transform into iterator
        .map(|p| create_signal(p)) // <=   for each elemennt, create a reactive signal
        .collect(); // <=     collect all into a vector

    // # Set up our playlist buffer
    // Here is our playlist buffer, which holds one playlist reference. This is
    // going to be used to find the selected playlist for now, but can be extended
    // for different things.
    // Note: this comes before reactive_playlists because it is better for us to
    // clone and reactive_playlists to consume.
    let (playlist_buff, set_playlist_buff) = create_signal(playlists[0].clone());

    // # Final playlists
    // Now we convert the list of reactive playlists into a reactive list of
    // reactive playlists that we will use in our application.
    let (reactive_playlists, set_reactive_playlists) = create_signal(playlists);

    view! {
      <div class="hackerfont text-gray-300">
        // # Command line
        <p class="hackerfont text-lg text-white">
          <span class="text-red-500">
            "[root@csci.etsu.edu /usr/src/linux-6.5.6-gentoo]# "
          </span>
          <span>
            {move || {cmd_text.with(|cmd| {
              view! {
                  <TypedText
                      centered=false
                      interval={CMD_INTERVAL}
                      stop=true
                      text={cmd.to_string()}
                   />
              }
            })}}
          </span>
        </p>
        // # Page/view handling
        {move || {
            // On change to page, run this match statement
            page.with(|p| {match p {
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
                    logging::log!("Building ViewAlbumList");
                    view! {
                        <ViewAlbumListPage
                            playlists=reactive_playlists()
                            set_page=set_page
                            set_playlist_buff=set_playlist_buff
                        />
                    }
                }
                Pages::ViewAlbum => {
                    set_cmd_text("nmap -p445 --script smb-vuln-ms17-010 127.0.0.1");
                    logging::log!("Building ViewAlbums");
                    view! {
                        <ViewAlbumPage
                            playlist=playlist_buff().0
                            set_playlist=playlist_buff().1
                            set_page=set_page
                        />
                    }
                }
                Pages::AddAlbum => {
                    set_cmd_text("cat | /bin/powershell -c './create_album'");
                    logging::log!("Bulding AddAlbum");
                    view! {
                        <CreateAlbumPage
                            set_page=set_page
                            set_playlists=set_reactive_playlists
                        />
                    }
                }
                Pages::CreateSong => {
                    set_cmd_text("cat | ./albums.txt");
                    logging::log!("Bulding CreateSong");
                    view! {
                        <AddSongPage
                            set_page=set_page
                            set_playlist=playlist_buff().1
                        />
                    }
                }
            }})
        }}
      </div>
    }
}
