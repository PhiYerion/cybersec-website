use leptos::*;

/// Creates "-->" to the left of children during hover.
/// "-->" is centered and will not wrap.
///
/// We use [leptos::ev::mouseenter] and [leptos::ev::mouseleave], as these have proven to be the
/// mose reliable and least-triggered events for this purpose.
#[component]
pub fn PointerOnHover(children: Children) -> impl IntoView {
    logging::log!("Creating button with typer effect");
    let (arrow, set_arrow) = create_signal(None);

    view! {
      <button
        on:mouseenter=move |_| {
            set_arrow(Some("-->"));
        }
        on:mouseleave=move |_| {
            set_arrow(None);
        }
      >
        <div class="flex flex-row items-center">
          <span class="whitespace-nowrap flex-shrink-0 text-2xl">
            {arrow}
          </span>
          {children()}
        </div>
      </button>
    }
}
