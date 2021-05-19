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
