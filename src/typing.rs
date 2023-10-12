use std::{time::Duration};
use leptos::{*, leptos_dom::helpers::IntervalHandle};
use wasm_bindgen::JsValue;

pub fn start_cursor<'a>(set_cursor: WriteSignal<Option<&'a str>>) -> Result<IntervalHandle, JsValue> {
    logging::log!("starting cursor");
    set_interval_with_handle(move || {
        set_cursor.update(|x| {
            if *x == None   { *x = Some("|") } 
            else            { *x = None }
        })
    }, Duration::from_millis(400))
}

#[component]
pub fn TypedText(
    text: String, 
    interval: u64, 
    #[prop(default = false)]
    centered: bool, 
    #[prop(default = false)]
    stop: bool,
    #[prop(default = 0)]
    delay: u64,
) -> impl IntoView {
    logging::log!("Creating typed effect with {}", text);

    let (index, set_index)                  = create_signal(0usize);
    let (text_finished, set_text_finished)  = create_signal(false);

    let (text_render, set_text_render)      = create_signal(String::new());
    let (cursor, set_cursor)                = create_signal(None);

    let time_to_run: u64 = (text.len() as u64 * (interval * 3 / 2) ).try_into().unwrap();
    
    set_timeout(move || {
        let cursor_handle = start_cursor(set_cursor).unwrap();

        let typer = set_interval_with_handle (move || {

            if index() >= text.len() { return };
            logging::log!("{}[{}]", text, index());
            
            set_text_render.update(|s| 
                (*s).push(
                  text.chars().nth(index()).unwrap()
            ));

            set_index.update(|idx| *idx += 1);
        }, Duration::from_millis(interval));
        

        set_timeout(move || {
            typer.unwrap().clear();
            set_text_finished.update(|x| *x = true);
            if stop { cursor_handle.clear(); set_cursor.update(|x| *x = None) }
        }, Duration::from_millis(time_to_run));
    }, Duration::from_millis(delay));

    
    view! {
      // padding for when text is being made, but reserve cool effect
      // when text is drawn.
      <span class="invisible">
        {move || if (text_finished() || !centered) {None} else {cursor()}}
      </span>
      {text_render}
      <span class="text-gray-300">
        {cursor}
      </span>
    }
}

