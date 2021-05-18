use super::divide::DividedBinary;

// block_sideは文字サイズと同じという想定
pub fn get_filtered_binary(divided_binary: &DividedBinary, block_side: u32) -> Vec<bool> {
    let mut result = Vec::with_capacity(divided_binary.labels.len());
    for i in 0..divided_binary.labels.len() {
        // 文字より長いもの禁止
        if divided_binary.sizes[divided_binary.labels[i] as usize].rows > block_side
        || divided_binary.sizes[divided_binary.labels[i] as usize].cols > block_side {
            result.push(false);
            continue
        }
    
        // 塗り面積が大きいのと極端に小さいノイズを無視
        if divided_binary.areas[divided_binary.labels[i] as usize] > block_side * block_side / 2
        || divided_binary.areas[divided_binary.labels[i] as usize] < block_side * block_side / 100 {
            result.push(false);
            continue
        } else {
            result.push(true);
            continue
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::divide::get_divided_binary;
    use crate::utils::test;
    #[test]
    fn can_noise_filter() {
        let (width, height, gray) = test::load_gray_image();
        let binary = crate::threshold::gray_to_binary(&gray);
        let divided_binary = get_divided_binary(&(width, height, binary));
        let noise_filtered_binary = get_filtered_binary(&divided_binary, width / 40);
        test::binary_to_image((width, height, noise_filtered_binary));
    }
}
