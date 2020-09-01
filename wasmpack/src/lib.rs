mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Counter {
    tick: usize,
}

#[wasm_bindgen]
impl Counter {
    pub fn new() -> Counter {
        Counter { tick: 0 }
    }

    pub fn add(&mut self) {
        self.tick += 1;
    }

    pub fn count(&self) -> usize {
        self.tick
    }
}
