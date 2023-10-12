use leptos::{*};

mod section;
mod typing;
mod MainMenu;
mod ViewAlbum;
mod Song;
mod Playlist;
mod EditAlbum;
mod CreateAlbum;
mod Genre;

use section::{Section, TypingConfig, TypedSection};
use typing::{TypedText};


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

enum Pages {
    MainMenu,
    AddAlbum,
    EditAlbum,
}

#[component]
fn Body() -> impl IntoView {
    logging::log!("Creating body element");
    let (cmd_text, set_cmd_text) = create_signal("msbuild ./vmlinuz-linux.sln");
    let cmd_interval = 80;
    let (page, set_page) = create_signal(Pages::MainMenu);

    view! {
      <div>
        <p class="hackerfont text-lg text-white">
          <span class="text-red-500">
            "[root@csci.etsu.edu /usr/src/linux-6.5.6-gentoo]# "
          </span>
          <span>
            {move || {cmd_text.with(|cmd| {
              view! {<TypedText centered=false interval={cmd_interval} stop=true text={cmd.to_string()} />}
            })}}
          </span>
        </p>
        {move || {
            page.with(|p| match p {
                Pages::MainMenu => {
                    set_cmd_text("msbuild ./linux.sln");
                    logging::log!("Building MainMenu");
                    MainMenu::View(MainMenu::ViewProps{ 
                        delay: cmd_text().len() as u64 * cmd_interval + 20,
                        set_page,
                    })
                } 
                _ => {
                    set_cmd_text("ms bufjdik alsjkfl ild ./linux.sln");
                    MainMenu::View(MainMenu::ViewProps{ 
                        delay: cmd_text().len() as u64 * cmd_interval + 20,
                        set_page,
                    })
                }
            })
        }}
      </div>
    }
}


#[component]
fn App() -> impl IntoView {
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
