// src/main.rs
use std::error::Error;
use data_processing_rust::process_data;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {

    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.pop();
    input_path.push("data/input.csv");

    let mut output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    output_path.pop(); // Move up one directory level
    output_path.push("output.csv");

    process_data(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        "Fare",
    )?;
    Ok(())
}
