use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    name: String,
    unoptimized_size: f32,
    optimized_size: f32,
}

#[derive(Debug)]
struct CsvComparitor {
    old_csv: HashMap<String, f32>,
    new_csv: HashMap<String, f32>,
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

    pub fn get_diffs(&mut self) {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        for (k, v) in &self.old_csv {
            let new_size = self.new_csv.get(k).unwrap();
            let old_size = v;
            let diff = new_size - old_size;

            dbg!(&k);
            dbg!(diff);

            wtr.serialize(Record {
                name: k.to_string(),
                unoptimized_size: diff,
                optimized_size: Default::default(),
            }).unwrap();
        }
    }
}

fn read_csv(map: &mut HashMap<String, f32>, file: File) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(file);

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        // println!("{:?}", record);
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
    dbg!(&comparitor);

    comparitor.get_diffs();

    // if let Err(err) = read_csv(f) {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }

    Ok(())
}
