#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use stdweb::Value;
use stdweb::unstable::TryInto;

/// Represents a single slide.
///
/// Available slide types:
/// - Title: displays a string with big font
/// - Image: displays an image and a caption string
/// - List: a list of items with a title
/// - Code: a code snipped with a title
#[derive(Clone)]
pub enum Slide {
    Title(String),
    Image(&'static str, String),
    List(String, Vec<String>),
    Code(String, String),
}

impl Slide {
    /// short-hand function for creating a title slide
    pub fn title(title: &str) -> Slide {
        Slide::Title(String::from(title))
    }

    /// short-hand function for creating a image slide
    pub fn image(resource: &'static str, text: &str) -> Slide {
        Slide::Image(resource, String::from(text))
    }

    /// short-hand function for creating a list slide
    pub fn list(title: &str, list: &[&'static str]) -> Slide {
        let items = list.iter().map(|s| String::from(*s)).collect();
        Slide::List(String::from(title), items)
    }

    /// short-hand function for creating a list slide
    pub fn code(title: &str, code: &str) -> Slide {
        Slide::Code(String::from(title), String::from(code))
    }
}

/// Represents a story (list of slides).
pub struct Story {
    pub slides: Vec<Slide>,
}

struct Registry {
    console: ConsoleService,
    story: Option<Story>,
}

struct RootModel {
    story: Story,
    current_slide: usize,
    #[allow(dead_code)]
    handle: Value,
}

enum RootMessage {
    Keydown(u32),
}

impl Component<Registry> for RootModel {
    type Message = RootMessage;
    type Properties = ();

    fn create(_props: <Self as Component<Registry>>::Properties, context: &mut Env<Registry, Self>) -> Self {
        let callback = context.send_back(|code: u32| RootMessage::Keydown(code));
        let js_callback = move |v: Value| { callback.emit(v.try_into().unwrap()) };
        let handle = js! {
          var cb = @{js_callback};
          return document.addEventListener("keypress", function (e) {
            console.log(e.keyCode);
            cb(e.keyCode);
          })
        };
        let story = context.story.take().unwrap_or_else(|| Story { slides: vec!() });
        RootModel {
            story,
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

impl RootModel {
    fn list_item_view(&self, string: &String) -> Html<Registry, RootModel> {
        html! {
          <li> { string } </li>
        }
    }
}

impl Renderable<Registry, RootModel> for RootModel {
    fn view(&self) -> Html<Registry, RootModel> {
        let current_slide = &self.story.slides[self.current_slide];
        match (self.story.slides.len(), current_slide) {
            (0, _) => {
                html! {
                <div class="slide-wrapper",>
                  <div class="slide",class="empty",>
                    { "Nothing to display" }
                  </div>
                </div>
                }
            }
            (_, Slide::Title(string)) => {
                html! {
                <div class="slide-wrapper",>
                  <div class="slide",class="text",>
                    <div class="content",>
                      <h2>
                        { string }
                      </h2>
                    </div>
                  </div>
                </div>
                }
            }
            (_, Slide::Image(resource, string)) => {
                html! {
                <div class="slide-wrapper",>
                  <div class="slide",class="image",>
                    <div class="content",>
                      <img src=resource, />
                      <div>
                        { string }
                      </div>
                    </div>
                  </div>
                </div>
                }
            }
            (_, Slide::List(title, list)) => {
                html! {
                <div class="slide-wrapper",>
                  <div class="slide",class="list",>
                    <div class="content",>
                        <h2> { title } </h2>
                        <ul> { for list.iter().map(|i| self.list_item_view(i)) } </ul>
                    </div>
                  </div>
                </div>
                }
            }
            (_, Slide::Code(title, code)) => {
                html! {
                <div class="slide-wrapper",>
                  <div class="slide",class="code",>
                    <div class="content",>
                      <div> { title } </div>
                      <pre><code> { code } </code></pre>
                    </div>
                  </div>
                </div>
                }
            }
        }
    }
}

/// Run slides engine with provided story.
pub fn run(story: Story) {
    yew::initialize();
    let registry = Registry {
        console: ConsoleService::new(),
        story: Some(story),
    };
    let app = App::<Registry, RootModel>::new(registry);
    app.mount_to_body();
    yew::run_loop();
}
