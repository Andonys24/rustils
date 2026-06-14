use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use clap::Parser;

/// Concatenate FILE(s) to standart output
#[derive(Parser)]
#[command(name = "cat", version = "0.1.0")]
struct Cli {
    /// Number all output lines
    #[arg(short = 'n')]
    number: bool,

    /// Number nonempty output lines, overrides -n
    #[arg(short = 'b')]
    number_nonempty: bool,

    /// Display $ at end of each line
    #[arg(short = 'E', long = "show-ends")]
    show_ends: bool,

    /// Files to concatenate and display
    #[arg(required = true, value_name = "FILE")]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for file_path in &args.files {
        match File::open(file_path) {
            Ok(mut file) => {
                if let Err(e) = io::copy(&mut file, &mut handle) {
                    eprintln!(
                        "cat: error writing {} to stdout: {}",
                        file_path.display(),
                        e
                    );
                }
            }
            Err(e) => {
                eprintln!("cat: {}: {}", file_path.display(), e);
            }
        }
    }

    handle.flush()?;
    Ok(())
}
