// src/lib.rs
use polars::prelude::*;
use std::error::Error;
use std::fs::File;

pub fn process_data(input_file: &str, output_file: &str, col_name: &str) -> Result<(), Box<dyn Error>> {
    // Read CSV into a DataFrame
    let file = File::open(input_file)?;
    let mut df = CsvReader::new(file)
        .finish()?;


    // Perform data processing
    df = df
    .lazy()
    .with_column(
        (col(col_name).fill_null(lit(0)) * lit(2)).alias("processed_column"),
    )
    .collect()?;

    // Write DataFrame to CSV
    let mut file = File::create(output_file)?;
    CsvWriter::new(&mut file)
        .finish(&mut df)?;

    Ok(())
}
