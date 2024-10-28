import polars as pl

def process_data(input_file, output_file, col_name):
    # Read the CSV file into a Polars DataFrame

    df = pl.read_csv(input_file)
    
    # Create a new column 'processed_column' by multiplying 'value' by 2
    df = df.with_columns(
        (pl.col(col_name).fill_null(0) * 2).alias('processed_column')
    )
    
    # Write the DataFrame to a new CSV file
    df.write_csv(output_file)

if __name__ == "__main__":
    process_data(r'data/input.csv', 'output.csv', "Fare")

