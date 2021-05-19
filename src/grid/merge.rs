use crate::utils;

pub fn get_merged_grid_labels(grid_labels: &mut utils::GridLabel, scores: &[u32], grid: &utils::Grid, threshold: u32) {
    for i in 0..grid_labels.labels.len(){
        if !grid_labels.contrours[i]  {
            continue
        }
        // ○○○●○●○○○
        // ●●●○○○●●●
        // ○○○●○●○○○
        // ●部分を探索
        let group_id = grid_labels.labels[i];
        if group_id == 1 {
            continue
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
