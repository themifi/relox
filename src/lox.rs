use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub t: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.t, self.lexeme, self.literal)
    }
}

pub fn error<T: AsRef<str>>(line: usize, message: T) {
    report(line, "", message);
}

pub fn report<T: AsRef<str>>(line: usize, place: &str, message: T) {
    eprintln!("[line {}] Error{}: {}", line, place, message.as_ref());
    unsafe {
        HAD_ERROR = true;
    }
}

pub static mut HAD_ERROR: bool = false;
