use polars::prelude::*;
use std::error::Error;
use std::fs::File;

pub fn process_data(input_file: &str, output_file: &str, col_name: &str) -> Result<(), Box<dyn Error>> {
    // Read CSV into a DataFrame
    let df = CsvReader::from_path(input_file)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    // Perform data processing
    let df = df
        .lazy()
        .with_column(
            col(col_name)
                .fill_null(lit(0))
                .mul(lit(2))
                .alias("processed_column"),
        )
        .collect()?;

    // Write DataFrame to CSV
    let mut file = File::create(output_file)?;
    CsvWriter::new(&mut file)
        .has_header(true)
        .finish(&df)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    process_data("data/input.csv", "output.csv")?;
    Ok(())
}
