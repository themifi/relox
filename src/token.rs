use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
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

    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),

            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),

            TokenType::Identifier => write!(f, "identifier"),
            TokenType::String => write!(f, "string"),
            TokenType::Number => write!(f, "number"),

            TokenType::And => write!(f, "and"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::Fun => write!(f, "fun"),
            TokenType::For => write!(f, "for"),
            TokenType::If => write!(f, "if"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Or => write!(f, "or"),
            TokenType::Print => write!(f, "print"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::This => write!(f, "this"),
            TokenType::True => write!(f, "true"),
            TokenType::Var => write!(f, "var"),
            TokenType::While => write!(f, "while"),

            TokenType::Eof => write!(f, "eof"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub t: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.t)?;
        if let Some(literal) = &self.literal {
            write!(f, " {}", literal)?
        } else {
            write!(f, " {}", self.lexeme)?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Identifier(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Literal::Nil => write!(f, "nil"),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Number(num) => write!(f, "{}", num),
            Literal::String(ref s) => write!(f, "{:?}", s),
            Literal::Identifier(ref s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_display() {
        assert_eq!("nil", format!("{}", Literal::Nil));
        assert_eq!("true", format!("{}", Literal::Boolean(true)));
        assert_eq!("false", format!("{}", Literal::Boolean(false)));
        assert_eq!("2", format!("{}", Literal::Number(2.0)));
        assert_eq!("2.4", format!("{}", Literal::Number(2.4)));
        assert_eq!("\"foo\"", format!("{}", Literal::String("foo".to_owned())));
        assert_eq!("foo", format!("{}", Literal::Identifier("foo".to_owned())));
    }

    #[test]
    fn display_number() {
        assert_eq!(
            "number 2.3",
            format!(
                "{}",
                Token {
                    t: TokenType::Number,
                    lexeme: "2.3".to_owned(),
                    literal: Some(Literal::Number(2.3)),
                    line: 1,
                }
            )
        );
    }
}
