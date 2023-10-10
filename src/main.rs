use std::{string, time::Duration};

use leptos::*;

fn create_cursor<'a>() -> ReadSignal<&'a str> {
    let (cursor, set_cursor) = create_signal("I");

    set_interval(move || {
        set_cursor.update(|x| {
        if *x == "|"    {*x = " "}
        else            {*x = "|"} 
        })
    }, Duration::from_millis(400));

    cursor
}

#[component]
fn TypedText(text: &'static str) -> impl IntoView {
    let text_len: usize = text.len();

    let (text_render, set_text_render)      = create_signal(String::new());
    let (index, set_index)                  = create_signal(0usize);
    let (text_finished, set_text_finished)  = create_signal(false);

    let duration: u64 = 275;
    let time_to_run: u64 = (text_len as u64 * (duration + 1) ).try_into().unwrap();

    let typer = set_interval_with_handle (move || {

        if index() >= text_len { return };
        
        set_text_render.update(|s| 
            (*s).push(
                text.chars().nth(index()).unwrap()
        ));

        set_index.update(|idx| *idx += 1);
    }, Duration::from_millis(duration));

    set_timeout(move || {
        typer.unwrap().clear();
        set_text_finished.update(|x| *x = true);
    }, Duration::from_millis(time_to_run));

    let cursor = create_cursor();
    
    view! {
      // padding for when text is being made, but reserve cool effect
      // when text is drawn.
      <span class="invisible">
        {move || if text_finished() {""} else {cursor()}}
      </span>
      {text_render}{cursor}
    }
}

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

fn Body() -> impl IntoView {
    view! {
      <body style="background-color: #111">

      </body>
    }


#[component]
fn App() -> impl IntoView {
    <Body>
      <Header/>
    <Body>
}

fn main() {
    leptos::mount_to_body(App)
}
