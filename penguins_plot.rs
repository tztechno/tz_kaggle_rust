use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the CSV file
    let file = File::open("penguins.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Extract SepalLengthCm and PetalLengthCm columns
    let mut length_values = Vec::<f64>::new();
    let mut width_values = Vec::<f64>::new();

    for result in rdr.records() {
        let record = result?;
        if let (Some(_length_str), Some(_width_str)) = (record.get(2), record.get(3)) {
            if let (Ok(length), Ok(width)) = (_length_str.parse::<f64>(), _width_str.parse::<f64>()) {
                length_values.push(length);
                width_values.push(width);
            }
        }
    }

    // Create scatter plot
    let root = BitMapBackend::new("scatter_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Scatter Plot", ("sans-serif", 40))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..100.0, 0.0..35.0)?;

    chart.configure_mesh()
        .x_desc("culmen_length_mm")
        .y_desc("culmen_width_mm")
        .draw()?;

    chart.draw_series(
        length_values.iter().zip(width_values.iter()).map(|(x, y)| {
            Circle::new((*x, *y), 5, &RED)
        })
    )?;

    Ok(())
}
