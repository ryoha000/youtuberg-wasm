use crate::utils;

pub fn get_divided_binary(binary: &(u32, u32, Vec<bool>)) -> utils::DividedBinary {
    let mut group_id = 1;
    let mut labels = vec![0; (binary.0 * binary.1) as usize];
    let mut areas = Vec::new();
    areas.push(0);
    let mut sizes = Vec::new();
    sizes.push(utils::DividedSize{ rows: 0, cols: 0 });
    for i in 0..binary.2.len() {
        // もう、このマスが探索されてたらスキップ
        if labels[i] != 0 {
            continue
        }

        let value = binary.2[i];
        labels[i] = group_id;
        // そのループで探索するindexが入ってる配列
        let mut to_check_indexes = utils::get_around_indexes(i as u32, &binary);

        let mut area = 1;
        let col = i as u32 % binary.0;
        let mut min_col = col;
        let mut max_col = col;
        let mut min_index = i;
        let mut max_index = i;

        loop {
            if to_check_indexes.len() == 0 {
                break
            }
            // 次のループで探索するindexを入れておく配列
            let mut new_to_check_indexes = Vec::new();
            // 上下左右で閾値を超えているものがあるか探索
            for j in 0..to_check_indexes.len() {
                // もう、このマスが探索されてたらスキップ
                if labels[to_check_indexes[j]] != 0 {
                    continue
                }

                if binary.2[to_check_indexes[j]] != value {
                    continue
                }
                labels[to_check_indexes[j]] = group_id;

                // sizeを作るための処理
                let col = (to_check_indexes[j] as u32) % binary.0;
                if max_col < col {
                    max_col = col;
                }
                if min_col > col {
                    min_col = col;
                }
                if min_index > to_check_indexes[j] {
                    min_index = to_check_indexes[j];
                }
                if max_index < to_check_indexes[j] {
                    max_index = to_check_indexes[j];
                }

                new_to_check_indexes.append(&mut utils::get_around_indexes(to_check_indexes[j] as u32, &binary));
                area += 1;
            }
            to_check_indexes = new_to_check_indexes;
        }
        areas.push(area);
        sizes.push(utils::DividedSize{ rows: max_index as u32 / binary.0 - min_index as u32 / binary.0 + 1, cols: max_col - min_col + 1 });
        group_id += 1;
    }
    let contrours = utils::get_contrours_from_labels(&labels, &binary);
    utils::DividedBinary{
        labels: labels, contrours: contrours, areas: areas, sizes: sizes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test;
    #[test]
    fn can_get_divided_binary() {
        let (width, height, gray) = test::load_gray_image();
        let binary = crate::threshold::gray_to_binary(&gray);
        let b_copy = binary.clone();
        let divided_binary = get_divided_binary(&(width, height, binary));
        test::get_visualized_labels(&divided_binary.labels, &(width, height, b_copy), 1, &vec![0]);
    }
}
