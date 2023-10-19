use leptos::*;

use blazor_website::{body::Body, Header};
use std::panic;

/// Constructs the frame of the website from a header and body.
// IntoView makes a renderable element, and impl makes an
// implimentation for each variety of IntoView
#[component]
fn App() -> impl IntoView {
    // This will pull the header and body custom elements
    // Note this does not have any reactivity in this part
    view! {
      <html>
        <Header/>
        <Body/>
      </html>
    }
}

/// Entry point to program.
/// # Error Handling
/// During [specific javascript calls with leptos](blazor_website::utils::TypedText), this program
/// can still encounters runtime errors and our use of [std::panic] amd [console_error_panic_hook] with the messages
fn main() {
    // This gives better error messages
    extern crate console_error_panic_hook;

    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // Simply mounts the entry point.
    leptos::mount_to_body(App);
}
