use std::{
    env,
    fs::File,
    io::{self, Write},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Error: Please provider a file path.");
        return;
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for file_path in &args {
        match File::open(file_path) {
            Ok(mut file) => {
                if let Err(e) = io::copy(&mut file, &mut handle) {
                    eprintln!("cat: error writing {} to stdout: {}", file_path, e);
                }
            }
            Err(e) => {
                eprintln!("cat: {}: {}", file_path, e);
            }
        }
    }
    let _ = handle.flush();
}
