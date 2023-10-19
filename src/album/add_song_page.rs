use leptos::{html::Input, *};
use web_sys::SubmitEvent;

use crate::{album::Playlist, body::Pages, main_menu::ReturnToMainMenu, song::Genre, song::Song};

/// Add a [Song] to a [Playlist].
/// Displays a series of input boxes to allow user to enter Author, [Genre], Duration, and Title
/// for the song to be added.
/// In current implimentation, [the album page](crate::album::ViewAlbumPage) redirects here through
/// setting the [global playlist buffer](crate::body::Body) and changing the [page](Pages)
///
/// # Arguements
/// * `set_playlist` - [WriteSignal] to playlist that will be added to
/// * `set_page` - [global set_page](crate::body::Body)

#[component]
pub fn AddSongPage(
    set_playlist: WriteSignal<Playlist>,
    set_page: WriteSignal<Pages>,
) -> impl IntoView {
    // Create the references that will be used to hold input
    let title: NodeRef<Input> = create_node_ref();
    let author: NodeRef<Input> = create_node_ref();
    let genre: NodeRef<Input> = create_node_ref();
    let duration: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        // Make sure that the page doesn't refresh or redirect
        ev.prevent_default();

        // Create a new song based on user input. None of the [NodeRef]s should error in here so
        // long as there is an input element for it (no input would be "")
        let new_song = Song {
            title: title().expect("Error getting user input").value(),
            duration: duration()
                .expect("Error getting user input")
                .value()
                .parse()
                .expect("Error parsing user input for duraction"),
            author: author().expect("Error getting user input").value(),

            genre: match genre()
                .expect("Error getting genre user input")
                .value()
                .as_str()
            {
                "Pop" => Genre::Pop,
                "Rock" => Genre::Rock,
                "Edm" => Genre::EDM,
                "Jazz" => Genre::Jazz,
                _ => {
                    logging::error!(
                        "unmatched genre \"{:?}\" during create album sumbision",
                        genre().unwrap().value().as_str()
                    );
                    Genre::Pop
                }
            },
        };

        // Add the new song
        set_playlist.update(|p| p.add_song(new_song));

        // Return to Main Menu
        set_page.set(Pages::MainMenu);
    };

    view! {
        <form on:submit=on_submit class="hackerfont w-full flex flex-col justify-center items-center">
          // # Title & Author
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
              <label class="block uppercase tracking-wide text-gray-300 text-lg font-bold mb-2">
                Title
              </label>
              <input
                class="bg-gray-800 text-gray-300 focus:bg-gray-900 rounded p-2"
                type="text"
                value="Title"
                node_ref=title
              /> <br/>
            </div>
            <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
              <label class="block uppercase tracking-wide text-gray-300 text-lg font-bold mb-2">
                Author
              </label>
              <input
                class="bg-gray-800 text-gray-300 focus:bg-gray-900 rounded p-2"
                type="text"
                value="Author"
                node_ref=author
              /> <br/>
            </div>
          </div>

          // # Duration
          <div class="mb-6">
            <label class="block uppercase tracking-wide text-gray-300 text-lg font-bold mb-2">
              Duration
            </label>
            <input
              class="bg-gray-800 text-gray-300 focus:bg-gray-900 w-16 h-8 rounded p-2"
              type="number"
              node_ref=duration
              min="1"
              max="10000"
            /> <br/>
          </div>

          // # Genre
          <div class="mb-12">
            <label class="block uppercase tracking-wide text-gray-300 text-lg font-bold mb-2">
              Genere
            </label>
            <div>
              <div class="flex-row flex">
                <input type="radio"   value="Pop"   id="pop"    node_ref=genre name="genre"/> <br/>
                <label class="ml-4 block uppercase tracking-wide text-gray-300 mb-1">
                  Pop
                </label>
              </div>
              <div class="flex-row flex">
                <input type="radio"   value="Rock"  id="rock"   node_ref=genre name="genre"/> <br />
                <label class="ml-4 block uppercase tracking-wide text-gray-300 mb-1">
                  Rock
                </label>
              </div>
              <div class="flex-row flex">
                <input type="radio"   value="EDM"   id="edm"    node_ref=genre name="genre"/> <br />
                <label class="ml-4 block uppercase tracking-wide text-gray-300 mb-1">
                  EDM
                </label>
              </div>
              <div class="flex-row flex">
                <input type="radio"   value="Jazz"  id="jazz"   node_ref=genre name="genre"/> <br />
                <label class="ml-4 block uppercase tracking-wide text-gray-300 mb-2">
                  Jazz
                </label>
              </div>
            </div>
          </div>

          // # Submit button
          <input
            class="block uppercase tracking-wide text-gray-300 text-xl font-bold mb-12 cursor-pointer"
            type="submit"
            value="Sumbit"/>
        </form>


        <ReturnToMainMenu set_page=set_page/>
    }
}
