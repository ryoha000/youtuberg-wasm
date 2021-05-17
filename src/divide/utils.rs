pub fn get_around_indexes(index: u32, binary: &(u32, u32, Vec<u32>)) -> Vec<u32> {
    // TODO: ここ一々配列をアロケートしてるのをなんとかする
    let mut result = Vec::new();
    let top = index - binary.0;
    let bottom = index + binary.0;
    let left = index - 1;
    let right = index + 1;

    // ちゃんとindex内にあるかチェック
    if top >= 0 {
        result.push(top)
    }
    if bottom < binary.1 * binary.0 {
        result.push(bottom)
    }
    if left % binary.0 != 0 && left >= 0 {
        result.push(left)
    }
    if right % binary.0 != 0 && right < binary.1 * binary.0 {
        result.push(right)
    }
    result
}
