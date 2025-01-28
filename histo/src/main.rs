mod histogram;
mod image_processing;
mod structs;
use walkdir::WalkDir;
use rayon::prelude::*;

fn main() {
    let dir_path: &str = "/Users/sennevandoorn/Documents/AI_Satellite/unet/data/archive/95-cloud_training_only_additional_to38-cloud/train_red_additional_to38cloud/";
    
    // Use parallel iterator to process images
    let dead_pixel_percent_counts: Vec<u32> = WalkDir::new(&dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge() // Multi-threading 
        .filter_map(|entry| {
            let path = entry.path();
            
            if path.is_file() {
                path.to_str().and_then(|path_str| {
                    // Create paths for the red, green, blue, and NIR images
                    let path = structs::UnmergedImage::new(
                        path_str,
                        &path_str.replace("red", "green"),
                        &path_str.replace("red", "blue"),
                        &path_str.replace("red", "nir"),
                    );

                    let merge_image = image_processing::merge_images(&path);
                    Some(image_processing::count_zero_pixels(&merge_image))
                })
            } else {
                None
            }
        })
        .collect();

    println!("Processed {} images", dead_pixel_percent_counts.len());

    let bucket_count = 11;
    histogram::plot_histogram(&dead_pixel_percent_counts, "output.png", bucket_count);
}
