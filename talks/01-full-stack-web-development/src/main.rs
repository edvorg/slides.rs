#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

use slides::run;
use slides::Story;
use slides::Slide;
use slides::CustomData;
use slides::RootMessage;
use stdweb::unstable::TryInto;

static COUNT_TO_MAX: u32 = 1000000;
static COUNT_TO_TIMES_JS: u32 = 15000;
static COUNT_TO_TIMES_RUST: u32 = 15000;

fn perf() -> f64 {
    js! (
      return performance.now();
    ).try_into().unwrap()
}

fn count_js(max: u32, times: u32) {
    js! {
      window.count_js(@{max}, @{times});
    }
}

fn count_rust(max: u32, times: u32) {
    let mut res = 0u64;
    for _j in 0..times {
        for _i in 0..max {
            res = res + 1;
        };
    }
    let s = format!("{}", res);
    js! { console.log("rust:", @{s}); }
}

fn main() {
    run(
        Story {
            slides: vec!(
                Slide::title("Full-Stack Web Development in Rust"),
                Slide::list("Background", &[
                    "3 years of game development in C++",
                    "1 year of server-side development in Java/Scala",
                    "2 years of full-stack development in Clojure",
                ]),
                Slide::image("rust-logo.png", "Rust is a systems programming language with a focus on memory safety and memory management"),
                Slide::list("Rust strengths", &[
                    "Static type system",
                    "Type-safe",
                    "Memory management",
                    "Native",
                    "Cross-platform",
                ]),
                Slide::image("wasm-logo.png", "WebAssembly (abbreviated WASM) is a binary instruction format for a stack-based virtual machine."),
                Slide::list("WebAssembly strength", &[
                    "Low-level",
                    "Efficient and fast",
                    "Safe",
                    "Portable",
                ]),
                Slide::list("Rust on WASM", &[
                    "wasm-bindgen and js-sys/web-sys",
                    "stdweb",
                ]),
                Slide::image_with_title("bindgen-exports.png", "Bindgen", "Exporting Rust function to JavaScript"),
                Slide::image_with_title("bindgen-calling.png", "Bindgen", "Calling exported function from JavaScript"),
                Slide::image_with_title("stdweb-exports.png", "Stdweb", "Exporting Rust function to JavaScript"),
                Slide::image_with_title("stdweb-calling.png", "Stdweb", "Calling exported function from JavaScript"),
                Slide::image_with_title("stdweb-interop-1.png", "Stdweb", "Bi-directional interop with JavaScript"),
                Slide::image_with_title("stdweb-interop-2.png", "Stdweb", "Capture closures from Rust code"),
                Slide::image_with_title("stdweb-serialization.png", "Stdweb", "Serialization using Serde derivations"),
                Slide::list("Frontend frameworks", &[
                    "JS frameworks + hot code in Rust",
                    "Yew",
                ]),
                // How to implement components
                Slide::image_with_title("todo.png", "Rust bindings for JS code", "Iteration 1"),
                Slide::image_with_title("todo.png", "Rust bindings for JS code", "Iteration 2"),
                Slide::image_with_title("todo.png", "Rust bindings for JS code", "Iteration 3"),
                Slide::image_with_title("todo.png", "Porting js code to rust", "Iteration 1"),
                Slide::image_with_title("todo.png", "Porting js code to rust", "Iteration 2"),
                Slide::image_with_title("todo.png", "Porting js code to rust", "Iteration 3"),
                Slide::title("What does 100% Rust app looks like?"), // Demo https://github.com/edvorg/rustmith
                Slide::list("Pros", &[
                    "Close to zero runtime errors",
                    "Good IDE support (Tested on Intellij Idea)",
                    "Higher performance on computations",
                    "Reuse existing rust code in browser",
                    "Confidence when adding new features",
                ]),
                Slide::list("Cons", &[
                    "Performance penalty on data conversion",
                    "Higher learning curve",
                    "Lack of web libraries for rust",
                ]),
                Slide::list("Alternative targets", &[
                    "Wasmer - standalone WASM runtime in Rust",
                    "WasmJIT - runs WASM in linux kernel space",
                    "Nebulet - runs WASM at ring 0",
                    "awesome-wasm-runtimes - 20+ runtimes in different languages on github",
                ]),
                Slide::image_with_title("links.png", "Links", "https://rustmith.rocks/links"),
                Slide::custom(
                    "Benchmark Rust (Counting)",
                    &|| CustomData::Unit,
                    &|state, message, _| {
                        match message {
                            CustomData::StringRef("rust") => {
                                let started_at = perf();
                                count_rust(COUNT_TO_MAX, COUNT_TO_TIMES_RUST);
                                let finished_at = perf();
                                *state = CustomData::Number((finished_at - started_at) as u64);
                                true
                            }
                            CustomData::StringRef("js") => {
                                let started_at = perf();
                                count_js(COUNT_TO_MAX, COUNT_TO_TIMES_JS);
                                let finished_at = perf();
                                *state = CustomData::Number((finished_at - started_at) as u64);
                                true
                            }
                            _ => {
                                false
                            }
                        }
                    },
                    &|state| {
                        let result_view = || {
                            match state {
                                CustomData::Number(result) => {
                                    html! {
                                      <p>
                                        <span> { format!("Result: {}ms", result) } </span>
                                      </p>
                                    }
                                }
                                _ => {
                                    html! {
                                      <p>
                                        { "Run benchmark to see measurements" }
                                      </p>
                                    }
                                }
                            }
                        };
                        html! {
                          <div>
                            <p>
                            <div>
                              <span> { format!("Count to {}, {} times in Rust ", COUNT_TO_MAX, COUNT_TO_TIMES_RUST) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("rust")) ,> { "Benchmark" } </button>
                            </div>
                            </p>
                            <p>
                            <div>
                              <span> { format!("Count to {}, {} times in JavaScript ", COUNT_TO_MAX, COUNT_TO_TIMES_JS) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("js")) ,> { "Benchmark" } </button>
                            </div>
                            </p>
                            { result_view() }
                          </div>
                        }
                    }
                ),
            )
        }
    );
}
