use crate::utils::Grid;

pub fn get_grid_scores(binary: &(u32, u32, Vec<bool>), grid: &Grid, side: usize) -> Vec<u32> {
    let mut result = Vec::with_capacity(grid.rows * grid.cols);
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let mut score = 0;
            for k in (row * side)..(std::cmp::min((row + 1) * side, binary.1 as usize)) {
                for l in (col * side)..(std::cmp::min((col + 1) * side, binary.0 as usize)) {
                    if binary.2[k * (binary.0 as usize) + l] {
                        score += 1;
                    }
                }
            }
            result.push(score);
        }
    }
    return result
}
