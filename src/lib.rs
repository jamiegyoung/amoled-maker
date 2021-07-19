use std::{ops::Index, usize};

use image::{
    io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, ImageResult, Pixels,
    Rgba,
};

pub fn get_black_pixel_count(pixel_iter: Pixels<DynamicImage>) -> usize {
    pixel_iter.fold(0, |acc: usize, x| {
        let r: u8 = x.2.index(0).to_owned();
        let g: u8 = x.2.index(1).to_owned();
        let b: u8 = x.2.index(2).to_owned();
        if r == 0 && g == 0 && b == 0 {
            return acc + 1;
        }
        acc
    })
}

pub fn get_pixel_count(pixel_iter: Pixels<DynamicImage>) -> usize {
    pixel_iter.count()
}

pub fn get_black_pixel_info(path: &str) {
    let img: DynamicImage = ImageReader::open(path).unwrap().decode().unwrap();
    let pixel_iter: Pixels<DynamicImage> = img.pixels();
    println!(
        "Black pixel percentage: {}% - {}/{}",
        calc_black_pixel_percentage_with_iter(pixel_iter.clone()),
        get_black_pixel_count(pixel_iter.clone()),
        get_pixel_count(pixel_iter)
    );
}

pub fn calc_black_pixel_percentage_with_iter(pixel_iter: Pixels<DynamicImage>) -> f32 {
    let total_pixel_count: usize = get_pixel_count(pixel_iter.clone());
    let black_pixel_count: usize = get_black_pixel_count(pixel_iter);
    calc_black_pixel_percentage(black_pixel_count, total_pixel_count)
}

pub fn calc_black_pixel_percentage(black_pixel_count: usize, total_pixel_count: usize) -> f32 {
    (black_pixel_count as f32 / total_pixel_count as f32 * 10000.0).round() / 100.0
}

pub fn save_image(dynamic_image: DynamicImage, path: &str) -> ImageResult<()> {
    dynamic_image.save(path)?;
    Ok(())
}

pub fn generate_new_black_point_image(path: &str, black_point: u8) -> DynamicImage {
    let mut mut_img: DynamicImage = ImageReader::open(path).unwrap().decode().unwrap();
    let tmp_image = mut_img.clone();
    let pixel_iter: Pixels<DynamicImage> = tmp_image.pixels();

    let mut black_pixel_count = 0;
    let mut pixel_count = 0;

    for x in pixel_iter {
        pixel_count += 1;
        let r: u8 = x.2.index(0).to_owned();
        if r > black_point {
            continue;
        }

        let g: u8 = x.2.index(1).to_owned();
        if g > black_point {
            continue;
        }

        let b: u8 = x.2.index(2).to_owned();
        if b > black_point {
            continue;
        }

        black_pixel_count += 1;
        if r != 0 && g != 0 && b != 0 {
            mut_img.put_pixel(x.0, x.1, Rgba([0, 0, 0, *x.2.index(3)]))
        }
    }

    println!(
        "New black pixel percentage: {}% - {}/{}",
        calc_black_pixel_percentage(black_pixel_count, pixel_count),
        black_pixel_count,
        pixel_count
    );
    mut_img
}

#[cfg(test)]
mod tests {
    use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

    use crate::{calc_black_pixel_percentage_with_iter, get_black_pixel_count, get_pixel_count};

    const TEST_IMG_WIDTH: u32 = 10;
    const TEST_IMG_HEIGHT: u32 = 10;

    fn generate_colored_image() -> DynamicImage {
        let mut image: DynamicImage = DynamicImage::new_rgb16(TEST_IMG_WIDTH, TEST_IMG_HEIGHT);
        image.put_pixel(5, 5, Rgba([255, 255, 255, 255]));
        image.put_pixel(4, 4, Rgba([255, 255, 255, 255]));
        return image;
    }

    #[test]
    fn test_black_pixel_count() {
        let image1: DynamicImage = DynamicImage::new_rgb16(TEST_IMG_WIDTH, TEST_IMG_HEIGHT);
        let image2: DynamicImage = generate_colored_image();

        assert_eq!(get_black_pixel_count(image1.pixels()), 100);
        assert_eq!(get_black_pixel_count(image2.pixels()), 98);
    }

    #[test]
    fn test_black_pixel_percentage() {
        let image: DynamicImage = DynamicImage::new_rgb16(TEST_IMG_WIDTH, TEST_IMG_HEIGHT);
        let image2: DynamicImage = generate_colored_image();

        assert_eq!(calc_black_pixel_percentage_with_iter(image.pixels()), 100.0);
        assert_eq!(calc_black_pixel_percentage_with_iter(image2.pixels()), 98.0);
    }

    #[test]
    fn test_pixel_count() {
        let image1: DynamicImage = DynamicImage::new_rgb16(TEST_IMG_WIDTH, TEST_IMG_HEIGHT);
        let image2: DynamicImage = DynamicImage::new_rgb16(1920, 1080);
        let image3: DynamicImage = generate_colored_image();

        assert_eq!(get_pixel_count(image1.pixels()), 100);
        assert_eq!(get_pixel_count(image2.pixels()), 2073600);
        assert_eq!(get_pixel_count(image3.pixels()), 100);
    }
}
