use super::token::Token;
use std::fmt;

pub fn report<T: fmt::Display>(e: T) {
    eprintln!("{}", e);
    unsafe {
        HAD_ERROR = true;
    }
}

pub fn runtime_error<T: fmt::Display>(e: T) {
    eprintln!("{}", e);
    unsafe {
        HAD_RUNTIME_ERROR = true;
    }
}

pub static mut HAD_ERROR: bool = false;
pub static mut HAD_RUNTIME_ERROR: bool = false;

pub fn format_error<T: AsRef<str>>(line: usize, message: T) -> String {
    format!("[line {}] Error: {}", line, message.as_ref())
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    OperandMustBeANumber { token: Token },
    OperandsMustBeNumbers { token: Token },
    OperandsMustBeTwoNumbersOrTwoStrings { token: Token },
    UndefinedVariable { token: Token },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::OperandMustBeANumber { token } => {
                format_error(token.line, "operand must be a number")
            }
            Self::OperandsMustBeNumbers { token } => {
                format_error(token.line, "operands must be numbers")
            }
            Self::OperandsMustBeTwoNumbersOrTwoStrings { token } => {
                format_error(token.line, "operands must be two numbers or two strings")
            }
            Self::UndefinedVariable { token } => format_error(token.line, "undefined variable"),
        };
        write!(f, "{}", msg)
    }
}
