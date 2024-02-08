extern crate csv;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "iris.csv";
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    for result in rdr.records() {
        let record = result?;
        for field in record.iter() {
            print!("{}, ", field);
        }
        println!();
    }
    Ok(())
}

main();








