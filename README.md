# slides.rs

Slides engine in rust

[Documentation](https://docs.rs/crate/slides)

[Crate](https://crates.io/crates/slides)

## Getting started

Create an empty WASM project

```shell
cargo new --bin example
```

Add slides dependency to Cargo.toml

```toml
slides = "0.1.2"
```

Add [index.html](https://raw.githubusercontent.com/edvorg/slides.rs/master/frontend/static/index.html) and [index.css](https://raw.githubusercontent.com/edvorg/slides.rs/master/frontend/static/index.css) to **./static** directory.

Start project with

```shell
cargo web start --auto-reload
```
