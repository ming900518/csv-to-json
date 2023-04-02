use std::{collections::BTreeMap, fs::write, path::PathBuf, process::exit, sync::Arc};

use clap::Parser;
use csv::{Reader, StringRecord};
use mimalloc::MiMalloc;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use serde_json::to_string;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify CSV input file. Required.
    #[arg(short, long, value_name = "CSV FILE")]
    input: PathBuf,

    /// Specify JSON output file. Optional. (If not set, the result will be printed out directly.)
    #[arg(short, long, value_name = "JSON FILE")]
    output: Option<PathBuf>,
}

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let args = Args::parse();
    let mut csv_reader = match Reader::from_path(args.input) {
        Ok(reader) => reader,
        Err(error) => {
            eprintln!("Error occured when opening CSV file reader: {}", error);
            exit(1)
        }
    };

    let header_arc = if let Ok(header) = csv_reader.headers() {
        Arc::new(header.clone())
    } else {
        eprintln!("Unable to structure JSON because there's no header found in the CSV file you provided.");
        exit(1)
    };

    let rows = csv_reader
        .into_records()
        .collect::<Vec<Result<StringRecord, csv::Error>>>();

    let result_vec = rows
        .into_par_iter()
        .filter(|row_result| row_result.is_ok())
        .map(|row_result| {
            let row = row_result.unwrap();
            let mut map = BTreeMap::new();
            let header = header_arc.clone();
            header.iter().enumerate().for_each(|(i, head)| {
                map.insert(head.to_owned(), row.clone().get(i).unwrap().to_owned());
            });
            map
        })
        .collect::<Vec<BTreeMap<String, String>>>();

    let json = to_string(&result_vec).expect("Unable to serialize the result.");

    match args.output {
        Some(path) => write(path, &*json).expect("Unable to write the output file."),
        None => println!("{}", json),
    }
}
