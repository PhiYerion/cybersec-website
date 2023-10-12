use leptos::{*};
use crate::typing::{TypedText};

#[derive(Debug, Clone, Copy)]
pub struct Section {
    pub header:     ReadSignal<String>,
    pub set_header: WriteSignal<String>,
    pub text:       ReadSignal<String>,
    pub set_text:   WriteSignal<String>,
    pub cfg:        &'static TypingConfig,
    pub delay:      u64,
}

impl Section {
    pub fn new(header: String, text: String, delay: u64, cfg: &'static TypingConfig) -> Self {
        let (header, set_header) = create_signal(header);
        let (text, set_text) = create_signal(text);
        Section {
            header,
            set_header,
            text,
            set_text,
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
    pub text_interval: u64,
}

#[component]
pub fn TypedSection(base: Section) -> impl IntoView {
    let header_interval = 70;
    let text_interval = 7;

    logging::log!("Creating section with {:?}", base);
    let text_delay = base.delay + (base.header)().len() as u64 * header_interval + 20;

    view! {
      <div class="px-6">
        <h2 class="text-4xl">
          {move || base.header.with(|header| {
              view! {
                <TypedText 
                  text={header.to_string()} 
                  delay={base.delay} 
                  interval={base.header} 
                  centered=false 
                  stop=true 
                />
              }
          })}
        </h2>
        <p class="text-xl">
          {move || base.text.with(|text| {
              view! {
                <TypedText 
                  text={text.to_string()} 
                  delay=text_delay
                  interval={text_interval} 
                  centered=false 
                  stop=true 
                />
              }
        })}
        </p>
      </div>
    }
}
