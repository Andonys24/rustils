use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

use clap::Parser;

/// Print the first 10 lines of each FILE to standard output.
#[derive(Parser)]
#[command(name = "head", version = "0.1.0")]
struct Cli {
    /// Print the first K lines instead of the first 10
    #[arg(short = 'n', default_value_t = 10, value_name = "LINES")]
    lines: usize,

    /// File to read
    #[arg(required = true, value_name = "FILE")]
    file_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We collected the arguments.
    let args = Cli::parse();

    let file = File::open(args.file_path)?;
    let mut reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut line_buffer = String::new();

    for _ in 0..args.lines {
        line_buffer.clear();

        if reader.read_line(&mut line_buffer)? == 0 {
            break;
        }

        write!(handle, "{}", line_buffer)?;
    }

    Ok(())
}
