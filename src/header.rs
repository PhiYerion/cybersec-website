use crate::{body::Pages, utils::TypedText};
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    logging::log!("Creating header.");

    // We are going to unsafely circumvent page-check safeguards as we know that this header
    // will not change. Be warned if you change this component.
    let (fakepage, _) = create_signal(Pages::MainMenu);

    view! {
      <nav style="background-color: #222" class="text-6xl flex justify-center items-center w-screen pt-12 pb-6">
        <h1 class="hackerfont text-center text-green-500">
          <TypedText
            centered=true
            interval=275
            stop=false
            text="''Blazor App''".to_string()
            current_page=fakepage
          />
        </h1>
      </nav>
    }
}
