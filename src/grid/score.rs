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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_0_scores() {
        let binary = (5, 3, vec![
            false,false,false,false,false,
            false,false,false,false,false,
            false,false,false,false,false,
        ]);
        let side = 1;
        let grid = Grid{ rows: (binary.1 as f32 / side as f32).ceil() as usize, cols: (binary.0 as f32 / side as f32).ceil() as usize};
        let scores = get_grid_scores(&binary, &grid, side);
        for score in scores {
            assert_eq!(score, 0)
        }
    }

    #[test]
    fn all_1_scores() {
        let binary = (5, 3, vec![
            true,true,true,true,true,
            true,true,true,true,true,
            true,true,true,true,true,
        ]);
        let side = 1;
        let grid = Grid{ rows: (binary.1 as f32 / side as f32).ceil() as usize, cols: (binary.0 as f32 / side as f32).ceil() as usize};
        let scores = get_grid_scores(&binary, &grid, side);
        for score in scores {
            assert_eq!(score, 1)
        }
    }

    #[test]
    fn can_get_surplus_scores() {
        let binary = (5, 3, vec![
            true,   true,   true,   false,  true,
            true,   false,  false,  true,   false,
            false,  false,  true,   false,  true,
        ]);
        let side = 2;
        let grid = Grid{ rows: (binary.1 as f32 / side as f32).ceil() as usize, cols: (binary.0 as f32 / side as f32).ceil() as usize};
        let scores = get_grid_scores(&binary, &grid, side);
        assert_eq!(scores[0], 3);
        assert_eq!(scores[1], 2);
        assert_eq!(scores[2], 1);
        assert_eq!(scores[3], 0);
        assert_eq!(scores[4], 1);
        assert_eq!(scores[5], 1);
    }
}
