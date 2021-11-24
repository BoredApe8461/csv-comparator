use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;

use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type OptimizedSize = f32;
type UnoptimizedSize = f32;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Row {
    name: String,
    unoptimized_size: UnoptimizedSize,
    optimized_size: OptimizedSize,
}

#[derive(Debug, Default)]
struct CsvComparator {
    old_csv: HashMap<String, (UnoptimizedSize, OptimizedSize)>,
    new_csv: HashMap<String, (UnoptimizedSize, OptimizedSize)>,
}

impl CsvComparator {
    fn new() -> Self {
        Default::default()
    }

    fn write_old(&mut self, file: File) -> Result<()> {
        read_csv(&mut self.old_csv, file)
    }

    fn write_new(&mut self, file: File) -> Result<()> {
        read_csv(&mut self.new_csv, file)
    }

    fn get_diffs(&self) -> Result<Vec<Row>> {
        let mut result = Vec::new();
        let mut all_contracts: HashMap<String, ()> = self.old_csv.iter().map(|(k, _)| (k.clone(), ())).collect();
        self.new_csv.iter().for_each(|(k, _)| {
            all_contracts.insert(k.clone(), ());
        });

        for (contract, _) in all_contracts {
            let def = (UnoptimizedSize::default(), OptimizedSize::default());
            let (unopt_size_old, opt_size_old) = self.old_csv.get(&contract).or(Some(&def)).expect("failed getting old_csv entry");
            let (unopt_size_new, opt_size_new) = self.new_csv.get(&contract).or(Some(&def)).expect("failed getting new_csv entry");
            let unopt_diff = unopt_size_new - unopt_size_old;
            let opt_diff = opt_size_new - opt_size_old;

            let row = Row {
                name: contract.to_string(),
                unoptimized_size: unopt_diff,
                optimized_size: opt_diff,
            };
            result.push(row);
        }

        Ok(result)
    }
}

fn read_csv(map: &mut HashMap<String, (UnoptimizedSize, OptimizedSize)>, file: File) -> Result<()> {
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

    let mut comparator = CsvComparator::new();
    comparator.write_old(old)?;
    comparator.write_new(new)?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());

    for row in comparator.get_diffs()? {
        wtr.serialize(row)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_from_old_csv_must_appear_in_diff() {
        // given
        let mut old_csv: HashMap::<String, (UnoptimizedSize, OptimizedSize)> = HashMap::new();
        let new_csv: HashMap::<String, (UnoptimizedSize, OptimizedSize)> = HashMap::new();
        old_csv.insert("removed_in_new_csv".to_string(), (UnoptimizedSize::default(), OptimizedSize::default()));
        let comparator = CsvComparator {
            old_csv,
            new_csv,
        };

        // when
        let res = comparator.get_diffs().expect("getting diffs failed");

        // then
        let mut iter = res.iter();
        assert_eq!(iter.next().expect("first diff entry must exist"), &Row { name: "removed_in_new_csv".to_string(),
            unoptimized_size: UnoptimizedSize::default(),
            optimized_size: OptimizedSize::default(),
        });
        assert_eq!(iter.next(), None);
    }
}