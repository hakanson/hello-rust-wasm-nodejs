mod utils;


use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// TODO: better console.log
// https://rustwasm.github.io/wasm-bindgen/examples/import-js.html

#[wasm_bindgen]
extern {
    fn console(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
  use web_sys::console;
  console::log_1(&JsValue::from_str(&format!("Hello, {}!", name)));
}
