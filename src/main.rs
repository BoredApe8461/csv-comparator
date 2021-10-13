use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    // TODO: Make these floats
    unoptimized_size: String,
    optimized_size: String,
}

#[derive(Debug)]
struct CsvComparitor {
    old_csv: HashMap<String, String>,
    new_csv: HashMap<String, String>,
}

impl CsvComparitor {
    pub fn new() -> Self {
        Self {
            old_csv: Default::default(),
            new_csv: Default::default(),
        }
    }

    pub fn write_old(&mut self, file: File) {
        read_csv(&mut self.old_csv, file).unwrap();
    }

    pub fn write_new(&mut self, file: File) {
        read_csv(&mut self.new_csv, file).unwrap();
    }
}

fn read_csv(map: &mut HashMap<String, String>, file: File) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(file);

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
        map.insert(record.name, record.unoptimized_size);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let old = File::open(&args[1])?;
    let new = File::open(&args[2])?;

    let mut comparitor = CsvComparitor::new();
    comparitor.write_old(old);
    comparitor.write_new(new);
    dbg!(comparitor);

    // if let Err(err) = read_csv(f) {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }

    Ok(())
}
