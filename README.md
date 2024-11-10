# bigspose

A command-line tool for efficiently transposing large TSV (Tab-Separated Values) files.

## Description

`bigspose` is a Rust-based utility that transposes TSV files - converting rows to columns and columns to rows. It's designed to handle large files efficiently and preserves the tab-separated format.

## Installation

### From Source

1. Ensure you have Rust and Cargo installed (https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/bigspose
   cd bigspose
   ```
3. Build and install using Cargo:
   ```bash
   cargo install --path .
   ```

## Usage

Basic usage:
```bash
bigspose INPUT_FILE OUTPUT_FILE
```

Example:
```bash
bigspose input.tsv transposed_output.tsv
```

### Arguments

- `INPUT_FILE`: Path to the input TSV file you want to transpose
- `OUTPUT_FILE`: Path where the transposed TSV file will be saved

## Author

Andrea Guarracino <aguarra1@uthsc.edu>
