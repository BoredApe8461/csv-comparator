use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;

use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Serialize, Deserialize)]
struct Row {
    name: String,
    unoptimized_size: f32,
    optimized_size: f32,
}

#[derive(Debug, Default)]
struct CsvComparitor {
    old_csv: HashMap<String, (f32, f32)>,
    new_csv: HashMap<String, (f32, f32)>,
}

impl CsvComparitor {
    fn new() -> Self {
        Default::default()
    }

    fn write_old(&mut self, file: File) -> Result<()> {
        read_csv(&mut self.old_csv, file)
    }

    fn write_new(&mut self, file: File) -> Result<()> {
        read_csv(&mut self.new_csv, file)
    }

    fn get_diffs(&mut self) -> Result<()> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        for (contract, (unopt_size_old, opt_size_old)) in &self.old_csv {
            let (unopt_size_new, opt_size_new) = self.new_csv.get(contract).unwrap();
            let unopt_diff = unopt_size_new - unopt_size_old;
            let opt_diff = opt_size_new - opt_size_old;

            dbg!(&contract);
            dbg!(opt_diff);
            dbg!(unopt_diff);

            wtr.serialize(Row {
                name: contract.to_string(),
                unoptimized_size: unopt_diff,
                optimized_size: opt_diff,
            })?;
        }

        Ok(())
    }
}

fn read_csv(map: &mut HashMap<String, (f32, f32)>, file: File) -> Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(file);

    for result in rdr.deserialize() {
        let record: Row = result?;
        map.insert(
            record.name,
            (record.unoptimized_size, record.optimized_size),
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let old = File::open(&args[1])?;
    let new = File::open(&args[2])?;

    let mut comparitor = CsvComparitor::new();
    comparitor.write_old(old)?;
    comparitor.write_new(new)?;
    comparitor.get_diffs()?;

    // if let Err(err) = read_csv(f) {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }

    Ok(())
}
