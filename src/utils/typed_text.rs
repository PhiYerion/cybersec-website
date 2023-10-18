use leptos::{leptos_dom::helpers::IntervalHandle, *};
use std::time::Duration;
use wasm_bindgen::JsValue;

use crate::body::Pages;

pub fn start_cursor(set_cursor: WriteSignal<&'static str>) -> Result<IntervalHandle, JsValue> {
    logging::log!("starting cursor");
    set_interval_with_handle(
        move || set_cursor.update(|x| if *x == "" { *x = "|" } else { *x = "" }),
        Duration::from_millis(400),
    )
}

#[component]
pub fn TypedText(
    text: String,
    interval: u64,
    current_page: ReadSignal<Pages>,
    #[prop(default = false)] centered: bool,
    #[prop(default = false)] stop: bool,
    #[prop(default = 0)] delay: u64,
) -> impl IntoView {
    logging::log!("Creating typed effect with {}", text);

    let (text_finished, set_text_finished) = create_signal(false);

    let (text_render, set_text_render) = create_signal(String::new());
    let (cursor, set_cursor) = create_signal("");

    let time_to_run: u64 = (text.len() as u64 * (interval * 3 / 2))
        .try_into()
        .expect("typing ttl error");

    let starting_page = current_page();

    set_timeout(
        move || {
            let (index, set_index) = create_signal(0usize);

            if current_page() != starting_page {
                return;
            }
            let cursor_handle = start_cursor(set_cursor).expect("unable to start cursor");

            let typer = set_interval_with_handle(
                move || {
                    if current_page() != starting_page {
                        logging::log!("{:?} != {:?} : stop", current_page(), starting_page);
                        return;
                    }
                    logging::log!("{:?} != {:?} : continue", current_page(), starting_page);

                    if index() >= text.len() {
                        return;
                    };
                    logging::log!("1");
                    logging::log!("Idx: {:?}", index());
                    logging::log!("{}[{}]", &text[..2], index());
                    set_text_render.update(|s| {
                        logging::log!("2");
                        (*s).push(
                            text.chars()
                                .nth(index())
                                .expect("text out of bounds on typer"),
                        )
                    });

                    set_index.update(|idx| *idx += 1);
                },
                Duration::from_millis(interval),
            )
            .expect("Unable to create JS interval @ typing.rs:16");

            set_timeout(
                move || {
                    if current_page() != starting_page {
                        return;
                    }
                    logging::log!("Stopping cursor handle({:?})", typer);
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
      // padding for when text is being made, but reserve cool effect
      // when text is drawn.
      <span class="invisible">
        {move || if (text_finished() || !centered) {""} else {cursor()}}
      </span>
      {text_render}
      <span class="text-gray-300">
        {cursor}
      </span>
    }
}
