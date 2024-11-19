use plotters::prelude::*;

fn cal_histogram(data: &[u32], num_bucket: usize) -> Vec<(u32, i32)> {
    let min = *data.iter().min().unwrap_or(&0);
    let max = *data.iter().max().unwrap_or(&100);
    let bucket_width = ((max - min) as f64 / num_bucket as f64).ceil() as u32;

    let mut buckets = vec![0; num_bucket];

    for &value in data {
        let bucket_index = ((value - min) / bucket_width).min(num_bucket as u32 - 1) as usize;
        buckets[bucket_index] += 1;
    }

    (0..num_bucket)
        .map(|i| (min + (i as u32 * bucket_width), buckets[i]))
        .collect()
}

// Add this function to plot the histogram
pub fn plot_histogram(data: &[u32], output_path: &str, bucket_count: usize) {
    let histogram_data = cal_histogram(data, bucket_count);

    let backend = BitMapBackend::new(output_path, (800, 600));
    let root = backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Dead Pixel Histogram", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(
            histogram_data.first().unwrap().0..histogram_data.last().unwrap().0,
            0..*histogram_data.iter().map(|(_, count)| count).max().unwrap(),
        )
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Pixel Value Ranges")
        .y_desc("Counts")
        .draw()
        .unwrap();

    chart
        .draw_series(histogram_data.iter().map(|&(range, count)| {
            Rectangle::new(
                [
                    (range, 0),
                    (range + (histogram_data[1].0 - histogram_data[0].0), count),
                ],
                RED.filled(),
            )
        }))
        .unwrap();
}