####################################################

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

fn read_csv_file(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let records: Vec<Vec<String>> = rdr.records().map(|r| r.unwrap().iter().map(|s| s.to_string()).collect()).collect();
    Ok(records)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./iris.csv";
    let data = read_csv_file(file_path)?;
    // Use the 'data' vector as needed
    println!("{:?}", data);
    Ok(())
}

####################################################

mod iris {
    // Assuming load_dataset is implemented to load data in the desired format
    pub fn load_dataset() -> Vec<Vec<String>> {
        // Implementation of loading data, replace this with the actual implementation
        vec![vec!["value1".to_string(), "value2".to_string()], vec!["value3".to_string(), "value4".to_string()]]
    }
}

fn main() {
    let data = iris::load_dataset();
    // Use the 'data' vector as needed
    println!("{:?}", data);
}
####################################################
