use std::{time::Duration};
use leptos::{*, leptos_dom::helpers::IntervalHandle};


fn create_cursor<'a>() -> (ReadSignal<&'a str>, WriteSignal<&'a str>, IntervalHandle) {
    let (cursor, set_cursor) = create_signal("I");

    let handler = set_interval_with_handle(move || {
        set_cursor.update(|x| {
        if *x == "|"    {*x = " "}
        else            {*x = "|"} 
        })
    }, Duration::from_millis(400));

    (cursor, set_cursor, handler.unwrap())
}

#[component]
pub fn TypedText(
        text: &'static str, 
        interval: u64, 
        #[prop(default = false)]
        centered: bool, 
        #[prop(default = false)]
        stop: bool,
        #[prop(default = 0)]
        delay: u64
    ) -> impl IntoView {

    let text_len: usize = text.len();

    let (text_render, set_text_render)      = create_signal(String::new());
    let (index, set_index)                  = create_signal(0usize);
    let (text_finished, set_text_finished)  = create_signal(false);
    let (cursor, set_cursor, cursor_handle) = create_cursor();
    
    let time_to_run: u64 = (text_len as u64 * (interval * 3 / 2) ).try_into().unwrap();
    
    set_timeout(move || {
        let typer = set_interval_with_handle (move || {

            if index() >= text_len { return };
            
            set_text_render.update(|s| 
                (*s).push(
                    text.chars().nth(index()).unwrap()
            ));

            set_index.update(|idx| *idx += 1);
        }, Duration::from_millis(interval));
        

        set_timeout(move || {
            typer.unwrap().clear();
            set_text_finished.update(|x| *x = true);
            if stop { cursor_handle.clear(); set_cursor.update(|x| *x = "") }
        }, Duration::from_millis(time_to_run));
    }, Duration::from_millis(delay));

    
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

