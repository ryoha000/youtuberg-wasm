extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
mod threshold;
mod divide;
mod utils;
mod filter;
mod grid;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

fn get_labels_by_gray(gray: &[u8], width: u32, height: u32) -> Vec<u32> {
    let binary = crate::threshold::gray_to_binary(gray);
    let divided_binary = divide::get_divided_binary(&(width, height, &binary));
    let noise_filtered_binary = filter::get_filtered_binary(&divided_binary, width / 40);
    let labels = grid::get_labels(width, height, &noise_filtered_binary);
    labels
}
