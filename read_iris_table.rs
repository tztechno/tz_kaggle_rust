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


/*
+-----+-----+-----+-----+-----+-----------------+
| 1   | 5.1 | 3.5 | 1.4 | 0.2 | Iris-setosa     |
+-----+-----+-----+-----+-----+-----------------+
| 2   | 4.9 | 3.0 | 1.4 | 0.2 | Iris-setosa     |
+-----+-----+-----+-----+-----+-----------------+
| 3   | 4.7 | 3.2 | 1.3 | 0.2 | Iris-setosa     |
+-----+-----+-----+-----+-----+-----------------+
| 4   | 4.6 | 3.1 | 1.5 | 0.2 | Iris-setosa     |
+-----+-----+-----+-----+-----+-----------------+
*/
