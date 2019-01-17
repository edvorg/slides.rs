# slides.rs

Slides engine in rust

[Documentation](https://docs.rs/crate/slides)

[Crate](https://crates.io/crates/slides)

[Demo](https://edvorg.github.io/slides.rs)

## Getting started

Create an empty WASM project:

```shell
cargo new --bin example
```

Add slides dependency to Cargo.toml:

```toml
slides = "<latest version from crates.io>"
```

Add [index.html](https://raw.githubusercontent.com/edvorg/slides.rs/master/frontend/static/index.html) and [index.css](https://raw.githubusercontent.com/edvorg/slides.rs/master/frontend/static/index.css) to **./static** directory.

Run engine from main.rs:

```rust
use slides::run;
use slides::Story;
use slides::Slide;

fn main() {
    run(
        Story {
            slides: vec!(
                Slide::text("Hello World"),
            )
        }
    );
}
```

Start project with

```shell
cargo web start --auto-reload
```

## Example talks

[Full-Stack Web Development in Rust](https://edvorg.github.io/slides.rs/talk-1)
