mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add_num(num_1: i32, num_2: i32) -> i32 {
    num_1 * num_2
}
