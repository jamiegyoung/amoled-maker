use core::panic;
use std::{env, ops::Index, path::Path};

use amoled_maker::{generate_new_black_point_image, get_black_pixel_info, save_image};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        panic!("Please pass an image path!");
    }
    let path = args.index(1);
    let path_exists = Path::new(path).exists();
    if !path_exists {
        panic!("Image cannot be found!")
    }

    // Black pixel percentage
    match args.iter().position(|s| s.as_str() == "-p") {
        Some(_) => {
            get_black_pixel_info(path);
        }
        None => {}
    }

    // Create new image
    match args.iter().position(|s| s.as_str() == "-c") {
        Some(index) => {
            let black_point = args.index(index + 1).parse::<u8>().unwrap();
            println!(
                "Making amoled verison with a black point at the rgb value {}",
                black_point
            );
            let new_image = generate_new_black_point_image(path, black_point);
            save_image(new_image, "./amoled_image.png").unwrap();
        }
        None => {}
    }
}
