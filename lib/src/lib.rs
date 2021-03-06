#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use stdweb::Value;
use stdweb::unstable::TryInto;
use stdweb::web::window;
use yew::services::timeout::TimeoutService;

static PREFIX: &str = "#slide-";

/// Data format for storing user-defined component state
#[derive(Debug)]
pub enum CustomData {
    Number(u64),
    String(String),
    StringRef(&'static str),
    Unit,
}

/// Represents a single slide.
///
/// Available slide types:
/// - Title: displays a string with big font
/// - Image: displays an image and a caption string
/// - List: a list of items with a title
/// - Code: a code snippet with a title
/// - Custom: a custom component with user-defined logic
#[derive(Clone)]
pub enum Slide {
    Title(String),
    Image(&'static str, Option<String>, String),
    List(String, Vec<String>),
    Code(String, String),
    Custom {
        title: String,
        init: Box<&'static Fn() -> CustomData>,
        update: Box<&'static Fn(&mut CustomData, CustomData, &mut Env<Registry, RootModel>) -> bool>,
        render: Box<&'static Fn(&CustomData) -> Html<Registry, RootModel>>,
    },
}

impl Slide {
    /// short-hand function for creating a title slide
    pub fn title(title: &str) -> Slide {
        Slide::Title(String::from(title))
    }

    /// short-hand function for creating a image slide
    pub fn image(resource: &'static str, text: &str) -> Slide {
        Slide::Image(resource, None, String::from(text))
    }

    /// short-hand function for creating a image slide
    pub fn image_with_title(resource: &'static str, title: &str, text: &str) -> Slide {
        Slide::Image(resource, Some(String::from(title)), String::from(text))
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

    /// short-hand function for creating a list slide
    pub fn custom(title: &str, init: &'static Fn() -> CustomData, update: &'static Fn(&mut CustomData, CustomData, &mut Env<Registry, RootModel>) -> bool, render: &'static Fn(&CustomData) -> Html<Registry, RootModel>) -> Slide {
        Slide::Custom {
            title: String::from(title),
            init: Box::new(init),
            update: Box::new(update),
            render: Box::new(render),
        }
    }
}

/// Represents a story (list of slides).
pub struct Story {
    pub slides: Vec<Slide>,
}

pub struct Registry {
    pub console: ConsoleService,
    story: Option<Story>,
    pub timeout: TimeoutService,
}

pub struct RootModel {
    story: Story,
    current_slide: usize,
    current_hash: String,
    #[allow(dead_code)]
    handle: Value,
    custom_data: CustomData,
}

pub enum RootMessage {
    Keydown(u32),
    Custom(CustomData),
}

impl Component<Registry> for RootModel {
    type Message = RootMessage;
    type Properties = ();

    fn create(_props: Self::Properties, context: &mut Env<Registry, Self>) -> Self {
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
        let current_slide = RootModel::get_location_slide().unwrap_or(0);
        let current_hash = RootModel::get_slide_hash(current_slide);
        RootModel::set_location_hash(&current_hash);
        let custom_data = match &story.slides[current_slide] {
            Slide::Custom { init, .. } => init(),
            _ => CustomData::Unit,
        };
        RootModel {
            story,
            current_slide,
            current_hash,
            handle,
            custom_data,
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Registry, Self>) -> bool {
        let slides_count = self.story.slides.len();
        match msg {
            RootMessage::Keydown(46) => {
                self.current_slide = (slides_count + self.current_slide + 1).min(slides_count + slides_count - 1) % slides_count;
                self.current_hash = RootModel::get_slide_hash(self.current_slide);
                RootModel::set_location_hash(&self.current_hash);
                let custom_data = match &self.story.slides[self.current_slide] {
                    Slide::Custom { init, .. } => init(),
                    _ => CustomData::Unit,
                };
                self.custom_data = custom_data;
                true
            }
            RootMessage::Keydown(44) => {
                self.current_slide = (slides_count + self.current_slide - 1).max(slides_count) % slides_count;
                self.current_hash = RootModel::get_slide_hash(self.current_slide);
                RootModel::set_location_hash(&self.current_hash);
                let custom_data = match &self.story.slides[self.current_slide] {
                    Slide::Custom { init, .. } => init(),
                    _ => CustomData::Unit,
                };
                self.custom_data = custom_data;
                true
            }
            RootMessage::Keydown(code) => {
                context.console.log(&format!("Unhandled key {}", code));
                false
            }
            RootMessage::Custom(data) => {
                match &self.story.slides[self.current_slide] {
                    Slide::Custom { update, .. } => {
                        (*update)(&mut self.custom_data, data, context)
                    }
                    _ => {
                        false
                    }
                }
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

    fn title_view(&self, string: &String) -> Html<Registry, RootModel> {
        html! {
          <h2> { string } </h2>
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
            (_, Slide::Image(resource, None, text)) => {
                html! {
                <div class="slide-wrapper",>
                  <div class="slide",class="image",>
                    <div class="content",>
                      <img src=resource, />
                      <p>
                        <div>
                          { text }
                        </div>
                      </p>
                    </div>
                  </div>
                </div>
                }
            }
            (_, Slide::Image(resource, Some(title), text)) => {
                html! {
                <div class="slide-wrapper",>
                  <div class="slide",class="image",>
                    <div class="content",>
                      <p>
                        { self.title_view(title) }
                      </p>
                      <img src=resource, />
                      <p>
                        <div>
                          { text }
                        </div>
                      </p>
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
                        { self.title_view(title) }
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
                      { self.title_view(title) }
                      <pre><code> { code } </code></pre>
                    </div>
                  </div>
                </div>
                }
            }
            (_, Slide::Custom { render, title, .. }) => {
                html! {
                <div class="slide-wrapper",>
                  <div class="slide",class="code",>
                    <div class="content",>
                      { self.title_view(title) }
                      { (*render)(&self.custom_data) }
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
        timeout: TimeoutService::new(),
    };
    let app = App::<Registry, RootModel>::new(registry);
    app.mount_to_body();
    yew::run_loop();
}

impl RootModel {
    fn get_location_slide() -> Option<usize> {
        window().location()
            .and_then(|l| l.hash().ok())
            .filter(|h| h.starts_with(PREFIX))
            .and_then(|h| h[PREFIX.len()..].parse::<usize>().ok())
    }

    fn get_slide_hash(slide: usize) -> String {
        format!("{}{}", PREFIX, slide)
    }

    fn set_location_hash(hash: &str) {
        js! {
          window.location.hash = @{hash};
        }
    }
}
