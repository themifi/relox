use std::{env, fs, io, process};

mod lox;

fn main() {
    let args = env::args();
    if args.len() > 2 {
        println!("Usage: lox [script]");
    } else if args.len() == 2 {
        let file = args.skip(1).next().unwrap();
        run_file(file);
    } else {
        run_prompt();
    }
}

fn run_file(file: String) {
    let text  = fs::read_to_string(file).expect("file read failed");
    run(text.as_str());
    unsafe {
        if HAD_ERROR {
            process::exit(65);
        }
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("> ");
 
        let bytes_read = stdin.read_line(&mut input).expect("read line failed");
        let eof = bytes_read == 0;
        if eof {
            break;
        }

        run(input.as_ref());  

        input.clear();
        unsafe {
            HAD_ERROR = false;
        }
    }
}

fn run(source: &str) {
    let scanner = Scanner::new(source);
    let tokens = scanner.tokens();
    for token in tokens {
        println!("{}", token);
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, place: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, place, message);
    unsafe {
        HAD_ERROR = true;
    }
}

static mut HAD_ERROR: bool = false;

struct Scanner{}

impl Scanner {
    fn new(source: &str) -> Self {
        todo!();
    }

    fn tokens(&self) -> Vec<lox::Token> {
        todo!();
    }
}
