use crate::structs::UnmergedImage;
use image::{self, Rgba, RgbaImage};

pub fn merge_images(file_paths: &UnmergedImage) -> RgbaImage {
    let img_r = image::open(&file_paths.image_path_r).expect("Can find image_path_r");
    let img_g = image::open(&file_paths.image_path_g).expect("Can find image_path_g");
    let img_b = image::open(&file_paths.image_path_b).expect("Can find image_path_b");
    let img_nir = image::open(&file_paths.image_path_nir).expect("Can find image_path_nir");

    let gray_r = img_r.to_luma8();
    let gray_g = img_g.to_luma8();
    let gray_b = img_b.to_luma8();
    let gray_nir = img_nir.to_luma8();

    let (width, height) = gray_r.dimensions();

    let mut output_image = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let r = gray_r.get_pixel(x, y)[0];
            let g = gray_g.get_pixel(x, y)[0];
            let b = gray_b.get_pixel(x, y)[0];
            let nir = gray_nir.get_pixel(x, y)[0];

            output_image.put_pixel(x, y, Rgba([r, g, b, nir]));
        }
    }

    output_image
}

pub fn count_zero_pixels(rgba_image: &RgbaImage) -> u32 {
    let mut dead_pixel: u32 = 0;
    let (width, height) = rgba_image.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = rgba_image.get_pixel(x, y);
            let (r, g, b, nir) = (pixel[0], pixel[1], pixel[2], pixel[3]);

            if r == 0 && g == 0 && b == 0 && nir == 0 {
                dead_pixel += 1;
            }
        }
    }

    let percent: f32 = ((dead_pixel as f32) / (width * height) as f32) * 100.0;
    let percent_round = percent.round() as u32;

    percent_round
}
