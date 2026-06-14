use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let user_args: &[String] = &args[1..];
    let has_flag_n: bool = user_args.contains(&String::from("-n"));
    let has_flag_e: bool = user_args.contains(&String::from("-e"));

    let words_text: Vec<&String> = user_args
        .iter()
        .filter(|arg| arg.as_str() != "-n" && arg.as_str() != "-e")
        .collect();

    let mut final_text: String = words_text
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    if has_flag_e {
        final_text = final_text
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\\\", "\\");
    }

    if has_flag_n {
        print!("{}", final_text);
    } else {
        println!("{}", final_text);
    }
}
