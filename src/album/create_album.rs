use crate::{album::Playlist, body::Pages, main_menu::ReturnToMainMenu, song::Genre};
use leptos::html::Input;
use leptos::*;
use web_sys::SubmitEvent;

/// A page/view to create new albums/playlists.
/// There are inputs for Author (optional), Genre (required), and Title (required)
///
/// # Arguments
/// * `set_page` the set_page [global state](crate::body::Body)
/// * `set_playlists` the [WriteSignal] for the [Vec] of ([ReadSignal], [WriteSignal]) pairs
#[component]
pub fn CreateAlbumPage(
    set_page: WriteSignal<Pages>,
    set_playlists: WriteSignal<Vec<(ReadSignal<Playlist>, WriteSignal<Playlist>)>>,
) -> impl IntoView {
    let title: NodeRef<Input> = create_node_ref();
    let author: NodeRef<Input> = create_node_ref();
    let genre: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        // Don't change the page on submit
        ev.prevent_default();

        let new_playlist = Playlist::new(
            // This shouldn't error because no input would be ""
            title().expect("Error getting user input").value(),
            // If there is no input, Author is [None] not ""
            match author() {
                Some(author) => {
                    if author.value().trim() == "" {
                        None
                    } else {
                        Some(author.value())
                    }
                }
                None => None,
            },
            // Transform genre string into [Genre] Enum
            match genre()
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
        );

        // update the global playlist
        set_playlists.update(|p| p.push(create_signal(new_playlist)));
        // go back to the main menu
        set_page.set(Pages::MainMenu);
    };

    view! {
        <form on:submit=on_submit class="hackerfont w-full flex flex-col justify-center items-center">
          // # Title and Author section
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
              <label class="block uppercase tracking-wide text-gray-300 text-lg font-bold mb-2">
                Title
              </label>
              <input
                class="bg-gray-800 text-gray-300 focus:bg-gray-900 rounded"
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
                class="bg-gray-800 text-gray-300 focus:bg-gray-900 rounded"
                type="text"
                value="Author"
                node_ref=author
              /> <br/>
            </div>
          </div>

          // # Genre Section
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
