use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Error: Please provider a filepath.");
        return;
    }

    let file_path = &args[0];

    match File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            for line_result in reader.lines().take(10) {
                match line_result {
                    Ok(line) => println!("{}", line),
                    Err(e) => {
                        eprintln!("head: error reading linea: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("head: {}: {}", file_path, e);
        }
    }
}
