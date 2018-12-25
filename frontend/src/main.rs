use slides::run;
use slides::Story;
use slides::Slide;

fn main() {
    run(
        Story {
            slides: vec!(
                Slide::Text(String::from("Slides engine in rust")),
                Slide::Image("rust-logo.png", String::from("Rust logo looks like this")),
                Slide::List(String::from("Rust strengths"), vec!(
                    String::from("Safety"),
                    String::from("Performance"),
                )),
                Slide::Code(String::from("Hello World in Rust"), String::from("fn main() {\n  println!(\"Hello World\");\n}")),
            )
        }
    );
}
