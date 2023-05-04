use std::{
    fmt, fs,
    io::{self, Write},
    process,
};
use wasm_bindgen::prelude::*;

mod error;
mod expression;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod token;
mod value;

pub fn run_file(file: String) {
    let text = fs::read_to_string(file).expect("file read failed");
    run_print_stdout(text);
    unsafe {
        if error::HAD_ERROR {
            process::exit(65);
        } else if error::HAD_RUNTIME_ERROR {
            process::exit(70);
        }
    }
}

pub fn run_prompt() {
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

        run_print_stdout(input);

        unsafe {
            error::HAD_ERROR = false;
        }
    }
}

#[wasm_bindgen]
pub fn run_with_string_output(source: String) -> String {
    let mut output = String::new();
    run_with_output(source, &mut output);
    output
}

fn run_print_stdout(source: String) {
    let output = run_with_string_output(source);
    println!("{}", output);
}

fn run_with_output(source: String, output: &mut dyn fmt::Write) {
    let lox = lox::Lox::new();
    let result = lox.run(source);
    if let Err(e) = result {
        match e {
            lox::Error::Runtime(e) => error::runtime_error(e, output),
            _ => error::report(e, output),
        }
    }
}
