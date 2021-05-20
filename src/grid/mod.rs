mod score;
mod label;
mod merge;

use crate::utils;

pub fn get_labels(width: u32, height: u32, binary: &[bool], side: u32) -> Vec<u32> {
    let grid = utils::Grid{ rows: (height as f32 / side as f32).ceil() as usize, cols: (width as f32 / side as f32).ceil() as usize };
    let scores = score::get_grid_scores(&(width, height, binary), &grid, side as usize);
    let mut grid_labels = label::get_labels_by_scores(&scores, &grid, side * side / 10);
    merge::get_merged_grid_labels(&mut grid_labels, &scores, &grid, 0);
    merge::fill_missing_grid(&mut grid_labels.labels, &grid);
    grid_labels.labels
}
