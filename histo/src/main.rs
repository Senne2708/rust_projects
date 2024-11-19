mod histogram;
mod image_processing;
mod structs;
use walkdir::WalkDir;

fn main() {
    let dir_path: &str = "/Users/sennevandoorn/Documents/AI_Satellite/unet/data/archive/95-cloud_training_only_additional_to38-cloud/train_red_additional_to38cloud/";
    let mut dead_pixel_percent_counts: Vec<u32> = Vec::new();
    let mut count: usize = 0;
    for entry in WalkDir::new(&dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Some(path_str) = entry.path().to_str() {
                // Create paths for the red, green, blue, and NIR images
                let path = structs::UnmergedImage::new(
                    path_str,
                    &path_str.replace("red", "green"),
                    &path_str.replace("red", "blue"),
                    &path_str.replace("red", "nir"),
                );

                let merge_image = image_processing::merge_images(&path);
                let percent = image_processing::count_zero_pixels(&merge_image);
                dead_pixel_percent_counts.push(percent);
                count += 1;
                println!("Count: {count}");
            }
        }
    }

    let bucket_count = 10;
    histogram::plot_histogram(&dead_pixel_percent_counts, "output.png", bucket_count);
}
