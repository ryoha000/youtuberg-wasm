pub fn get_around_indexes(index: u32, binary: &(u32, u32, Vec<u32>)) -> Vec<u32> {
    // TODO: ここ一々配列をアロケートしてるのをなんとかする
    let mut result = Vec::new();
    let bottom = index + binary.0;
    let right = index + 1;

    // ちゃんとindex内にあるかチェック
    if index >= binary.0 {
        result.push(index - binary.0);
    }
    if bottom < binary.1 * binary.0 {
        result.push(bottom);
    }
    if index >= 1 {
        let left = index - 1;
        if left % binary.0 != 0 {
            result.push(left);
        }
    }
    if right % binary.0 != 0 && right < binary.1 * binary.0 {
        result.push(right);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test;
    #[test]
    fn can_get_correct_indexes_of_edge() {
        let binary = (10, 10, vec![]);
        
        let indexes = get_around_indexes(0, &binary);
        let correct = vec![10, 1];
        assert!(test::cmp_vec(&indexes, &correct), "indexes({:?}) not equal {:?}", indexes, correct);
    }
    #[test]
    fn can_get_around_indexes_of_center() {
        let binary = (10, 10, vec![]);

        let indexes = get_around_indexes(55, &binary);
        let correct = vec![45, 65, 54, 56];
        assert!(test::cmp_vec(&indexes, &correct), "indexes({:?}) not equal {:?}", indexes, correct);
    }
}
