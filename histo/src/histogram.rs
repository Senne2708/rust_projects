use plotters::prelude::*;

pub fn plot_histogram(data: &[u32], output_path: &str, bucket_count: usize) {
    let histogram_data = cal_histogram(data, bucket_count);

    let backend = BitMapBackend::new(output_path, (800, 600));
    let root = backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Find max count for y-axis
    let max_count = histogram_data.iter().map(|&(_, count)| count).max().unwrap_or(0);

    // Adjust x-axis range to ensure 100% is visible
    let min_range = histogram_data.first().unwrap().0;
    let max_range = histogram_data.last().unwrap().0 + 
        (histogram_data[1].0 - histogram_data[0].0);

    let mut chart = ChartBuilder::on(&root)
        .caption("Dead Pixel Histogram", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(
            min_range..max_range,
            0..(max_count as f64 * 1.1) as i32,
        )
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Pixel Value Ranges (%)")
        .y_desc("Image Counts")
        .draw()
        .unwrap();

    chart
        .draw_series(histogram_data.iter().map(|&(range, count)| {
            let bucket_width = histogram_data[1].0 - histogram_data[0].0;
            Rectangle::new(
                [
                    (range, 0),
                    (range + bucket_width, count),
                ],
                RED.filled(),
            )
        }))
        .unwrap();
}

fn cal_histogram(data: &[u32], num_bucket: usize) -> Vec<(u32, i32)> {
    let min = *data.iter().min().unwrap_or(&0);
    let max = *data.iter().max().unwrap_or(&100);
    
    let bucket_width = 10;

    let mut buckets = vec![0; num_bucket];

    for &value in data {
        let bucket_index = ((value - min) / bucket_width).min(num_bucket as u32 - 2) as usize;
        buckets[bucket_index] += 1;
    }

    (0..num_bucket-1)
        .map(|i| (min + (i as u32 * bucket_width), buckets[i]))
        .collect()
}
