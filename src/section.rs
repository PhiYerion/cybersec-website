use crate::body::Pages;
use crate::utils::TypedText;
use leptos::*;

#[derive(Debug, Clone, Copy)]
pub struct Section {
    pub header: ReadSignal<String>,
    pub set_header: WriteSignal<String>,
    pub header_delay: ReadSignal<u64>,
    pub set_header_delay: WriteSignal<u64>,

    pub text: ReadSignal<String>,
    pub set_text: WriteSignal<String>,
    pub text_delay: ReadSignal<u64>,
    pub set_text_delay: WriteSignal<u64>,

    pub cfg: &'static TypingConfig,
    pub delay: u64,
}

impl Section {
    pub fn new(header: String, text: String, delay: u64, cfg: &'static TypingConfig) -> Self {
        let (text, set_text) = create_signal(text);
        let (text_delay, set_text_delay) =
            create_signal(delay + header.len() as u64 * cfg.header_interval + 20);

        let (header, set_header) = create_signal(header);
        let (header_delay, set_header_delay) = create_signal(delay);
        Section {
            header,
            set_header,
            header_delay,
            set_header_delay,
            text,
            set_text,
            set_text_delay,
            text_delay,
            cfg,
            delay,
        }
    }
    pub fn time_to_display<'a>(self: &Section) -> u64 {
        (self.header)().len() as u64 * self.cfg.header_interval
            + (self.text)().len() as u64 * self.cfg.text_interval
            + self.delay
    }
}

#[derive(Debug)]
pub struct TypingConfig {
    pub header_interval: u64,
    pub header_classes: &'static str,
    pub text_interval: u64,
    pub text_classes: &'static str,
}

#[component]
pub fn TypedSection(base: Section, current_page: ReadSignal<Pages>) -> impl IntoView {
    let header_interval = 70;
    let text_interval = 7;

    logging::log!("Creating section with {:?}", base);

    view! {
      <div class="px-6">
        <h2 class={base.cfg.header_classes}>
          {move || base.header.with(|header| {
              let delay = (base.header_delay)();
              (base.set_header_delay)(0);
              view! {
                <TypedText
                  text={header.to_string()}
                  delay=delay
                  interval=header_interval
                  centered=false
                  stop=true
                  current_page=current_page
                />
              }
          })}
        </h2>
        <p class={base.cfg.text_classes}>
          {move || base.text.with(|text| {
              let delay = (base.text_delay)();
              (base.set_text_delay)(0);
              view! {
                <TypedText
                  text={text.to_string()}
                  delay=delay
                  interval=text_interval
                  centered=false
                  stop=true
                  current_page=current_page
                />
              }
        })}
        </p>
      </div>
    }
}
