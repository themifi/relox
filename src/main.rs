use std::{
    env, fs,
    io::{self, Write},
    process,
};

mod lox;
mod scanner;

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
        if lox::HAD_ERROR {
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
            lox::HAD_ERROR = false;
        }
    }
}

fn run(source: String) {
    let scanner = scanner::Scanner::new();
    let tokens = scanner.scan_tokens(source);
    for token in tokens {
        println!("{}", token);
    }
}
