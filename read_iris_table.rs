extern crate csv;
extern crate prettytable;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use prettytable::{Table, Row, Cell};

fn main() -> Result<(), Box<dyn Error>> {

    let file_path = "iris.csv";
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut table = Table::new();

    for result in rdr.records() {
        let record = result?;
        let cells: Vec<Cell> = record.iter().map(|field| Cell::new(field)).collect();
        table.add_row(Row::new(cells));
    }
    table.printstd();
    Ok(())
}

main();


