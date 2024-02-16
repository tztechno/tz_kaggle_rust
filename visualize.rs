use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the CSV file
    let file = File::open("/kaggle/input/titanic/train.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Define the column indices you want to extract
    let column_indices = vec![0,1,2,3,4,5,6,7,8,9];

    // Create a vector to store the values for each column
    let mut column_values: Vec<Vec<f64>> = vec![Vec::new(); column_indices.len()];

    for result in rdr.records() {
        let record = result?;

        // Iterate over the specified column indices
        for (i, &column_index) in column_indices.iter().enumerate() {
            // Try to parse the value at the current column index
            if let Some(value_str) = record.get(column_index) {
                if let Ok(value) = value_str.parse::<f64>() {
                    // Push the parsed value into the corresponding vector
                    column_values[i].push(value);
                }
            }
        }
    }

    
    // Now you have vectors containing values for each specified column
    let a_values = &column_values[5];
    let b_values = &column_values[9];
    let c_values = &column_values[1];

    
    // Create scatter plot
    let root = BitMapBackend::new("scatter_plot1.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Scatter Plot", ("sans-serif", 40))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..100.0, 0.0..600.0)?;

    chart.configure_mesh()
        .x_desc("Age")
        .y_desc("Fare")
        .draw()?;

    chart.draw_series(
        a_values.iter().zip(b_values.iter()).zip(c_values.iter()).map(|((x, y), &c)| {
            Circle::new((*x, *y), 5, &Palette99::pick(c as usize))
        })
    )?;

    Ok(())
}
