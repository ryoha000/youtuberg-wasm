// ref: https://algorithm.joho.info/programming/python/opencv-otsu-thresholding-py/
mod histgram;
pub fn gray_to_binary(gray: &Vec<u8>) -> Vec<bool> {
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
