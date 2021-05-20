use crate::utils;

// TODO: この関数なんとかする
pub fn get_merged_grid_labels(grid_labels: &mut utils::GridLabel, scores: &[u32], grid: &utils::Grid, threshold: u32) {
    for i in 0..grid_labels.labels.len(){
        if !grid_labels.contrours[i]  {
            continue
        }
        // ○○○○●○○○○
        // ○○○●○●○○○
        // ●●●○○○●●●
        // ○○○●○●○○○
        // ○○○○●○○○○
        // ●部分を探索
        let group_id = grid_labels.labels[i];
        if group_id == 1 {
            continue
        }
        if i > i - grid.cols * 2 {
            update_grid_labels(grid_labels, grid, scores, i, i - grid.cols * 2, threshold, &vec![i - grid.cols]);
        }
        if i >= grid.cols + 1 {
            update_grid_labels(grid_labels, grid, scores, i, i - grid.cols - 1, threshold, &vec![i - 1, i - grid.cols]);
        }
        if i >= grid.cols - 1 && i >= grid.cols {
            update_grid_labels(grid_labels, grid, scores, i, i - grid.cols + 1, threshold, &vec![i + 1, i - grid.cols]);
        }
        if i >= 4 {
            update_grid_labels(grid_labels, grid, scores, i, i - 4, threshold, &vec![i - 1, i - 2, i - 3]);
        }
        if i >= 3 {
            update_grid_labels(grid_labels, grid, scores, i, i - 3, threshold, &vec![i - 1, i - 2]);
        }
        if i >= 2 {
            update_grid_labels(grid_labels, grid, scores, i, i - 2, threshold, &vec![i - 1]);
        }
        update_grid_labels(grid_labels, grid, scores, i, i + 2, threshold, &vec![i + 1]);
        update_grid_labels(grid_labels, grid, scores, i, i + 3, threshold, &vec![i + 1, i + 2]);
        update_grid_labels(grid_labels, grid, scores, i, i + 4, threshold, &vec![i + 1, i + 2, i + 3]);
        if i >= 1 {
            update_grid_labels(grid_labels, grid, scores, i, i + grid.cols - 1, threshold, &vec![i - 1, i + grid.cols]);
        }
        update_grid_labels(grid_labels, grid, scores, i, i + grid.cols + 1, threshold, &vec![i + 1, i + grid.cols]);
        update_grid_labels(grid_labels, grid, scores, i, i + grid.cols * 2, threshold, &vec![i + grid.cols]);
    }
}

fn is_exist_expected_index(grid: &utils::Grid, to_index: usize, from_index: usize) -> bool {
    let expected_row_diff = if to_index > from_index {
        (to_index - from_index) / grid.cols
    } else {
        (from_index - to_index) / grid.cols
    };
    let actual_row_diff = if to_index > from_index {
        to_index / grid.cols - from_index / grid.cols
    } else {
        from_index / grid.cols - to_index / grid.cols
    };
    actual_row_diff == expected_row_diff
}
  
fn update_grid_labels(grid_labels: &mut utils::GridLabel, grid: &utils::Grid, scores: &[u32], index: usize, before_index: usize, threshold: u32, buried_indexes: &[usize]) {
    if before_index >= grid_labels.labels.len() {
        return
    }
    if !is_exist_expected_index(grid, before_index, index) {
        return
    }
    let group_id = grid_labels.labels[index];
    let before_group_id = grid_labels.labels[before_index];
    if before_group_id == 1 || before_group_id == group_id {
        return
    }
    let mut is_over = false;
    for i in 0..(buried_indexes.len()) {
        // TODO: ここの閾値処理は多分もっといい感じにできる
        // TODO: とりあえずどこかが閾値を超えてたらfill_missing_gridで埋まるようにしてるけど要検討
        if scores[buried_indexes[i]] > threshold {
            is_over = true;
            grid_labels.labels[buried_indexes[i]] = group_id;
            grid_labels.contrours[buried_indexes[i]] = true;
        }
    }
    // TODO: ここ一々埋め立ててるけど、マージするgroup_idをもっておいてあとで一気に埋めたほうがよさそう
    if is_over {
        for j in 0..grid_labels.labels.len() {
            if grid_labels.labels[j] == before_group_id {
                grid_labels.labels[j] = group_id;
            }
        }
    }
}

