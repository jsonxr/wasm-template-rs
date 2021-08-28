extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
//use web_sys::console;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// // This is like the `main` function, except for JavaScript.
// #[wasm_bindgen(start)]
// pub fn main_js() -> Result<(), JsValue> {
//   // This provides better error messages in debug mode.
//   // It's disabled in release mode so it doesn't bloat up the file size.
//   #[cfg(debug_assertions)]
//   console_error_panic_hook::set_once();

//   // Your code goes here!
//   console::log_1(&JsValue::from_str("Hello world from rust!"));
//   Ok(())
// }

#[wasm_bindgen]
pub fn sum(x: i32, y: i32) -> i32 {
  log("Howdy");
  return x + y;
}

#[wasm_bindgen]
pub fn say_hello_from_rust() {
  log("howdy");
}
