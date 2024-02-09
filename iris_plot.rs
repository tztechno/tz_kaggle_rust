use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the CSV file
    let file = File::open("src/iris.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Extract SepalLengthCm and PetalLengthCm columns
    let mut sepal_length_values = Vec::<f64>::new();
    let mut petal_length_values = Vec::<f64>::new();

    for result in rdr.records() {
        let record = result?;
        if let (Some(sepal_length_str), Some(petal_length_str)) = (record.get(1), record.get(3)) {
            if let (Ok(sepal_length), Ok(petal_length)) = (sepal_length_str.parse::<f64>(), petal_length_str.parse::<f64>()) {
                sepal_length_values.push(sepal_length);
                petal_length_values.push(petal_length);
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
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

    chart.configure_mesh()
        .x_desc("SepalLengthCm")
        .y_desc("PetalLengthCm")
        .draw()?;

    chart.draw_series(
        sepal_length_values.iter().zip(petal_length_values.iter()).map(|(x, y)| {
            Circle::new((*x, *y), 5, &RED)
        })
    )?;

    Ok(())
}
