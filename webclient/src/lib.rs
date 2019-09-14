mod utils;

use wasm_bindgen::prelude::*;
use ferris_says::say;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn consoleLog(s: &str);
}

#[wasm_bindgen]
pub fn yeet() {
    let out = b"Crabs everywhere!";
    let width = out.len();
    let mut buf = Vec::new();
    say(out, width, &mut buf).unwrap();
    let text = String::from_utf8(buf).unwrap();
    consoleLog(&text);
}
