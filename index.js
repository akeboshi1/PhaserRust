const js = import("./lib/Rust_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});