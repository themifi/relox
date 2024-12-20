use super::{error, expression::pretty_print, interpreter, parser, scanner, value::Value};
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
            .interpret(&expression)
            .map_err(|e| e.into())
    }

    pub fn dump_ast(&self, source: String) -> Result<String, Error> {
        let tokens = self.scanner.scan_tokens(source)?;
        let expression = parser::parse(tokens)?;
        Ok(pretty_print(&expression))
    }
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_expression_calculator() {
        let lox = Lox::new();
        let result = lox.run("1 - (2 * 3) < 4 == true".to_string());
        assert_eq!(result, Ok(Value::Boolean(true)));
    }
}
