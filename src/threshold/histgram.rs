pub fn get_hist(gray: &[u8]) -> [u8; 256] {
    let mut hist: [u8; 256] = [0; 256];
    for i in gray.iter() {
        hist[*i as usize] += 1;
    }
    hist
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
