#####################################################################
#####################################################################
#####################################################################
#####################################################################
#####################################################################

let root = BitMapBackend::new("line_plot1.png", (800, 600)).into_drawing_area();
root.fill(&WHITE)?; // Optional background color

let mut chart = ChartBuilder::on(&root)
    .build_cartesian_2d(0.0..100.0, 0.0..600.0)?;

chart.configure_mesh()
    .x_desc("Age")
    .y_desc("Fare")
    .draw()?;

chart.draw_series(LineSeries::new(
    a_values.iter().zip(b_values.iter()).map(|(x, y)| (*x, *y)),
    &RED,
));

#####################################################################
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
#####################################################################
