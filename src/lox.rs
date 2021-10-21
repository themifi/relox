use super::{error, interpreter, parser, scanner};
use std::{fmt, io};

pub struct Lox {
    scanner: scanner::Scanner,
}

impl Lox {
    pub fn new() -> Self {
        let scanner = scanner::Scanner::new();
        Lox { scanner }
    }

    pub fn run(&mut self, source: String) {
        let result = self.scanner.scan_tokens(source);
        if let Err(e) = result {
            error::report(e);
            return;
        }
        let tokens = result.unwrap();

        let result = parser::parse(tokens);
        if let Err(e) = result {
            error::report(e);
            return;
        }
        let statements = result.unwrap();

        let mut output = io::stdout();
        let mut interpreter = interpreter::Interpreter::new(&mut output);
        let result = interpreter.interpret(statements);
        if let Err(e) = result {
            error::runtime_error(e);
        }
    }
}

pub enum Error {
    Scan(scanner::Error),
    Parse(parser::Error),
    Runtime(error::RuntimeError),
}

impl From<scanner::Error> for Error {
    fn from(error: scanner::Error) -> Self {
        Error::Scan(error)
    }
}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Self {
        Error::Parse(error)
    }
}

impl From<error::RuntimeError> for Error {
    fn from(error: error::RuntimeError) -> Self {
        Error::Runtime(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Scan(e) => write!(f, "{}", e),
            Self::Parse(e) => write!(f, "{}", e),
            Self::Runtime(e) => write!(f, "{}", e),
        }
    }
}
