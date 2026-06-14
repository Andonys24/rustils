use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    process,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We collected the arguments.
    let args: Vec<String> = env::args().skip(1).collect();

    // Default values
    let mut lines_to_read: usize = 10;
    let mut file_path: Option<&String> = None;

    // Argument Analysis
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-n" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(num) => {
                            lines_to_read = num;
                            i += 2;
                        }
                        Err(e) => {
                            eprintln!("head: invalid number of lines '{}': {}", args[i + 1], e);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("head: option requires an argument -- 'n'");
                    process::exit(1);
                }
            }
            // Anything that does not begin with '-' will be assumed to be a file path
            _path => {
                file_path = Some(&args[i]);
                i += 1;
            }
        }
    }

    // We validate that an argument exists
    let path = match file_path {
        Some(p) => p,
        None => {
            eprintln!("Error: Please provide a filepath.");
            process::exit(1);
        }
    };

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut line_buffer = String::new();

    for _ in 0..lines_to_read {
        line_buffer.clear();

        if reader.read_line(&mut line_buffer)? == 0 {
            break;
        }

        write!(handle, "{}", line_buffer)?;
    }

    Ok(())
}
