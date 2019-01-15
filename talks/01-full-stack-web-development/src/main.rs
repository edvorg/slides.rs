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
static COUNT_TO_TIMES_JS: u32 = 10000;
static COUNT_TO_TIMES_RUST: u32 = 10000;
static SUBSTRING_TIMES_RUST: u32 = 100000000;
static SUBSTRING_TIMES_JS: u32 = 100000000;
static SUBSTRING_DATA: &str = "foobarbazqux";
static SORT_TIMES_RUST: u32 = 10000000;
static SORT_TIMES_JS: u32 = 10000000;

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

fn count_rust(max: u32, times: u32) -> u64 {
    let mut res = 0u64;
    for _j in 0..times {
        for _i in 0..max {
            res = res + 1;
        };
    }
    let s = format!("{}", res);
    js! { console.log("rust:", @{s}); };
    res
}

fn substring_js(times: u32) {
    js! {
      window.substring_js(@{times});
    }
}

fn substring_rust(times: u32) {
    for _i in 0..times {
        SUBSTRING_DATA.contains("baz");
    };
    js! { console.log("rust: done"); }
}

fn sort_js(times: u32) {
    js! {
      window.sort_js(@{times});
    }
}

fn sort_rust(times: u32) {
    let mut array = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    for _k in 0..times {
        for i in 0..array.len() {
            for j in i..array.len() {
                let left = array[i];
                let right = array[j];
                if right < left {
                    array[i] = right;
                    array[j] = left;
                }
            }
        }
    };
}

