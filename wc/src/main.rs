use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

/// Print newline, word, and byte counts for each FILE.
#[derive(Parser)]
#[command(name = "wc", version = "0.1.0")]
struct Cli {
    /// Print the newline counts
    #[arg(short = 'l', long = "lines")]
    lines: bool,

    /// Print the word counts
    #[arg(short = 'w', long = "words")]
    words: bool,

    /// Print the byte counts
    #[arg(short = 'c', long = "bytes")]
    bytes: bool,

    /// Print the character counts
    #[arg(short = 'm', long = "chars")]
    chars: bool,

    /// Print the maximum display width (length of longest line)
    #[arg(short = 'L', long = "max-line-length")]
    max_line_length: bool,

    /// File to process
    #[arg(required = true, value_name = "FILE")]
    file_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Cli::parse();

    if !args.lines && !args.words && !args.bytes && !args.chars && !args.max_line_length {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let file = File::open(&args.file_path)?;
    let mut reader = BufReader::new(file);

    // We initialize all counters
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    let mut max_width = 0;

    let mut current_line_chars = 0;
    let mut in_word = false;
    let mut buffer = [0; 16384];

    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break; // end of file
        }

        for &byte in &buffer[..bytes_read] {
            total_bytes += 1;

            let is_utf8_continuation = (128..=191).contains(&byte);

            if !is_utf8_continuation {
                total_chars += 1;
                current_line_chars += 1;
            }

            // Actual line break detection
            if byte == b'\n' {
                total_lines += 1;
                if current_line_chars - 1 > max_width {
                    max_width = current_line_chars - 1; // Subtract the '\n'
                }
                current_line_chars = 0;
            }

            // State machine
            if byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b'\r' {
                in_word = false;
            } else if !in_word {
                in_word = true;
                total_words += 1;
            }
        }
    }

    // Special handling in case the file does not end on a blank line
    if current_line_chars > 0 && current_line_chars > max_width {
        max_width = current_line_chars;
    }

    // PRINTING SECTION
    let mut output = String::new();

    if args.lines {
        output.push_str(&format!("{} ", total_lines));
    }
    if args.words {
        output.push_str(&format!("{} ", total_words));
    }
    if args.chars {
        output.push_str(&format!("{} ", total_chars));
    }
    if args.bytes {
        output.push_str(&format!("{} ", total_bytes));
    }
    if args.max_line_length {
        output.push_str(&format!("{} ", max_width));
    }

    println!("{}{}", output, args.file_path.display());

    Ok(())
}
