use leptos::{*, html::Input};
use web_sys::SubmitEvent;

use crate::{Genre::Genre, Song, Pages, Playlist::Playlist};

#[component]
pub fn CreateSongView(playlist: &'static mut Playlist, set_page: WriteSignal<Pages>) -> impl IntoView {
    let title:      NodeRef<Input> = create_node_ref();
    let author:     NodeRef<Input> = create_node_ref();
    let genre:      NodeRef<Input> = create_node_ref();
    let duration:   NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let new_song = Song::Song {
            title:      title().unwrap().value(),
            duration:   duration().unwrap().value().parse().unwrap(),
            author:     author().unwrap().value(),

            genre:      match genre().unwrap().value().as_str() {
                            "pop"   => { Genre::Pop },
                            "rock"  => { Genre::Rock },
                            "edm"   => { Genre::EDM },
                            "jazz"  => { Genre::Jazz },
                            _ => { 
                                logging::error!("unmatched genre during create album sumbision"); 
                                Genre::Pop 
                            },
            }
        };

        playlist.add_song(new_song);
    };
            

    view! {
        <form on:submit=on_submit class="hackerfont text-white">
          <input type="text"    value="Title"   node_ref=title/>
          <input type="text"    value="Author"  node_ref=author/>
          <input type="number"                  node_ref=duration min="1" max="5"/>

          <input type="radio"   value="Pop"     node_ref=genre id="pop"/>
          <input type="radio"   value="Rock"    node_ref=genre id="rock"/>
          <input type="radio"   value="EDM"     node_ref=genre id="edm"/>
          <input type="radio"   value="Jazz"    node_ref=genre id="jazz"/>
          <input type="submit"  value="Sumbit"/>
        </form>
    }
}
