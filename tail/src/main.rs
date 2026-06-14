use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write},
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
                            eprintln!("tail: invalid number of lines '{}': {}", args[i + 1], e);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("tail: option requires an argument -- 'n'");
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

    let mut file = File::open(path)?;

    // We obtain the total file size
    let file_length = file.metadata()?.len();

    let mut lines_found = 0;
    let mut pos = file_length;

    // Move backward byte by byte from the end of the file

    while pos > 0 && lines_found < lines_to_read {
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
