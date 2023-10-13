use leptos::*;

use crate::{section::{TypingConfig, TypedSection, Section}, Pages, TyperButton, typing::TypedText};

#[component]
pub fn ReturnToMainMenu(set_page: WriteSignal<Pages>) -> impl IntoView {
    view! {
      <div class="hackerfont text-white text-2xl flex items-center justify-center pt-10vh">
        <TyperButton on:click=move |_| {set_page(Pages::MainMenu)}>
          <TypedText
            text="ReturnToMainMenu".to_string()
            interval=7
            delay=1000
          />
        </TyperButton>
      </div>
    }
}

  

#[component]
pub fn MainMenuView(delay: u64, set_page: WriteSignal<Pages>) -> impl IntoView {
    const hero_typing_cfg: TypingConfig = TypingConfig {
        header_interval:  70,
        header_classes:   "text-2xl text-white",
        text_interval:    7,
        text_classes:     "text-xl text-gray-200",
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
        <div class="flex hackerfont text-gray-200 text-base items-center self-center px-[10vw] pt-8">
          <TyperButton on:click=move |_| {set_page(Pages::ViewAlbums)}>
            <TypedSection base=view_album/>
          </TyperButton>
          <TyperButton on:click=move |_| {set_page(Pages::AddAlbum)}>
            <TypedSection base=create_album/>
          </TyperButton>
          <TyperButton on:click=move |_| {set_page(Pages::EditAlbum)}>
            <TypedSection base=edit_album/>
          </TyperButton>
        </div>
      </div>
    }
}
