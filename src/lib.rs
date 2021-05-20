extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
mod threshold;
mod divide;
mod utils;
mod filter;
mod grid;

#[wasm_bindgen]
pub fn get_labels_by_gray(gray: &[u8], width: u32, height: u32, side: u32) -> Box<[u32]> {
    let binary = crate::threshold::gray_to_binary(gray);
    let divided_binary = divide::get_divided_binary(&(width, height, &binary));
    let noise_filtered_binary = filter::get_filtered_binary(&divided_binary, width / 40);
    let labels = grid::get_labels(width, height, &noise_filtered_binary, side);
    labels.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test;
    #[test]
    fn can_get_labels_by_gray() {
        let (width, height, gray) = test::load_gray_image();
        let binary = threshold::gray_to_binary(&gray);
        let divided_binary = divide::get_divided_binary(&(width, height, &binary));
        let noise_filtered_binary = filter::get_filtered_binary(&divided_binary, width / 40);
        let side = width / 50;
        let labels = grid::get_labels(width, height, &noise_filtered_binary, side);
        let grid = utils::Grid{ rows: (height as f32 / side as f32).ceil() as usize, cols: (width as f32 / side as f32).ceil() as usize };
        test::get_visualized_labels(&labels, &(width, height, &noise_filtered_binary), side, &grid, &vec![0, 1]);
    }
}
