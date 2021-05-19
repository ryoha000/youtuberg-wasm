use super::super::utils;

// 返すlabelの0,1は管理用。0 => 未チェック, 1 => 閾値以下
pub fn get_labels_by_scores(scores: &[u32], grid: &utils::Grid, threshold: u32) -> utils::GridLabel {
    let mut group_id = 2;
    let size = utils::ImageSize{ width: grid.cols, height: grid.rows };
    let mut labels = vec![0; scores.len()];

    for i in 0..scores.len() {
        // もう、このマスに関係するGroupが探索されてたらスキップ
        if labels[i] != 0 {
            continue
        }
        labels[i] = 1;

        if scores[i] < threshold {
            continue
        }
    
        // 閾値以上の時
        labels[i] = group_id;
        let mut to_check_indexes = utils::get_around_indexes(i, &size);
    
        loop {
            if to_check_indexes.len() == 0 {
                break
            }
            let mut new_to_check_indexes = Vec::new();
            // 上下左右で閾値を超えているものがあるか探索
            for j in 0..(to_check_indexes.len()) {
                if labels[to_check_indexes[j]] != 0 {
                    continue
                }
                labels[to_check_indexes[j]] = 1;
        
                if scores[to_check_indexes[j]] < threshold {
                    continue
                }
                labels[to_check_indexes[j]] = group_id;
                new_to_check_indexes.append(&mut utils::get_around_indexes(to_check_indexes[j], &size));
            }
            to_check_indexes = new_to_check_indexes;
        }
        group_id += 1;
    }

    let contrours = utils::get_contrours_from_labels(&labels, &size);
    utils::GridLabel{ labels: labels, contrours: contrours }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_get_grid_labels() {
        let scores = vec![
             3, 10, 15,  6,  0,
            16, 12, 13, 10,  3,
            12, 16,  2,  5, 11,
        ];
        let grid = utils::Grid{ rows: 3, cols: 5 };
        let threshold = 10;
        let grid_labels = get_labels_by_scores(&scores, &grid, threshold);
        let expected_labels = vec![
            1, 2, 2, 1, 1,
            2, 2, 2, 2, 1,
            2, 2, 1, 1, 3,
        ];
        let expected_contrours = vec![
            false, true, true, false, false,
            true, false, true, true, false,
            true, true, false, false, true,
        ];
        for i in 0..scores.len() {
            assert_eq!(grid_labels.labels[i], expected_labels[i], "label is different. index: {}", i);
            assert_eq!(grid_labels.contrours[i], expected_contrours[i], "contrours is different. index: {}", i);
        }
    }
}
