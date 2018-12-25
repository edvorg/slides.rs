#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

use yew::prelude::*;
use stdweb::web::document;
use stdweb::web::INonElementParentNode;
use yew::services::console::ConsoleService;
use stdweb::Value;
use stdweb::unstable::TryInto;

#[derive(Clone)]
enum Slide {
    Text(String),
}

struct Story {
    slides: Vec<Slide>,
}

struct Registry {
    console: ConsoleService,
}

struct RootModel {
    story: Story,
    current_slide: usize,
    #[allow(dead_code)]
    handle: Value,
}

enum RootMessage {
    Keydown(u8),
}

impl Component<Registry> for RootModel {
    type Message = RootMessage;
    type Properties = ();

    fn create(_props: <Self as Component<Registry>>::Properties, context: &mut Env<Registry, Self>) -> Self {
        let callback = context.send_back(|code: u8| RootMessage::Keydown(code));
        let js_callback = move |v: Value| { callback.emit(v.try_into().unwrap()) };
        let handle = js! {
          var cb = @{js_callback};
          return document.addEventListener("keypress", function (e) {
            console.log(e.keyCode);
            cb(e.keyCode);
          })
        };
        RootModel {
            story: Story {
                slides: vec!(
                    Slide::Text(String::from("slide one")),
                    Slide::Text(String::from("slide two")),
                    Slide::Text(String::from("slide three")),
                )
            },
            current_slide: 0,
            handle,
        }
    }

    fn update(&mut self, msg: <Self as Component<Registry>>::Message, context: &mut Env<Registry, Self>) -> bool {
        let slides_count = self.story.slides.len();
        match msg {
            RootMessage::Keydown(46) => {
                self.current_slide = (slides_count + self.current_slide + 1).min(slides_count + slides_count - 1) % slides_count;
                true
            }
            RootMessage::Keydown(44) => {
                self.current_slide = (slides_count + self.current_slide - 1).max(slides_count) % slides_count;
                true
            }
            RootMessage::Keydown(code) => {
                context.console.log(&format!("Unhandled key {}", code));
                false
            }
        }
    }
}

impl Renderable<Registry, RootModel> for RootModel {
    fn view(&self) -> Html<Registry, RootModel> {
        let current_slide = &self.story.slides[self.current_slide];
        match current_slide {
            Slide::Text(string) => {
                html! {
                <div class="slide-wrapper",>
                  <div class="slide",>
                    { string }
                  </div>
                </div>
                }
            }
        }
    }
}

pub fn run() {
    yew::initialize();
    let registry = Registry {
        console: ConsoleService::new(),
    };
    let app = App::<Registry, RootModel>::new(registry);
    app.mount(document().get_element_by_id("app").expect("div with id app not found"));
    yew::run_loop();
}
