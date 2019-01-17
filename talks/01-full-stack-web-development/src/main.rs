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

static COUNT_TO_MAX: u32 = 1000000000;
static SUBSTRING_TIMES: u32 = 100000000;
static SUBSTRING_DATA: &str = "foobarbazqux";
static SORT_TIMES: u32 = 10000000;

fn perf() -> f64 {
    js! (
      return performance.now();
    ).try_into().unwrap()
}

fn count_js(max: u32) {
    js! {
      window.count_js(@{max});
    }
}

fn count_rust(max: u32) -> u32 {
    let mut res = 0u32;
    for _i in 0..max {
        res = res + 1;
    };
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
                Slide::list("Rust targets (70+ as of today)", &[
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
                Slide::image_with_title("wasm-support.png", "Support", "WebAssembly VM is built into all modern web browsers. Older browsers can still run wasm converted to asm.js"),
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
                Slide::list("WebAssembly format", &[
                    "Binary executable (WASM)",
                    "Human readable representation (WAT)"
                ]),
                Slide::list("Essential tools", &[
                    "WABT: WebAssembly binary toolkit (wasm2wat, wat2wasm, wasm-interp)",
                    "Binaryen (wasm-opt, asm2wasm, wasm2js, wasm-sheel)",
                ]),
                Slide::image_with_title("wasm-wat.png", "What does WAT module look like?", "Human readable representation of module exporting function that's summing it's arguments"),
                Slide::image_with_title("wat2wasm.png", "Converting WAT to WASM", "WAT representation can be converted to executable WASM module"),
                Slide::image_with_title("wasm-loading.png", "Loading WASM module", "Wasm module can be loaded and executed in just a few lines of JavaScript"),
                Slide::image_with_title("wasm-imports.png", "Importing JavaScript functions", "Executing JavaScript functions is also supported"),
                Slide::image_with_title("wasm-memory.png", "Wasm module memory access", "JavaScript environment has easy access to WASM module memory"),
                Slide::image_with_title("wasm2wat.png", "WABT: WebAssembly Binary Toolkit", "wasm2wat and wat2wasm run in browser"),
                Slide::list("Supported parameter types", &[
                    "i32: 32-bit integer",
                    "i64: 64-bit integer",
                    "f32: 32-bit float",
                    "f64: 64-bit float",
                ]),
                Slide::list("Rust on WASM", &[
                    "Use WebAssembly imports/exports",
                    "wasm-bindgen and js-sys/web-sys",
                    "stdweb",
                ]),
                Slide::custom(
                    "Benchmark Rust/WASM (Counting)",
                    &|| CustomData::Unit,
                    &|state, message, _| {
                        match message {
                            CustomData::StringRef("rust") => {
                                let started_at = perf();
                                count_rust(COUNT_TO_MAX);
                                let finished_at = perf();
                                *state = CustomData::Number((finished_at - started_at) as u64);
                                true
                            }
                            CustomData::StringRef("js") => {
                                let started_at = perf();
                                count_js(COUNT_TO_MAX);
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
                              <span> { format!("Count to {} in Rust ", COUNT_TO_MAX) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("rust")) ,> { "Run" } </button>
                            </div>
                            </p>
                            <p>
                            <div>
                              <span> { format!("Count to {} in JavaScript ", COUNT_TO_MAX) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("js")) ,> { "Run" } </button>
                            </div>
                            </p>
                            { result_view() }
                          </div>
                        }
                    }
                ),
                Slide::custom(
                    "Benchmark Rust/WASM (Substring)",
                    &|| CustomData::Unit,
                    &|state, message, _| {
                        match message {
                            CustomData::StringRef("rust") => {
                                let started_at = perf();
                                substring_rust(SUBSTRING_TIMES);
                                let finished_at = perf();
                                *state = CustomData::Number((finished_at - started_at) as u64);
                                true
                            }
                            CustomData::StringRef("js") => {
                                let started_at = perf();
                                substring_js(SUBSTRING_TIMES);
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
                              <span> { format!("Finding substring, {} times in Rust ", SUBSTRING_TIMES) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("rust")) ,> { "Run" } </button>
                            </div>
                            </p>
                            <p>
                            <div>
                              <span> { format!("Finding substring, {} times in JavaScript ", SUBSTRING_TIMES) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("js")) ,> { "Run" } </button>
                            </div>
                            </p>
                            { result_view() }
                          </div>
                        }
                    }
                ),
                Slide::custom(
                    "Benchmark Rust/WASM (Bubble Sort ;))",
                    &|| CustomData::Unit,
                    &|state, message, _| {
                        match message {
                            CustomData::StringRef("rust") => {
                                let started_at = perf();
                                sort_rust(SORT_TIMES);
                                let finished_at = perf();
                                *state = CustomData::Number((finished_at - started_at) as u64);
                                true
                            }
                            CustomData::StringRef("js") => {
                                let started_at = perf();
                                sort_js(SORT_TIMES);
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
                              <span> { format!("Sorting an array, {} times in Rust ", SORT_TIMES) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("rust")) ,> { "Run" } </button>
                            </div>
                            </p>
                            <p>
                            <div>
                              <span> { format!("Sorting an array, {} times in JavaScript ", SORT_TIMES) } </span>
                              <button onclick=|_| RootMessage::Custom(CustomData::StringRef("js")) ,> { "Run" } </button>
                            </div>
                            </p>
                            { result_view() }
                          </div>
                        }
                    }
                ),
                Slide::list("wasm-bindgen", &[
                    "Library - includes utility macros",
                    "CLI optimization tool wasm-bindgen-cli",
                ]),
                Slide::image_with_title("bindgen-exports.png", "Bindgen", "Exporting/importing Rust function to/from JavaScript"),
                Slide::image_with_title("bindgen-calling.png", "Bindgen", "Calling exported function from JavaScript"),
                Slide::image_with_title("wasm-project.png", "Hello world in Rust/WASM", "Project setup"),
                Slide::image_with_title("wasm-step-0.png", "Hello world in Rust/WASM", "Counting benchmark code"),
                Slide::image_with_title("wasm-step-1.png", "Hello world in Rust/WASM", "Release build includes big amount of \"dead\" code. (1.3mb binary)"),
                Slide::image_with_title("wasm-step-2.png", "Hello world in Rust/WASM", "wasm-bindgen strips away unused code. (871b binary)"),
                Slide::image_with_title("wasm-step-3.png", "Hello world in Rust/WASM", "wasm-opt allows to get even smaller binary (817b binary)"),
                Slide::image_with_title("wasm-step-4.png", "Hello world in Rust/WASM", "WASM format is compact and not readable"),
                Slide::image_with_title("wasm-pre-final.png", "Hello world in Rust/WASM", "Final code is well optimized but includes data of 1kb memory page filled with zeroes"),
                Slide::image_with_title("wasm-final.png", "Hello world in Rust/WASM", "Cutting out unnecessary data allows to reduce final binary down to 58 bytes"),
                Slide::list("Gotchas", &[
                    "Pure algorithms are faster in Rust",
                    "Allocations are slower in Rust",
                    "Code is well optimized by Rust compiler",
                    "Passing data between Rust and JS is expensive",
                    "JS can access data in WASM module without interop",
                    "WASM binaries are huge if not dealt with",
                    "Use wasm-bindgen CLI tool to get smaller binaries",
                    "Use wasm-opt from projet binaryen to optimize wasm binary",
                    "Use wasm2wat from project wabt to inspect final wasm code",
                ]),
                Slide::list("Ideal application for Rust/WASM", &[
                    "CPU intensive computations in browser",
                    "Web games",
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
                    "JSX-like templates for view code",
                    "Agents - actors model based on web workers",
                    "Components - defining custom components for html",
                    "Virtual dom - re-rendering only state that changed",
                    "Services - injecting dependencies into lifecycle hooks",
                ]),
                Slide::image_with_title("structure.png", "Project structure", "Cargo workspaces allow to keep code organized and share common parts"),
                Slide::image_with_title("yew-project.png", "Dependencies", "Minimal frontend project needs yew and stdweb. Optionally serde for json serialization."),
                Slide::image_with_title("yew-context.png", "Dependency injection", "Context is injected in lifecycle hooks of every component"),
                Slide::image_with_title("yew-entry.png", "Minimal frontend project", "Minimal yew project includes event loop and mounting code for root component."),
                Slide::image_with_title("yew-component.png", "Yew component lifecycle", "Every component has to implement Component trait"),
                Slide::image_with_title("yew-renderable.png", "Make Yew component renderable", "Every component has to implement Renderable trait"),
                Slide::image_with_title("yew-component-props.png", "Yew component properties", "Properties allow passing data from parent to child"),
                Slide::image_with_title("yew-component-embedding.png", "Composing yew components", "All component properties are type-checked at compile time"),
                Slide::image_with_title("cargo-web-dev.png", "Running project in dev mode", "'cargo web start --auto-reload' allows quick compile-run-test loop due to incremental comilation"),
                Slide::image_with_title("cargo-web-release.png", "Building project for release mode", "'cargo web deploy' compiles release version and copies all assets to target/deploy"),
                Slide::image_with_title("conditional-compilation.png", "Conditional compilation", "'target_arch' configuration allows to conditionally compile code based on target"),
                Slide::image_with_title("bindings-interface.png", "Rust bindings for JS code", "Define an interface and common functionality"),
                Slide::image_with_title("bindings-impl.png", "Rust bindings for JS code", "Provide am implementation"),
                Slide::image_with_title("porting-snippet.png", "Porting JS code to rust", "Porting JavaScript code to Rust is surprisingly easy. Move code to js! block first"),
                Slide::image_with_title("porting-impl.png", "Porting js code to rust", "Unwrap js! block and fix compilation errors"),
                // show how to define rocket handlers
                Slide::title("Demo"), // Demo https://github.com/edvorg/rustmith
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
