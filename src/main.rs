use leptos::{*};

mod section;
mod typing;

use section::{Section, TypingConfig, TypedSection};
use typing::TypedText;


#[component]
fn Header() -> impl IntoView {
    view! {
      <nav style="background-color: #222" class="text-4xl flex justify-center items-center w-screen pt-12 pb-6">
        <h1 class="hackerfont text-center text-green-500">
          <TypedText centered=true interval=275 stop=false text="''Blazor App''" />
        </h1>
      </nav>
    }
}

#[component]
fn Aboutus() -> impl IntoView {
    let cmd_text = "msbuild ./CSharpApp.sln";
    let cmd_interval = 80;
    let hero_delay = cmd_text.len() as u64 * cmd_interval + 20;

    const hero_typing_cfg: TypingConfig = TypingConfig {
        header_interval: 70,
        text_interval: 7,
    };

    let view_album = Section {
        header: "View Album",
        text:   "",
        delay:  hero_delay,
        cfg:    &hero_typing_cfg,
    };
    let create_album = Section {
        header: "Create Album",
        text:   "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
        delay:  view_album.header.len() as u64 * 70 
              + view_album.text.len() as u64 * 7
              + view_album.delay + 20,
        cfg:    &hero_typing_cfg,
    };
    let edit_album = Section {
        header: "Edit Album",
        text:   "Edit an existing album, including 1. adding and deleting songs and 2. deleting an album",
        delay:  create_album.delay + create_album.text.len() as u64,
        cfg:    &hero_typing_cfg,
    };

    view! {
      <div class="px-8 text-gray-200">
        <p class="hackerfont text-lg">
          <span class="text-red-500">
            "[root@csc.etsu.edu /docs/aboutus]# "
          </span>
          <span>
            <TypedText centered=false interval={cmd_interval} stop=true text={cmd_text} />
          </span>
        </p>
        <div class="flex hackerfont text-gray-200 text-base px-8">
          <TypedSection base=&view_album/>
          <TypedSection base=&create_album/>
        </div>
      </div>
    }
}


#[component]
fn Main(
    children: Children
) -> impl IntoView {
    view! {
      <div>
        {children()}
      </div>
    }
}


#[component]
fn App() -> impl IntoView {
    view! {
        <Main>
          <Header/>
          <Aboutus/>
        </Main>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
