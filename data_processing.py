import polars as pl

def process_data(input_file, output_file):
    # Read the CSV file into a Polars DataFrame
    df = pl.read_csv(input_file)
    
    # Create a new column 'processed_column' by multiplying 'value' by 2
    df = df.with_column((pl.col('value') * 2).alias('processed_column'))
    
    # Write the DataFrame to a new CSV file
    df.write_csv(output_file)

if __name__ == "__main__":
    process_data('/data/input.csv', 'output.csv')

