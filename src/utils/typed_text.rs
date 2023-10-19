use leptos::{leptos_dom::helpers::IntervalHandle, *};
use std::time::Duration;
use wasm_bindgen::JsValue;

/// Starts a blinking cursur using [leptos_dom::helpers::set_interval_with_handle], a javascript
/// wrapper.
///
/// # Arguements
/// `set_cursor` - a write signal to the cursor to update
///
/// # Returns
/// Either
/// 1. An error in form of a [JsValue] **or**
/// 2. An [IntervalHandle] to stop the loop if
/// desired.
///
/// # Panics
/// This function may panic due to the instability with the cross between the JS and Rust.
pub fn start_cursor(set_cursor: WriteSignal<&'static str>) -> Result<IntervalHandle, JsValue> {
    logging::log!("starting cursor");
    set_interval_with_handle(
        move || set_cursor.update(|x| if *x == "" { *x = "|" } else { *x = "" }),
        Duration::from_millis(400),
    )
}

/// Component that types out text supplied to it with a blinking cursor
///
/// # Arguements
/// ##### Required
/// * `text`     - Text to be typed
/// * `interval` - Miliseconds between each letter
/// ##### Optional
/// * `centered` - (Def: false) Places a mock cursor to offset blinking cursor
/// * `stop`     - (Def: false) Stops the cursor after text has been rendered
/// * `delay`    - (Def: 0) [Delay in milliseconds](Duration::from_millis) before typing
///
/// # Panics
/// This function contains timeouts and intervals which have caused problems in the past when this
/// function goes out of scope but the variables do not.
///
/// # Notes
/// This function creates [leptos_dom::helpers::set_timeout] and
/// [leptos_dom::helpers::set_interval_with_handle]. These call out to JS functions and don't play
/// nice with the safe rust side becuase both rust and JS are dealing with DOM values created in rust with
/// [leptos::create_signal]

#[component]
pub fn TypedText(
    text: String,
    interval: u64,
    #[prop(default = false)] centered: bool,
    #[prop(default = false)] stop: bool,
    #[prop(default = 0)] delay: u64,
) -> impl IntoView {
    logging::log!("Creating typed effect with {}", text);

    // Signal for when text is finished. This is triggered by a set_timeout function later.
    let (text_finished, set_text_finished) = create_signal(false);

    // Text that is being rendered.
    let (text_render, set_text_render) = create_signal(String::new());
    // The blinking cursor at the end of the line.
    let (cursor, set_cursor) = create_signal("");

    //                              Add 50% to make sure  ~~~~~~~
    let time_to_run: u64 = (text.len() as u64 * (interval * 3 / 2))
        .try_into() // Try to convert to u64. Should be fool proof.
        .expect("typing ttl parsing error");

    set_timeout(
        move || {
            // The of text text_render is at. DO NOT MOVE THIS OUT OF THE CLOSURE.
            // This closure will make sure that it doesn't get disposed of at the
            // end of TypedText.
            // ```
            // "hello world" <= text
            // "hell"        <= text_render
            // ```
            // at index = 3 or 4 depending on the stage of the code.
            let (index, set_index) = create_signal(0usize);

            // handle of the cursor JS loop
            let cursor_handle = start_cursor(set_cursor).expect("unable to start cursor");

            // Loop that handles the typing of the text
            let typer = set_interval_with_handle(
                move || {
                    if index() >= text.len() {
                        return;
                    };

                    // Add text[index] to text_render
                    set_text_render.update(|s| {
                        (*s).push(
                            text.chars()
                                .nth(index())
                                .expect("text out of bounds on typer"),
                        )
                    });

                    // increment text_render
                    set_index.update(|idx| *idx += 1);
                },
                Duration::from_millis(interval),
            )
            .expect("Unable to create JS interval @ typing.rs:16");

            // Timeout function to stop the handle and cursor (if stop == true)
            set_timeout(
                move || {
                    logging::log!("Stopping typer handle({:?})", typer);
                    typer.clear();
                    set_text_finished.update(|x| *x = true);
                    if stop {
                        cursor_handle.clear();
                        set_cursor.update(|x| *x = "")
                    }
                },
                Duration::from_millis(time_to_run),
            );
        },
        Duration::from_millis(delay),
    );

    view! {
      // The invisible element here adds the exact same cursor to the other side of the element to
      // offset the one to the right. This keeps elements centered, but if they are aligned it will
      // make them move weirdly.
      <span class="invisible">
        {move ||
            if text_finished() || !centered
                {""}
            else
                {cursor()}
        }
      </span>
      {text_render}
      <span class="text-gray-300">
        {cursor}
      </span>
    }
}
