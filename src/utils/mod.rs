pub mod test;

pub struct GridLabel {
    pub labels: Vec<u32>,
    pub contrours: Vec<bool>,
}

pub struct DividedBinary {
    pub labels: Vec<u32>,
    pub areas: Vec<u32>,
    pub sizes: Vec<DividedSize>,
}

pub struct DividedSize {
    pub rows: u32,
    pub cols: u32,
}

pub struct Grid {
    pub rows: usize,
    pub cols: usize,
}

pub struct ImageSize {
    pub width: usize,
    pub height: usize,
}

pub fn get_around_indexes(index: usize, size: &ImageSize) -> Vec<usize> {
    // TODO: ここ一々配列をアロケートしてるのをなんとかする
    let mut result = Vec::new();
    let bottom = index + size.width;
    let right = index + 1;

    // ちゃんとindex内にあるかチェック
    if index >= size.width {
        result.push((index - size.width) as usize);
    }
    if bottom < size.height * size.width {
        result.push(bottom as usize);
    }
    if index >= 1 {
        let left = index - 1;
        if left % size.width != size.width - 1 {
            result.push(left as usize);
        }
    }
    if right % size.width != size.width - 1 && right < size.height * size.width {
        result.push(right as usize);
    }
    result
}

// TODO: ここでやってる処理はlabelsを組み立てるときにできるはず
pub fn get_contrours_from_labels(labels: &[u32], size: &ImageSize) -> Vec<bool> {
    let mut contrours = vec![false; labels.len()];
    for i in 0..labels.len() {
        contrours[i] = false;
        if labels[i] == 1 {
            continue
        }
        let indexes = get_around_indexes(i, size);
        if indexes.len() != 4 {
            contrours[i] = true;
        }
        let mut is_edge = false;
        for j in 0..indexes.len() {
            if labels[indexes[j]] == 1 {
                is_edge = true;
                break;
            }
        }
        if is_edge {
            contrours[i] = true;
        }
    }
    contrours
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test;
    #[test]
    fn can_get_correct_indexes_of_left_top_edge() {        
        let indexes = get_around_indexes(0, &ImageSize{ width: 10, height: 10 });
        let correct = vec![10, 1];
        assert!(test::cmp_vec(&indexes, &correct), "indexes({:?}) not equal {:?}", indexes, correct);
    }
    #[test]
    fn can_get_correct_indexes_of_left_center() {        
        let indexes = get_around_indexes(11, &ImageSize{ width: 10, height: 10 });
        let correct = vec![1, 21, 10, 12];
        assert!(test::cmp_vec(&indexes, &correct), "indexes({:?}) not equal {:?}", indexes, correct);
    }
    #[test]
    fn can_get_around_indexes_of_center() {
        let indexes = get_around_indexes(55, &ImageSize{ width: 10, height: 10 });
        let correct = vec![45, 65, 54, 56];
        assert!(test::cmp_vec(&indexes, &correct), "indexes({:?}) not equal {:?}", indexes, correct);
    }
}
