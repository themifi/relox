use relox::{dump_file_ast, run_file, run_prompt};
use std::env;

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        print_help_and_exit();
    }

    let command = args.nth(1).unwrap();
    match command.as_str() {
        "run" => match args.next() {
            None => run_prompt(),
            Some(file) => run_file(file),
        },
        "ast" => {
            let file = args.next().unwrap();
            dump_file_ast(file)
        }
        _ => print_help_and_exit(),
    }
}

fn print_help_and_exit() -> ! {
    println!(
        "Usage: 
    lox run [script]
    lox ast <script>"
    );
    std::process::exit(64);
}
