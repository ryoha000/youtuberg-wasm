// ref: https://algorithm.joho.info/programming/python/opencv-otsu-thresholding-py/
mod histgram;
pub fn gray_to_binary(gray: &[u8]) -> Vec<bool> {
    let hist = histgram::get_hist(gray);
    let threshold = histgram::get_threshold_by_hist(&hist);
    let mut binary = Vec::with_capacity(gray.len());
    for i in gray {
        if i < &threshold {
            binary.push(false)
        } else {
            binary.push(true)
        }
    }
    binary
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test;
    #[test]
    fn can_gray_to_binary() {
        let (width, height, gray) = test::load_gray_image();
        let binary = gray_to_binary(&gray);
        let binary_img = test::binary_to_image((width, height, binary));
        let diff = test::image_diff_ratio(binary_img, "opencv_binary");
        assert!(diff < 0.03, "diff({}) over 3%", diff);
    }
}
