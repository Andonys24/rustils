use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

/// Print the last 10 lines of each FILE to standard output.
#[derive(Parser)]
#[command(name = "tail", version = "0.1.0")]
struct Cli {
    /// Output the last K lines instead of the last 10
    #[arg(short = 'n', default_value_t = 10, value_name = "LINES")]
    lines: usize,

    /// File to read
    #[arg(required = true, value_name = "FILE")]
    file_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We collected the arguments.
    let args = Cli::parse();

    let mut file = File::open(args.file_path)?;

    // We obtain the total file size
    let file_length = file.metadata()?.len();

    let mut lines_found = 0;
    let mut pos = file_length;

    // Move backward byte by byte from the end of the file

    while pos > 0 && lines_found < args.lines {
        pos -= 1;
        file.seek(SeekFrom::Start(pos))?;

        let mut buffer = [0; 1];
        file.read_exact(&mut buffer)?;

        if buffer[0] == b'\n' && pos != file_length - 1 {
            lines_found += 1;
        }
    }

    if pos > 0 {
        file.seek(SeekFrom::Start(pos + 1))?;
    } else {
        file.seek(SeekFrom::Start(0))?;
    }

    let reader = BufReader::new(file);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in reader.lines() {
        writeln!(handle, "{}", line?)?;
    }

    Ok(())
}
