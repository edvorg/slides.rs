use slides::run;
use slides::Story;
use slides::Slide;

fn main() {
    run(
        Story {
            slides: vec!(
                Slide::Text(String::from("Full-stack web development with Rust")),
                Slide::Image("rust-logo.png", String::from("Rust is a systems programming language that guarantees")),
                Slide::Text(String::from("Web assembly")),
                Slide::Text(String::from("Eduard Knyshov @edvorg")),
            )
        }
    );
}
