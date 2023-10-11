use leptos::{*};

mod typing;

use typing::TypedText;


#[component]
fn Header() -> impl IntoView {
    view! {
      <nav style="background-color: #222" class="text-4xl flex justify-center items-center w-screen pt-12 pb-6">
        <h1 class="hackerfont text-center text-green-500">
          <TypedText centered=true interval=275 stop=false text="ETSU CYBERSECURITY CLUB" />
        </h1>
      </nav>
    }
}

struct Section {
  header: &'static str,
  text: &'static str,
  delay: u64,
}

#[component]
fn TypedSection(base: Section) -> impl IntoView {
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

#[component]
fn Aboutus() -> impl IntoView {
    let cmd_text = "cat README.md";
    let cmd_interval = 4;
    let hero_delay = cmd_text.len() as u64 * cmd_interval;

    let sectionOne = Section {
        header: "test header",
        text: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
        delay: 89,
    };
    let sectionTwo = Section {
        header: "test header two",
        text: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
        delay: sectionOne.header.len() as u64 * 70 
             + sectionOne.text.len() as u64 * 7
             + sectionOne.delay + 20,
    };

    view! {
      <div class="px-8 text-gray-200">
        <p class="hackerfont text-lg">
          <span class="text-red-500">
            "[root@csc.etsu.edu /docs/aboutus]# "
          </span>
          <span>
            <TypedText centered=false interval={cmd_interval} stop=true text={cmd_text} />
          </span>
        </p>
        <div class="flex hackerfont text-gray-200 text-base px-8">
          <TypedSection base=sectionOne/>
          <TypedSection base=sectionTwo/>
        </div>
      </div>
    }
}


#[component]
fn Main(
    children: Children
) -> impl IntoView {
    view! {
      <div>
        {children()}
      </div>
    }
}


#[component]
fn App() -> impl IntoView {
    view! {
        <Main>
          <Header/>
          <Aboutus/>
        </Main>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
