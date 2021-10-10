use std::{
    env, fs,
    io::{self, Write},
    process,
};

mod error;
mod expression;
mod parser;
mod scanner;
mod token;

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

fn run_file(file: String) {
    let text = fs::read_to_string(file).expect("file read failed");
    run(text);
    unsafe {
        if error::HAD_ERROR {
            process::exit(65);
        }
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = stdin.read_line(&mut input).expect("read line failed");
        let eof = bytes_read == 0;
        if eof {
            break;
        }

        run(input);

        unsafe {
            error::HAD_ERROR = false;
        }
    }
}

fn run(source: String) {
    let scanner = scanner::Scanner::new();
    let result = scanner.scan_tokens(source);
    if let Err(e) = result {
        error::report(e);
        return;
    }
    let tokens = result.unwrap();

    match parser::parse(tokens) {
        Ok(expression) => println!("{}", expression),
        Err(err) => error::report(err),
    };
}
