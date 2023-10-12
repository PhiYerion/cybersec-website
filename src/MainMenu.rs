use leptos::*;

use crate::{section::{TypingConfig, TypedSection, Section}, Pages, TyperButton};

#[component]
pub fn View(delay: u64, set_page: WriteSignal<Pages>) -> impl IntoView {
    const hero_typing_cfg: TypingConfig = TypingConfig {
        header_interval: 70,
        text_interval: 7,
    };

    let view_album = Section::new(
        "View Album".to_string(),
        "Display songs and other album content".to_string(),
        delay,
        &hero_typing_cfg,
    );
    let create_album = Section::new(
        "Create Album".to_string(),
        "Create a new album, optionally with songs".to_string(),
        view_album.time_to_display(),
        &hero_typing_cfg,
    );
    let edit_album = Section::new(
        "Edit Album".to_string(),
        "Add and remove songs from albums. You can also delete albums here".to_string(),
        create_album.time_to_display(),
        &hero_typing_cfg,
    );

    view! {
      <div class="px-8 text-gray-200">
        <div class="flex hackerfont text-gray-200 text-base items-center self-center px-[20vw] pt-8">
          <TyperButton on:click=move |_| {set_page(Pages::MainMenu)}>
            <TypedSection base=view_album/>
          </TyperButton>
          <TyperButton on:click=move |_| {set_page(Pages::AddAlbum)}>
            <TypedSection base=create_album/>
          </TyperButton>
          <TyperButton>
            <TypedSection base=edit_album/>
          </TyperButton>
        </div>
      </div>
    }
}
