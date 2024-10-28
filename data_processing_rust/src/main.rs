use std::error::Error;
use polars::prelude::*;

fn process_data(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    // Read CSV into a DataFrame
    let mut df = CsvReader::from_path(input_file)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    // Perform data processing
    df = df.lazy()
        .with_column((col("value") * lit(2)).alias("processed_column"))
        .collect()?;

    // Write DataFrame to CSV
    CsvWriter::new(std::fs::File::create(output_file)?)
        .has_header(true)
        .finish(&mut df)?;

    Ok(())
}