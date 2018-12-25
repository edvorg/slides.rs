use slides::run;
use slides::Story;
use slides::Slide;

fn main() {
    run(
        Story {
            slides: vec!(
                Slide::Text(String::from("test 1")),
                Slide::Text(String::from("test 2")),
                Slide::Text(String::from("test 3")),
            )
        }
    );
}
