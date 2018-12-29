use slides::run;
use slides::Story;
use slides::Slide;

fn main() {
    run(
        Story {
            slides: vec!(
                Slide::title("Slides engine in Rust (press < or >)"),
                Slide::image("rust-logo.png", "We can display slides with images"),
                Slide::image_with_title("rust-logo.png", "And optional title", "depending on your needs"),
                Slide::code("Simple code snippets", "fn main() {\n  println!(\"Hello World\");\n}"),
                Slide::list("Full list of supported slide types", &[
                    "Title",
                    "Image",
                    "Code",
                    "List",
                ]),
            )
        }
    );
}
