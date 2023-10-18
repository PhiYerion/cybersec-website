use crate::{body::Pages, utils::TypedText};
use leptos::*;

#[component]
/// Header
pub fn Header() -> impl IntoView {
    logging::log!("Creating header.");

    view! {
      <nav style="background-color: #222" class="text-6xl flex justify-center items-center w-screen pt-12 pb-6">
        <h1 class="hackerfont text-center text-green-500">
          <TypedText
            centered=true
            interval=275
            stop=false
            text="''Blazor App''".to_string()
          />
        </h1>
      </nav>
    }
}
