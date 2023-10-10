use std::{time::Duration};
use leptos::*;
mod typing;
use typing::TypedText;


#[component]
fn Header() -> impl IntoView {
    view! {
      <nav class="text-3xl flex justify-center items-center w-screen pt-12">
        <h1 class="hackerfont text-center text-green-500">
          <TypedText text="ETSU CYBERSECURITY CLUB"/>
        </h1>
      </nav>
    }
}

#[component]
fn Body(
    children: Children
) -> impl IntoView {
    view! {
      <body style="background-color: #111">
        {children()}
      </body>
    }
}


#[component]
fn App() -> impl IntoView {
    view! {
        <Body>
          <Header/>
        </Body>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
