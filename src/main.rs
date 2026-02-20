use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // 1. Handle command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Read from a file if an argument is provided, otherwise read from stdin
    let mut reader: Box<dyn BufRead> = if args.len() > 1 {
        Box::new(BufReader::new(File::open(&args[1])?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut rows: Vec<Vec<String>> = Vec::new();

    // 2. Read lines and split into columns
    for line_result in reader.as_mut().lines() {
        let line = line_result?;
        
        // Split by whitespace.
        let cols: Vec<String> = line.split_whitespace().map(String::from).collect();
        
        if !cols.is_empty() {
            rows.push(cols);
        }
    }

    if rows.is_empty() {
        return Ok(());
    }

    // 3. Calculate the maximum width of each column
    let mut col_widths: Vec<usize> = Vec::new();
    for row in &rows {
        for (i, col) in row.iter().enumerate() {
            if i >= col_widths.len() {
                col_widths.push(col.len());
            } else if col.len() > col_widths[i] {
                col_widths[i] = col.len();
            }
        }
    }

    // 4. Print the aligned output
    let padding = 2; // Number of spaces between columns
    for row in rows {
        for (i, col) in row.iter().enumerate() {
            print!("{}", col);
            
            // Apply padding to all columns except the last one in the row
            if i < row.len() - 1 {
                let spaces_needed = col_widths[i] - col.len() + padding;
                print!("{}", " ".repeat(spaces_needed));
            }
        }
        println!();
    }

    Ok(())
}