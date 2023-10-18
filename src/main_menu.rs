use leptos::*;

use crate::{
    body::Pages,
    section::{Section, TypedSection, TypingConfig},
    utils::PointerOnHover,
    utils::TypedText,
};

#[component]
pub fn ReturnToMainMenu(
    set_page: WriteSignal<Pages>,
    current_page: ReadSignal<Pages>,
) -> impl IntoView {
    view! {
      <div class="bottom-0 fixed w-full pb-20 hackerfont text-white text-4xl flex items-center justify-center pt-4">
        <PointerOnHover on:click=move |_| {set_page(Pages::MainMenu)}>
          <TypedText
            text="ReturnToMainMenu".to_string()
            interval=7
            delay=1000
            current_page=current_page
          />
        </PointerOnHover>
      </div>
    }
}

#[component]
pub fn MainMenuPage(
    delay: u64,
    set_page: WriteSignal<Pages>,
    current_page: ReadSignal<Pages>,
) -> impl IntoView {
    const hero_typing_cfg: TypingConfig = TypingConfig {
        header_interval: 70,
        header_classes: "text-4xl text-white",
        text_interval: 7,
        text_classes: "text-2xl text-gray-200",
    };

    let view_album = Section::new(
        "View Albums".to_string(),
        "View all albums with the choice to add, delete, or simply just view the album."
            .to_string(),
        delay,
        &hero_typing_cfg,
    );
    let create_album = Section::new(
        "Create Album".to_string(),
        "Create a new album with title, author, and genre.".to_string(),
        view_album.time_to_display(),
        &hero_typing_cfg,
    );

    view! {
      <div class="text-gray-200 flex hackerfont text-gray-200 text-base items-center self-center px-[25vw] pt-8">
        <div class="px-8">
          <PointerOnHover on:click=move |_| {set_page(Pages::ViewAlbumList)}>
            <TypedSection base=view_album current_page=current_page/>
          </PointerOnHover>
        </div>
        <div class="px-8">
          <PointerOnHover on:click=move |_| {set_page(Pages::AddAlbum)}>
            <TypedSection base=create_album current_page=current_page/>
          </PointerOnHover>
        </div>
      </div>
    }
}
