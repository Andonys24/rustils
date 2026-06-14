use clap::Parser;

#[derive(Parser)]
#[command(
    name = "echo",
    about = "Display a line of text",
    arg_required_else_help = false
)]
struct Cli {
    /// Do not output the trailing newline
    #[arg(short = 'n')]
    no_newline: bool,

    /// Enable interpretation of backslash escape
    #[arg(short = 'e')]
    enable_escapes: bool,

    /// The text or words to display
    text: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    let mut final_text = args.text.join(" ");

    if args.enable_escapes {
        final_text = final_text
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\\\", "\\");
    }

    if args.no_newline {
        print!("{}", final_text);
    } else {
        println!("{}", final_text);
    }
}
