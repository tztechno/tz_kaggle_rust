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


/*
Train Data:
StringRecord(["147", "6.3", "2.5", "5.0", "1.9", "Iris-virginica"])
StringRecord(["131", "7.4", "2.8", "6.1", "1.9", "Iris-virginica"])
StringRecord(["139", "6.0", "3.0", "4.8", "1.8", "Iris-virginica"])
StringRecord(["39", "4.4", "3.0", "1.3", "0.2", "Iris-setosa"])
StringRecord(["111", "6.5", "3.2", "5.1", "2.0", "Iris-virginica"])
StringRecord(["106", "7.6", "3.0", "6.6", "2.1", "Iris-virginica"])
StringRecord(["8", "5.0", "3.4", "1.5", "0.2", "Iris-setosa"])
StringRecord(["97", "5.7", "2.9", "4.2", "1.3", "Iris-versicolor"])
*/

