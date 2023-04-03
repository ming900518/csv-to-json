use std::{fs::write, path::PathBuf, sync::Arc};

use clap::Parser;
use indexmap::IndexMap;
use mimalloc::MiMalloc;
use polars::prelude::*;
use rayon::prelude::*;
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

    let data_frame = Arc::new(
        CsvReader::from_path(args.input)
            .expect("File not found.")
            .has_header(true)
            .finish()
            .expect("Error when ."),
    );

    let column_names = Arc::new(data_frame.get_column_names());

    let height = data_frame.height();

    let result_vec = (0..height)
        .into_par_iter()
        .map(|i| {
            let row = data_frame.get_row(i).unwrap().0;
            column_names
                .iter()
                .zip(row.iter())
                .map(|(column, data)| (column.to_string(), data.get_str().unwrap_or("").to_owned()))
                .collect::<IndexMap<String, String>>()
        })
        .collect::<Vec<IndexMap<String, String>>>();

    let json = to_string(&result_vec).expect("Unable to serialize the result.");

    match args.output {
        Some(path) => write(path, &*json).expect("Unable to write the output file."),
        None => println!("{}", json),
    }
}