pub fn fill_missing_grid(labels: &mut [u32], grid: &utils::Grid) {
    const MAX_DIFF: usize = 3;
    for row in 0..(grid.rows) {
        let mut last_filled_col = grid.cols;
        let mut last_filled_group = 0;
        for col in 0..(grid.cols) {

            if labels[row * grid.cols + col] != 1 {

                if  col > last_filled_col + 1 && col <= MAX_DIFF + last_filled_col + 1 && last_filled_group == labels[row * grid.cols + col] {

                    for i in (row * grid.cols + last_filled_col + 1)..(row * grid.cols + col) {
                        labels[i] = last_filled_group;
                    }
                }
                last_filled_group = labels[row * grid.cols + col];
                last_filled_col = col;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn not_exist_expected_index() {
        let grid = utils::Grid{ rows: 3, cols: 6 };
        let is_exist = is_exist_expected_index(&grid, 6, 5);
        assert_eq!(is_exist, false);
        let is_exist = is_exist_expected_index(&grid, 5, 6);
        assert_eq!(is_exist, false);
        let is_exist = is_exist_expected_index(&grid, 5, 12);
        assert_eq!(is_exist, false);
    }
    #[test]
    fn exist_expected_index() {
        let grid = utils::Grid{ rows: 3, cols: 6 };
        let is_exist = is_exist_expected_index(&grid, 6, 7);
        assert_eq!(is_exist, true);
        let is_exist = is_exist_expected_index(&grid, 7, 6);
        assert_eq!(is_exist, true);
        let is_exist = is_exist_expected_index(&grid, 6, 13);
        assert_eq!(is_exist, true);
    }
    #[test]
    fn not_update_gls_when_same_label() {
        let mut grid_labels = utils::GridLabel{ labels: vec![1, 3, 1, 1, 3, 3], contrours: vec![false, true, false, false, true, true] };
        let scores = vec![1, 1, 1, 1, 1, 1];
        let threshold = 0;
        let index = 5;
        let group_id = grid_labels.labels[index];
        let buried_indexes = vec![2, 3];
        let grid = utils::Grid{ rows: 1, cols: 6 };
        update_grid_labels(&mut grid_labels, &grid, &scores, index, 1, threshold, &buried_indexes);
        for i in buried_indexes {
            assert_ne!(grid_labels.labels[i], group_id);
        }
    }
    #[test]
    fn not_update_gls_when_all_do_not_over() {
        let mut grid_labels = utils::GridLabel{ labels: vec![1, 2, 1, 1, 3, 3], contrours: vec![false, true, false, false, true, true] };
        let scores = vec![1, 4, 1, 1, 4, 4];
        let threshold = 2;
        let index = 5;
        let group_id = grid_labels.labels[index];
        let buried_indexes = vec![2, 3];
        let grid = utils::Grid{ rows: 1, cols: 6 };
        update_grid_labels(&mut grid_labels, &grid, &scores, index, 1, threshold, &buried_indexes);
        for i in buried_indexes {
            assert_ne!(grid_labels.labels[i], group_id);
        }
    }
    #[test]
    fn check_update_gls() {
        let mut grid_labels = utils::GridLabel{ labels: vec![1, 2, 1, 1, 3, 3], contrours: vec![false, true, false, false, true, true] };
        let scores = vec![1, 4, 1, 1, 4, 4];
        let threshold = 0;
        let index = 5;
        let group_id = grid_labels.labels[index];
        let buried_indexes = vec![2, 3];
        let grid = utils::Grid{ rows: 1, cols: 6 };
        update_grid_labels(&mut grid_labels, &grid, &scores, index, 1, threshold, &buried_indexes);
        for i in buried_indexes {
            assert_eq!(grid_labels.labels[i], group_id);
            assert_eq!(grid_labels.contrours[i], true);
        }
    }
    #[test]
    fn check_update_gls_and_fill_same_label() {
        let mut grid_labels = utils::GridLabel{ labels: vec![
                1, 2, 1, 1, 3, 3,
                1, 2, 2, 1, 1, 1,
            ], contrours: vec![
                false, true, false, false,  true,  true,
                false, true,  true, false, false, false,
            ] };
        let scores = vec![
            1, 4, 1, 1, 4, 4,
            1, 4, 4, 1, 1, 1,
        ];
        let threshold = 0;
        let index = 5;
        let buried_indexes = vec![2, 3];
        let grid = utils::Grid{ rows: 2, cols: 6 };
        update_grid_labels(&mut grid_labels, &grid, &scores, index, 1, threshold, &buried_indexes);
        let expected_labels = vec![
            1, 3, 3, 3, 3, 3,
            1, 3, 3, 1, 1, 1,
        ];
        for i in 0..expected_labels.len() {
            assert_eq!(grid_labels.labels[i], expected_labels[i]);
        }
    }

    #[test]
    fn check_get_merged_gls() {
        let mut grid_labels = utils::GridLabel{ labels: vec![
                1, 2, 1, 1, 3, 1,
                1, 2, 2, 1, 1, 4,
                1, 1, 1, 1, 1, 4,
                1, 5, 5, 1, 1, 1,
            ], contrours: vec![
                false,  true, false, false,  true, false,
                false,  true,  true, false, false,  true,
                false, false, false, false, false,  true,
                false,  true,  true, false, false, false,
            ] };
        let scores = vec![
            1, 4, 1, 1, 4, 1,
            1, 4, 4, 1, 1, 4,
            1, 1, 1, 1, 1, 4,
            1, 4, 4, 1, 1, 1,
        ];
        let threshold = 0;
        let grid = utils::Grid{ rows: 4, cols: 6 };
        get_merged_grid_labels(&mut grid_labels, &scores, &grid, threshold);
        let expected_labels = vec![
            1, 2, 2, 2, 2, 2,
            1, 2, 2, 1, 2, 2,
            1, 1, 1, 1, 1, 2,
            1, 5, 5, 1, 1, 1,
        ];
        for i in 0..expected_labels.len() {
            assert_eq!(grid_labels.labels[i], expected_labels[i], "index: {}, actual: {:?}, expected: {:?}", i, grid_labels.labels, expected_labels);
        }
    }

    #[test]
    fn not_fill_missing_grid() {
        let mut labels = vec![3, 1, 1, 1, 1, 3];
        let expected_labels = labels.clone();
        let grid = utils::Grid{ rows: 1, cols: 6 };
        fill_missing_grid(&mut labels, &grid);
        for i in 0..expected_labels.len() {
            assert_eq!(labels[i], expected_labels[i], "index: {}, actual: {:?}, expected: {:?}", i, labels, expected_labels);
        }
    }
    #[test]
    fn not_fill_missing_grid_when_new_line() {
        let mut labels = vec![
            1, 1, 1, 1, 1, 3,
            1, 1, 3, 1, 1, 1,
        ];
        let expected_labels = labels.clone();
        let grid = utils::Grid{ rows: 2, cols: 6 };
        fill_missing_grid(&mut labels, &grid);
        for i in 0..expected_labels.len() {
            assert_eq!(labels[i], expected_labels[i], "index: {}, actual: {:?}, expected: {:?}", i, labels, expected_labels);
        }
    }
    #[test]
    fn check_fill_missing_grid() {
        let mut labels = vec![3, 1, 1, 1, 3, 1];
        let grid = utils::Grid{ rows: 1, cols: 6 };
        fill_missing_grid(&mut labels, &grid);
        let expected_labels = vec![3, 3, 3, 3, 3, 1];
        for i in 0..expected_labels.len() {
            assert_eq!(labels[i], expected_labels[i], "index: {}, actual: {:?}, expected: {:?}", i, labels, expected_labels);
        }
    }
}
