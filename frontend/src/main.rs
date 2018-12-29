use slides::run;
use slides::Story;
use slides::Slide;

fn main() {
    run(
        Story {
            slides: vec!(
                Slide::title("Slides engine in rust"),
                Slide::image("rust-logo.png", "Rust logo looks like this"),
                Slide::list("Rust strengths", &[
                    "Safety",
                    "Performance",
                ]),
                Slide::code("Hello World in Rust", "fn main() {\n  println!(\"Hello World\");\n}"),
            )
        }
    );
}
