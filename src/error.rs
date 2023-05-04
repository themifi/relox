use super::token::Token;
use std::fmt;
use std::fmt::Write;

pub fn report<T: fmt::Display>(e: T, stderr: &mut dyn Write) {
    writeln!(stderr, "{}", e).unwrap();
}

pub fn format_error<T: AsRef<str>>(line: usize, message: T) -> String {
    format!("[line {}] Error: {}", line, message.as_ref())
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    OperandMustBeANumber { token: Token },
    OperandsMustBeNumbers { token: Token },
    OperandsMustBeTwoNumbersOrTwoStrings { token: Token },
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
        };
        write!(f, "{}", msg)
    }
}
