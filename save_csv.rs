use std::error::Error;
use std::fs::File;

use csv::{Reader, WriterBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    // Specify the path to the input CSV file
    let input_file_path = "./Iris.csv";

    // Specify the path to the output CSV file
    let output_file_path = "./Iris2.csv";

    // Read CSV from the input file
    let mut csv_reader = Reader::from_path(input_file_path)?;

    // Retrieve header record
    let header_record = csv_reader.headers()?.clone();

    // Open the output CSV file for writing
    let output_file = File::create(output_file_path)?;
    let mut csv_writer = WriterBuilder::new().from_writer(output_file);

    // Write the header record to the output file
    csv_writer.write_record(&header_record)?;

    // Iterate through the records and write them to the output file
    for result in csv_reader.records() {
        let record = result?;
        csv_writer.write_record(&record)?;
    }

    // Flush the writer to ensure data is written to the file
    csv_writer.flush()?;

    println!("CSV file successfully copied.");

    Ok(())
}
