use slides::run;
use slides::Story;
use slides::Slide;

fn main() {
    run(
        Story {
            slides: vec!(
                Slide::Text(String::from("Full-stack web development with Rust")),
                Slide::Image("rust-logo.png", String::from("Rust is a system programming language")),
                Slide::List(String::from("Rust strengths"), vec!(
                    String::from("Safety"),
                    String::from("Performance"),
                )),
                Slide::Text(String::from("Web assembly is binary executable format")),
                Slide::Text(String::from("Eduard Knyshov @edvorg")),
                Slide::Code(String::from("Hello World in Rust"), String::from("fn main() {\n  println!(\"Hello World\");\n}")),
            )
        }
    );
}
