use leptos::*;

use blazor_website::{body::Body, header::Header};

// Here is the entry point to our website. The component
// macro will transform this code into a suitible structure
// for leptos
#[component]
fn App() -> impl IntoView {
    // IntoView means a renderable element
    // This gives better error messages
    extern crate console_error_panic_hook;
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // This will pull the header and body custom elements
    // Note this does not have any reactivity in this part
    view! {
      <html>
        <Header/>
        <Body/>
      </html>
    }
}

fn main() {
    // Simply mounts the entry point.
    leptos::mount_to_body(App);
}
