// src/main.rs
use std::error::Error;
use data_processing_rust::process_data;

fn main() -> Result<(), Box<dyn Error>> {
    process_data("python_script_with_rust/data/input.csv", 
    "python_script_with_rust/output.csv", "Fare")?;
    Ok(())
}
