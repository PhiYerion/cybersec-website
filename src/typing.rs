use leptos::{leptos_dom::helpers::IntervalHandle, *};
use std::time::Duration;
use wasm_bindgen::JsValue;

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
    #[prop(default = false)] centered: bool,
    #[prop(default = false)] mut stop: bool,
    #[prop(default = 0)] delay: u64,
) -> impl IntoView {
    logging::log!("Creating typed effect with {}", text);

    let (index, set_index) = create_signal(0usize);
    let (text_finished, set_text_finished) = create_signal(false);

    let (text_render, set_text_render) = create_signal(String::new());
    let (cursor, set_cursor) = create_signal("");

    let time_to_run: u64 = (text.len() as u64 * (interval * 3 / 2))
        .try_into()
        .expect("typing ttl error");

    set_timeout(
        move || {
            let cursor_handle = start_cursor(set_cursor).expect("unable to start cursor");

            let typer = set_interval_with_handle(
                move || {
                    logging::log!("0");
                    if index() >= text.len() {
                        return;
                    };
                    logging::log!("Idx: {:?}", index());
                    logging::log!("{}[{}]", &text[..5], index());
                    set_text_render.update(|s| {
                        (*s).push(
                            text.chars()
                                .nth(index())
                                .expect("text out of bounds on typer"),
                        )
                    });
                    logging::log!("1");

                    set_index.update(|idx| *idx += 1);
                    logging::log!("2");
                },
                Duration::from_millis(interval),
            )
            .expect("Unable to create JS interval @ typing.rs:16");

            set_timeout(
                move || {
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
