use clap::{Arg, Command};
use csv::{ReaderBuilder, WriterBuilder, StringRecord};
use std::error::Error;
use std::fs::File;

fn transpose_tsv(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the input file and create a CSV reader
    let file = File::open(input_path)?;
    let mut rdr = ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(file);

    // Read the headers separately
    let headers = rdr.headers()?.clone();

    // Read all records into a vector of vectors (matrix)
    let mut matrix: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        matrix.push(record.iter().map(String::from).collect());
    }

    // Prepare to write the transposed matrix
    let file = File::create(output_path)?;
    let mut wtr = WriterBuilder::new().delimiter(b'\t').from_writer(file);

    // Include the headers as the first column of the transposed matrix
    if !matrix.is_empty() {
        let rows = matrix.len();
        let cols = matrix[0].len();
        for c in 0..cols {
            let mut transposed_row = Vec::<String>::with_capacity(rows + 1);
            for r in 0..rows {
                transposed_row.push(matrix[r][c].clone());
            }
            wtr.write_record(&transposed_row)?;
        }
    }

    // Flush the writer to ensure all data is written to the file
    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("bigspose")
        .version("0.1.0")
        .author("Andrea Guarracino <aguarra1@uthsc.edu")
        .about("Transposes a big TSV file.")
        .arg(Arg::new("input")
            .help("The input TSV file to transpose.")
            .required(true)
            .index(1))
        .arg(Arg::new("output")
            .help("The output TSV file to store the transposed data.")
            .required(true)
            .index(2))
        .get_matches();

    // Get file paths from command-line arguments
    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();

    transpose_tsv(input_path, output_path)?;

    Ok(())
}

