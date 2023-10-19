use crate::body::Pages;
use crate::utils::TypedText;
use leptos::*;

/// A struct with configuration values for [TypedText]
#[derive(Debug)]
pub struct TypingConfig {
    /// Milliseconds between each character typed for header.
    pub header_interval: u64,
    /// CSS classes for the header. Tailwind included.
    pub header_classes: &'static str,
    /// Milliseconds between each character typed for text.
    pub text_interval: u64,
    /// CSS classes for  the text. Tailwind included.
    pub text_classes: &'static str,
}

/// A base configuration for a section to provide ease of reuse.
#[derive(Debug, Clone, Copy)]
pub struct Section {
    /// A reactive header
    pub header: ReadSignal<String>,
    pub set_header: WriteSignal<String>,

    /// Delay to render header in milliseconds
    pub header_delay: ReadSignal<u64>,
    pub set_header_delay: WriteSignal<u64>,

    /// Text under the header
    pub text: ReadSignal<String>,
    pub set_text: WriteSignal<String>,

    /// Delay to render the text in milliseconds
    pub text_delay: ReadSignal<u64>,
    pub set_text_delay: WriteSignal<u64>,

    /// [TypingConfig] for [TypedText]
    pub cfg: &'static TypingConfig,
}

impl Section {
    /// Creates a new Section config. See Fields.
    pub fn new(header: String, text: String, delay: u64, cfg: &'static TypingConfig) -> Self {
        // # Transform the input into reactive signals
        let (text, set_text) = create_signal(text);

        // Calculate the time it takes to complete the header + 20ms for the text delay
        let (text_delay, set_text_delay) =
            create_signal(delay + header.len() as u64 * cfg.header_interval + 20);

        // We shadow input header: String with header: ReadSignal<String> here.
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
        }
    }

    /// Reports the time it takes the section to display in milliseconds.
    ///
    /// # Accuracy
    /// There are no unit tests for this, but the author's eye can't tell any inaccuracy.
    pub fn time_to_display(self: &Section) -> u64 {
        (self.text_delay)() + (self.text)().len() as u64 * self.cfg.text_interval
    }
}

/// A [TypedText] header with a [TypedText] textbox under it.
/// Neither the header nor the text box will have a blinking cursor after done typing.
///
/// # Arguements
/// * [Section] - Values for TypedSection
/// # Examples
///
/// ```
/// use blazor_website::section::{Section, TypedSection}
///
/// let typing_cfg = &TypingConfig {
///     header_interval: 50,
///     header_classes: "text-xl text-gray-200",
///     text_interval: 5,
///     text_classes: "text-bg text-gray-300",
/// };
///
/// let section = Section::new(
///     "Hello World".to_string(),
///     "This is my first section".to_string(),
///     10,
///     typing_cfg,
/// );
///
/// TypedSection(Section);
/// ```

#[component]
pub fn TypedSection(base: Section) -> impl IntoView {
    let header_interval = 70;
    let text_interval = 7;

    logging::log!("Creating section with {:?}", base);

    view! {
      <div class="px-6">
        // # Header
        <h2 class={base.cfg.header_classes}>
          // On a change of a the header,
          {move || base.header.with(|header| {
              // Keep the old value for use in TypedText
              let delay = (base.header_delay)();
              // Set the delay to 0 so we dont have to wait for the delay if we decide to just
              // change the header later.
              (base.set_header_delay)(0);
              view! {
                <TypedText
                  text={header.to_string()}
                  delay=delay
                  interval=header_interval
                  centered=false
                  stop=true
                />
              }
          })}
        </h2>
        // # sub-text
        <p class={base.cfg.text_classes}>
          // On a change of the text,
          {move || base.text.with(|text| {
              // Keep the old value for use in TypedText
              let delay = (base.text_delay)();
              // Set the delay to 0 so we dont have to wait for the delay if we decide to just
              // change the header later.
              (base.set_text_delay)(0);
              view! {
                <TypedText
                  text={text.to_string()}
                  delay=delay
                  interval=text_interval
                  centered=false
                  stop=true
                />
              }
        })}
        </p>
      </div>
    }
}