fn main() {
    run(
        Story {
            slides: vec!(
                Slide::title("Full-Stack Web Development in Rust"),
                Slide::list("Background", &[
                    "3 years of game development in C/C++",
                    "3 years of full-stack development in Java/Scala/Clojure",
                    "Currently Scala/Clojure by day and Rust by night",
                ]),
                Slide::image("rust-logo.png", "Rust is a systems programming language with a focus on memory safety and memory management"),
                Slide::list("Rust strengths", &[
                    "Static type system",
                    "Type-safe",
                    "Zero-Cost abstractions",
                    "Memory management",
                    "Native",
                    "Cross-platform",
                ]),
                Slide::list("Rust targets (70+)", &[
                    "Linux",
                    "OSX",
                    "iOS",
                    "Android",
                    "Fuchsia",
                    "Raspberry",
                    "Arduino (avr-rust)",
                    "WebAssembly",
                ]),
                Slide::image("wasm-logo.png", "WebAssembly (abbreviated WASM) is a binary instruction format for a stack-based virtual machine."),
                Slide::image_with_title("wasm-support.png", "Support", "WebAssembly VM is built into all modern web browsers"),
                Slide::list("WebAssembly strength", &[
                    "Low-level",
                    "Efficient and fast",
                    "Safe",
                    "Portable",
                ]),
                Slide::list("Alternative targets", &[
                    "Wasmer - standalone WASM runtime in Rust",
                    "WasmJIT - runs WASM in linux kernel space",
                    "Nebulet - runs WASM at ring 0",
                    "awesome-wasm-runtimes - 20+ wasm runtimes in different languages on github",
                ]),
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
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("rust")) ,> { "Run" } </button>
                            </div>
                            </p>
                            <p>
                            <div>
                              <span> { format!("Count to {}, {} times in JavaScript ", COUNT_TO_MAX, COUNT_TO_TIMES_JS) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("js")) ,> { "Run" } </button>
                            </div>
                            </p>
                            { result_view() }
                          </div>
                        }
                    }
                ),
                Slide::custom(
                    "Benchmark Rust (Substring)",
                    &|| CustomData::Unit,
                    &|state, message, _| {
                        match message {
                            CustomData::StringRef("rust") => {
                                let started_at = perf();
                                substring_rust(SUBSTRING_TIMES_RUST);
                                let finished_at = perf();
                                *state = CustomData::Number((finished_at - started_at) as u64);
                                true
                            }
                            CustomData::StringRef("js") => {
                                let started_at = perf();
                                substring_js(SUBSTRING_TIMES_JS);
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
                              <span> { format!("Finding substring, {} times in Rust ", SUBSTRING_TIMES_RUST) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("rust")) ,> { "Run" } </button>
                            </div>
                            </p>
                            <p>
                            <div>
                              <span> { format!("Finding substring, {} times in JavaScript ", SUBSTRING_TIMES_JS) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("js")) ,> { "Run" } </button>
                            </div>
                            </p>
                            { result_view() }
                          </div>
                        }
                    }
                ),
                Slide::custom(
                    "Benchmark Rust (Bubble Sort ;))",
                    &|| CustomData::Unit,
                    &|state, message, _| {
                        match message {
                            CustomData::StringRef("rust") => {
                                let started_at = perf();
                                sort_rust(SORT_TIMES_RUST);
                                let finished_at = perf();
                                *state = CustomData::Number((finished_at - started_at) as u64);
                                true
                            }
                            CustomData::StringRef("js") => {
                                let started_at = perf();
                                sort_js(SORT_TIMES_JS);
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
                              <span> { format!("Sorting an array, {} times in Rust ", SORT_TIMES_RUST) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("rust")) ,> { "Run" } </button>
                            </div>
                            </p>
                            <p>
                            <div>
                              <span> { format!("Sorting an array, {} times in JavaScript ", SORT_TIMES_JS) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("js")) ,> { "Run" } </button>
                            </div>
                            </p>
                            { result_view() }
                          </div>
                        }
                    }
                ),
                Slide::image_with_title("wasm-wat.png", "How does wasm look like?", "Human readable representation of function that's summing it's arguments"),
                Slide::image_with_title("wasm-loading.png", "Loading WASM module", "Wasm module can be loaded and executed in just a few lines of code"),
                Slide::image_with_title("wasm-loading.png", "Wasm module memory access", "JavaScript environment has easy access to WASM module memory"),
                Slide::list("Rust on WASM", &[
                    "Use WebAssembly imports/exports",
                    "wasm-bindgen and js-sys/web-sys",
                    "stdweb",
                ]),
                Slide::list("wasm-bindgen", &[
                    "Macro library",
                    "CLI optimization tool",
                ]),
                Slide::image_with_title("bindgen-exports.png", "Bindgen", "Exporting/importing Rust function to/from JavaScript"),
                // demonstrate some of the features of js-sys and web-sys
                Slide::image_with_title("bindgen-calling.png", "Bindgen", "Calling exported function from JavaScript"),
                Slide::image_with_title("wasm-project.png", "Hello world in Rust/WASM", "Project setup"),
                Slide::image_with_title("wasm-step-0.png", "Hello world in Rust/WASM", "Counting benchmark code"),
                Slide::image_with_title("wasm-step-1.png", "Hello world in Rust/WASM", "Release build includes big amount of \"dead\" code. (1.3mb binary)"),
                Slide::image_with_title("wasm-step-2.png", "Hello world in Rust/WASM", "wasm-bindgen strips away unused code. (871b binary)"),
                Slide::image_with_title("wasm-step-3.png", "Hello world in Rust/WASM", "wasm-opt allows to get even smaller binary (817b binary)"),
                Slide::image_with_title("wasm-step-4.png", "Hello world in Rust/WASM", "WASM format is compact and not readable"),
                Slide::image_with_title("wasm-step-5.png", "Hello world in Rust/WASM", "WASM can be converted to human readable WAT using wasm2wat"),
                Slide::image_with_title("wasm2wat.png", "Hello world in Rust/WASM", "wasm2wat runs in browser"),
                Slide::list("Gotchas", &[
                    "Pure algorithms are faster in Rust",
                    "Allocations a slower in Rust",
                    "Code is well optimized by compiler",
                    "Passing data between Rust and JS is expensive",
                    "JS can access data in WASM module without interop",
                    "WASM binaries are huge if not dealt with",
                    "Use wasm-bindgen CLI tool to get smaller binaries",
                    "Use wasm-opt from binaryen to optimize wasm binary",
                    "Use wasm2wat from wabt to inspect final wasm code",
                ]),
                Slide::list("Ideal application for Rust/WASM", &[
                    "CPU intensive computations in browser",
                    "Web games",
                    "Ports of existing rust software to browser",
                    "Small optimizations in Rust in existing JavaScript",
                ]),
                Slide::list("What would 100% Rust web app look like?", &[
                    "Effortless interop with JS",
                    "Rust frontend framework",
                    "Rust backend framework",
                    "Code sharing between frontend and backend",
                    "Simple workflow",
                ]),
                Slide::image_with_title("stdweb-exports.png", "Stdweb", "Exporting Rust function to JavaScript"),
                Slide::image_with_title("stdweb-calling.png", "Stdweb", "Calling exported function from JavaScript"),
                Slide::image_with_title("stdweb-interop-1.png", "Stdweb", "Embed JavaScript code using js! macro"),
                Slide::image_with_title("stdweb-interop-2.png", "Stdweb", "Capture closures from Rust code"),
                Slide::image_with_title("stdweb-serialization.png", "Stdweb", "Serialization using Serde derivations"),
                Slide::list("Frontend frameworks", &[
                    "JS frameworks + hot code in Rust using wasm-bindgen",
                    "Yew - react inspired framework for Rust based on stdweb",
                    "Seed - similar framework, but based on wasm-bindgen",
                    "Ruukh - yet another framework based on wasm-bindgen"
                ]),
                Slide::list("Yew", &[
                    "JSX like templates for view code",
                    "Agents - actors model based on web workers",
                    "Components - defining custom components for html",
                    "Virtual dom - re-rendering only state that changed",
                    "Services - injecting dependencies into lifecycle hook",
                ]),
                // show how to define rocket handlers
                Slide::image_with_title("structure.png", "Project structure", "Cargo workspaces allow to keep code organized and share common parts"),
                Slide::image_with_title("yew-project.png", "Dependencies", "Minimal project needs yew and stdweb. Optionally serde for json serialization."),
                Slide::image_with_title("yew-context.png", "Dependency injection", "Context is injected in every lifecycle hook"),
                Slide::image_with_title("yew-entry.png", "Minimal frontend project", "Minimal yew project includes event loop and mountin code for root component."),
                Slide::image_with_title("yew-component-message.png", "Yew component", "Every component has to implement Component trait"),
                Slide::image_with_title("yew-component.png", "Yew component lifecycle", "Every component has to implement Component trait"),
                Slide::image_with_title("yew-renderable.png", "Make Yew component renderable", "Every component has to implement Renderable trait"),
                Slide::image_with_title("yew-component-props.png", "Yew component properties", "Properties allow passing data from parent to child"),
                Slide::image_with_title("yew-component-embedding.png", "Composing yew components", "All component properties are type-checked at compile time"),
                Slide::image_with_title("cargo-web-dev.png", "Running project in dev mode", "'cargo web start --auto-reload' allows quick compile-run-test loop due to incremental comilation"),
                Slide::image_with_title("cargo-web-release.png", "Building project for release mode", "'cargo web deploy' compiles release version and copies all assets to target/deploy"),
                Slide::image_with_title("conditional-compilation.png", "Conditional compilation", "'target_arch' configuration allows to conditionally compile code based on target"),
                Slide::image_with_title("todo.png", "Rust bindings for JS code", "Iteration 1"),
                Slide::image_with_title("todo.png", "Rust bindings for JS code", "Iteration 2"),
                Slide::image_with_title("todo.png", "Rust bindings for JS code", "Iteration 3"),
                Slide::image_with_title("todo.png", "Porting js code to rust", "Iteration 1"),
                Slide::image_with_title("todo.png", "Porting js code to rust", "Iteration 2"),
                Slide::image_with_title("todo.png", "Porting js code to rust", "Iteration 3"),
                Slide::title("Demo time"), // Demo https://github.com/edvorg/rustmith
                Slide::list("Pros", &[
                    "Close to zero runtime errors",
                    "Good IDE support (Tested on Intellij Idea)",
                    "Higher performance on computations",
                    "Reuse backend code in browser",
                    "Confidence when adding new features",
                ]),
                Slide::list("Cons", &[
                    "Performance penalty on data conversion",
                    "Higher learning curve",
                    "Lack of web libraries for rust",
                    "Code needs to be compiled",
                ]),
                Slide::image_with_title("links.png", "Links", "https://rustmith.rocks/links"),
            )
        }
    );
}
