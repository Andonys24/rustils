use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    process,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let file_path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("Error: Please provide a filepath.");
            process::exit(1);
        }
    };
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in reader.lines().take(10) {
        writeln!(handle, "{}", line?)?;
    }

    Ok(())
}
