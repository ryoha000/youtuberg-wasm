pub fn get_hist(gray: &[u8]) -> [u8; 256] {
    let mut hist: [u8; 256] = [0; 256];
    for i in gray.iter() {
        hist[*i as usize] += 1;
    }
    hist
}

pub fn get_threshold_by_hist(hist: &[u8; 256]) -> u8 {
    // 元画像の要素の合計
    let mut number_sum: u32 = 0;
    // 画素値の合計
    let mut pixel_value_sum: u32 = 0;
    for i in 0..255 {
        number_sum += hist[i as usize] as u32;
        pixel_value_sum += hist[i as usize] as u32 * i;
    }
    let mut max_s_variance = (0, 0);
    let mut next = (0, number_sum, 0, pixel_value_sum);
    for i in 0..255 {
        let mu1: u32;
        match next.0 {
            0 => mu1 = 0,
            length => mu1 = next.2 / length
        }

        let mu2: u32;
        match next.0 {
            0 => mu2 = 0,
            length => mu2 = next.2 / length
        }

        let s = (next.0 * next.1 * (mu1 - mu2)).pow(2);

        if max_s_variance.1 < s {
            max_s_variance = (i, s);
        }

        let next_hist = hist[i as usize + 1] as u32;
        next = (next.0 + next_hist, next.1 - next_hist, next.2 + next_hist * (i as u32 + 1), next.3 - next_hist * (i as u32 - 1));
    }
    max_s_variance.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_hist_works() {
        let gray = [100, 0, 1, 2, 4, 100, 4, 100];
        let hist = get_hist(&gray);
        for i in 0..256 {
            match i {
                0 | 1 | 2 => assert_eq!(hist[i as usize], 1),
                4 => assert_eq!(hist[i as usize], 2),
                100 => assert_eq!(hist[i as usize], 3),
                _ => assert_eq!(hist[i as usize], 0)
            };
        }
    }
}
