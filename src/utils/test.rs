#[cfg(test)]
use image::{GenericImageView, RgbaImage, ImageBuffer};

#[cfg(test)]
pub fn load_gray_image() -> (u32, u32, Vec<u8>) {
    let img = image::open("assets/gray.png").unwrap();
    let (width, height) = img.dimensions();
    let mut gray = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            gray.push(img.get_pixel(x, y)[0]);
        }
    }
    (width, height, gray)
}

#[cfg(test)]
pub fn binary_to_image(binary: (u32, u32, Vec<bool>)) -> RgbaImage {
    let width = binary.0;
    let height = binary.1;
    let mut img: RgbaImage = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let b = binary.2[(y * width + x) as usize];
            let mut pixel = img[(x, y)];
            pixel[0] = 255 * b as u8;
            pixel[1] = 255 * b as u8;
            pixel[2] = 255 * b as u8;
            pixel[3] = 255;
            img.put_pixel(x, y, pixel)
        }
    }
    img
}

#[cfg(test)]
pub fn image_diff_ratio(img: RgbaImage, dst_file: &str) -> f32 {
    let dst_img = image::open(format!("assets/{}.png", dst_file)).unwrap();
    let (width, height) = img.dimensions();
    let mut diff = 0;
    for y in 0..height {
        for x in 0..width {
            let src_p = img.get_pixel(x, y);
            let dst_p = dst_img.get_pixel(x, y);
            if src_p[0] != dst_p[0] || src_p[1] != dst_p[1] || src_p[2] != dst_p[2] {
                diff += 1;
            }
        }
    }
    diff as f32 / (width as f32 * height as f32)
}

#[cfg(test)]
pub fn cmp_vec<T: std::cmp::Eq>(src: &[T], dst: &[T]) -> bool {
    if src.len() != dst.len() {
        return false
    }
    for i in 0..src.len() {
        if src[i] != dst[i] {
            return false
        }
    }
    true
}
