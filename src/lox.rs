use super::{error, interpreter, parser, scanner, value::Value};
use std::fmt;

pub struct Lox {
    scanner: scanner::Scanner,
    interpreter: interpreter::Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        let scanner = scanner::Scanner::new();
        let interpreter = interpreter::Interpreter::new();
        Lox {
            scanner,
            interpreter,
        }
    }

    pub fn run(&self, source: String) -> Result<Value, Error> {
        let tokens = self.scanner.scan_tokens(source)?;
        let expression = parser::parse(tokens)?;
        self.interpreter
            .interpret(expression.as_ref())
            .map_err(|e| e.into())
    }
}

pub enum Error {
    ScanError(scanner::Error),
    ParseError(parser::Error),
    RuntimeError(error::RuntimeError),
}

impl From<scanner::Error> for Error {
    fn from(error: scanner::Error) -> Self {
        Error::ScanError(error)
    }
}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Self {
        Error::ParseError(error)
    }
}

impl From<error::RuntimeError> for Error {
    fn from(error: error::RuntimeError) -> Self {
        Error::RuntimeError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ScanError(e) => write!(f, "{}", e),
            Self::ParseError(e) => write!(f, "{}", e),
            Self::RuntimeError(e) => write!(f, "{}", e),
        }
    }
}
