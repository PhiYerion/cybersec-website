use leptos::*;
use crate::typing::TypedText;

pub struct Section {
    pub header: &'static str,
    pub text: &'static str,
    pub cfg: &'static TypingConfig,
    pub delay: u64,
}

pub struct TypingConfig {
    pub header_interval: u64,
    pub text_interval: u64,
}

impl Section {
    pub fn timeToDisplay<'a>(self: Section) -> u64 {
        return (
            self.header.len() as u64 * self.cfg.header_interval
          + self.text.len() as u64 * self.cfg.text_interval
          + self.delay
        );
    }
}

#[component]
pub fn TypedSection<'a>(base: &'a Section) -> impl IntoView {
    let header_interval = 70;
    let text_interval = 7;

    view! {
      <div class="px-6">
        <h2 class="text-lg">
          <TypedText 
            text={base.header} 
            delay={base.delay} 
            interval={header_interval} 
            centered=false 
            stop=true 
          />
        </h2>
        <TypedText 
          text={base.text} 
          delay={base.delay + base.header.len() as u64 * header_interval + 20} 
          interval={text_interval} 
          centered=false 
          stop=true 
        />
      </div>
    }
}
