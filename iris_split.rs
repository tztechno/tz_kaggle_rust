extern crate csv;
extern crate rand;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "iris.csv";
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut records: Vec<_> = rdr.records().collect::<Result<_, _>>()?;
    let mut rng = rand::thread_rng();
    records.shuffle(&mut rng);
    let split_point = (records.len() as f64 * 0.8) as usize;
    let (train_data, test_data) = records.split_at(split_point);

    println!("Train Data:");
    for record in train_data {
        println!("{:?}", record);
    }
    println!("\nTest Data:");
    for record in test_data {
        println!("{:?}", record);
    }
    Ok(())
}

main();




