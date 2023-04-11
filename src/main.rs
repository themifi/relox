use relox::{run_file, run_prompt};
use std::env;

fn main() {
    let mut args = env::args();
    match args.len() {
        n if n > 2 => println!("Usage: lox [script]"),
        2 => {
            let file = args.nth(1).unwrap();
            run_file(file);
        }
        _ => run_prompt(),
    }
}
