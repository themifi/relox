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
    let err = run_print_stdout(text);
    if let Some(err) = err {
        match err {
            ExecErrorType::RuntimeError => process::exit(70),
            _ => process::exit(65),
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
    }
}

#[wasm_bindgen]
pub fn run_wasm(source: String) -> String {
    let result = run_with_result(source);
    result.output
}

fn run_print_stdout(source: String) -> Option<ExecErrorType> {
    let result = run_with_result(source);
    println!("{}", result.output);
    result.err
}

fn run_with_result(source: String) -> ExecutionResult {
    let mut output = String::new();
    let err = run_with_output(source, &mut output);
    ExecutionResult { output, err }
}

struct ExecutionResult {
    output: String,
    err: Option<ExecErrorType>,
}

// Execute the source and write to the output.
// Return type of error if there was any.
// The error is already printed in the output.
fn run_with_output(source: String, output: &mut dyn fmt::Write) -> Option<ExecErrorType> {
    let lox = lox::Lox::new();
    match lox.run(source) {
        Ok(value) => {
            writeln!(output, "{}", value).unwrap();
            None
        }
        Err(e) => match e {
            lox::Error::Runtime(e) => {
                error::report(e, output);
                Some(ExecErrorType::RuntimeError)
            }
            _ => {
                error::report(e, output);
                Some(ExecErrorType::GeneralError)
            }
        },
    }
}

enum ExecErrorType {
    RuntimeError,
    GeneralError,
}
