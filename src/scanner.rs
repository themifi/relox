use std::{collections::HashMap, fmt};

use super::{
    error::format_error,
    token::{Token, TokenType},
};

pub struct Scanner {
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            keywords: keywords(),
        }
    }

    pub fn scan_tokens(&self, source: String) -> Result<Vec<Token>, Error> {
        let mut reader = Reader::new(source);
        let mut tokens = Vec::new();

        while !reader.is_at_end() {
            reader.set_start();
            if let Some(token) = self.scan_token(&mut reader)? {
                tokens.push(token);
            }
        }
        tokens.push(Token {
            t: TokenType::Eof,
            lexeme: String::new(),
            literal: String::new(),
            line: reader.line(),
        });

        Ok(tokens)
    }

    fn scan_token(&self, reader: &mut Reader) -> Result<Option<Token>, Error> {
        let c = reader.advance();
        match c {
            '(' => Ok(Some(Self::token(TokenType::LeftParen, reader))),
            ')' => Ok(Some(Self::token(TokenType::RightParen, reader))),
            '{' => Ok(Some(Self::token(TokenType::LeftBrace, reader))),
            '}' => Ok(Some(Self::token(TokenType::RightBrace, reader))),
            ',' => Ok(Some(Self::token(TokenType::Comma, reader))),
            '.' => Ok(Some(Self::token(TokenType::Dot, reader))),
            '-' => Ok(Some(Self::token(TokenType::Minus, reader))),
            '+' => Ok(Some(Self::token(TokenType::Plus, reader))),
            ';' => Ok(Some(Self::token(TokenType::Semicolon, reader))),
            '*' => Ok(Some(Self::token(TokenType::Star, reader))),
            '!' => {
                let t = if Self::match_char('=', reader) {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                Ok(Some(Self::token(t, reader)))
            }
            '=' => {
                let t = if Self::match_char('=', reader) {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                Ok(Some(Self::token(t, reader)))
            }
            '<' => {
                let t = if Self::match_char('=', reader) {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                Ok(Some(Self::token(t, reader)))
            }
            '>' => {
                let t = if Self::match_char('=', reader) {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                Ok(Some(Self::token(t, reader)))
            }
            '/' => {
                if Self::match_char('/', reader) {
                    while reader.peek() != '\n' && !reader.is_at_end() {
                        reader.advance();
                    }
                    Ok(None)
                } else {
                    Ok(Some(Self::token(TokenType::Slash, reader)))
                }
            }
            ' ' | '\r' | '\t' | '\n' => Ok(None),
            '"' => {
                let token = Self::scan_string(reader)?;
                Ok(Some(token))
            }
            c if is_digit(c) => Ok(Some(Self::scan_number(reader))),
            c if is_alpha(c) => Ok(Some(self.scan_identifier(reader))),
            _ => Err(Error::UnexpectedCharacterError {
                line: reader.line(),
                c,
            }),
        }
    }

    fn token(t: TokenType, reader: &Reader) -> Token {
        Self::literal_token(t, String::new(), reader)
    }

    fn literal_token(t: TokenType, literal: String, reader: &Reader) -> Token {
        let lexeme = reader.lexeme();
        Token {
            line: reader.line(),
            t,
            lexeme,
            literal,
        }
    }

    fn match_char(expected: char, reader: &mut Reader) -> bool {
        if reader.is_at_end() || reader.peek() != expected {
            false
        } else {
            reader.advance();
            true
        }
    }

    fn scan_string(reader: &mut Reader) -> Result<Token, Error> {
        while reader.peek() != '"' && !reader.is_at_end() {
            reader.advance();
        }

        if reader.is_at_end() {
            return Err(Error::UnterminatedStringError {
                line: reader.line(),
            });
        }

        reader.advance();

        let value = reader.lexeme();
        let s = value[1..value.len() - 1].to_owned();
        Ok(Self::literal_token(TokenType::String, s, reader))
    }

    fn scan_number(reader: &mut Reader) -> Token {
        while is_digit(reader.peek()) {
            reader.advance();
        }

        if reader.peek() == '.' && is_digit(reader.peek_next()) {
            reader.advance();

            while is_digit(reader.peek()) {
                reader.advance();
            }
        }

        let literal = reader.lexeme();
        Self::literal_token(TokenType::Number, literal, reader)
    }

    fn scan_identifier(&self, reader: &mut Reader) -> Token {
        while is_alpha_numeric(reader.peek()) {
            reader.advance();
        }

        let literal = reader.lexeme();
        let t = self
            .keywords
            .get(literal.as_str())
            .unwrap_or(&TokenType::Identifier)
            .clone();
        Self::token(t, reader)
    }
}

fn is_digit(c: char) -> bool {
    ('0'..'9').contains(&c)
}

fn is_alpha(c: char) -> bool {
    ('a'..'z').contains(&c) || ('A'..'Z').contains(&c) || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

fn keywords() -> HashMap<&'static str, TokenType> {
    let mut m = HashMap::new();

    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::True);
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);

    m
}

struct Reader {
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Reader {
    fn new(source: String) -> Self {
        let chars = source.chars().collect();
        Self {
            chars,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1;
        if c == '\n' {
            self.line += 1;
        }
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn set_start(&mut self) {
        self.start = self.current;
    }

    fn line(&self) -> usize {
        self.line
    }

    fn lexeme(&self) -> String {
        self.chars[self.start..self.current].iter().collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnterminatedStringError { line: usize },
    UnexpectedCharacterError { line: usize, c: char },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Self::UnterminatedStringError { line } => format_error(line, "unterminated string"),
            Self::UnexpectedCharacterError { line, c } => {
                format_error(line, format!("unexpected character {:?}", c))
            }
        };
        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_comment() {
        let scanner = Scanner::new();
        let source = "// foo".to_owned();
        assert_eq!(
            Ok(vec![Token {
                t: TokenType::Eof,
                line: 1,
                lexeme: String::new(),
                literal: String::new(),
            }]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_parans() {
        let scanner = Scanner::new();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::LeftParen,
                    line: 1,
                    lexeme: "(".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::RightParen,
                    line: 1,
                    lexeme: ")".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 1,
                    lexeme: String::new(),
                    literal: String::new(),
                }
            ]),
            scanner.scan_tokens("()".to_owned())
        );
    }

    #[test]
    fn test_curly_braces() {
        let scanner = Scanner::new();
        let source = "{}".to_owned();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::LeftBrace,
                    line: 1,
                    lexeme: "{".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::RightBrace,
                    line: 1,
                    lexeme: "}".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 1,
                    lexeme: String::new(),
                    literal: String::new(),
                }
            ]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_signs() {
        let scanner = Scanner::new();
        let source = "+-*/".to_owned();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::Plus,
                    line: 1,
                    lexeme: "+".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Minus,
                    line: 1,
                    lexeme: "-".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Star,
                    line: 1,
                    lexeme: "*".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Slash,
                    line: 1,
                    lexeme: "/".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 1,
                    lexeme: String::new(),
                    literal: String::new(),
                }
            ]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_comparators() {
        let scanner = Scanner::new();
        let source = "< <= > >= ! != = ==".to_owned();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::Less,
                    line: 1,
                    lexeme: "<".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::LessEqual,
                    line: 1,
                    lexeme: "<=".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Greater,
                    line: 1,
                    lexeme: ">".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::GreaterEqual,
                    line: 1,
                    lexeme: ">=".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Bang,
                    line: 1,
                    lexeme: "!".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::BangEqual,
                    line: 1,
                    lexeme: "!=".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Equal,
                    line: 1,
                    lexeme: "=".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::EqualEqual,
                    line: 1,
                    lexeme: "==".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 1,
                    lexeme: String::new(),
                    literal: String::new(),
                }
            ]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_punctuation() {
        let scanner = Scanner::new();
        let source = ".,;".to_owned();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::Dot,
                    line: 1,
                    lexeme: ".".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Comma,
                    line: 1,
                    lexeme: ",".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Semicolon,
                    line: 1,
                    lexeme: ";".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 1,
                    lexeme: String::new(),
                    literal: String::new(),
                }
            ]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_string_literal() {
        let scanner = Scanner::new();
        let source = "\"foo\"".to_owned();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::String,
                    line: 1,
                    lexeme: "\"foo\"".to_owned(),
                    literal: "foo".to_owned(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 1,
                    lexeme: String::new(),
                    literal: String::new(),
                }
            ]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_integer_number() {
        let scanner = Scanner::new();
        let source = "123".to_owned();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::Number,
                    line: 1,
                    lexeme: "123".to_owned(),
                    literal: "123".to_owned(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 1,
                    lexeme: String::new(),
                    literal: String::new(),
                }
            ]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_real_number() {
        let scanner = Scanner::new();
        let source = "3.14".to_owned();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::Number,
                    line: 1,
                    lexeme: "3.14".to_owned(),
                    literal: "3.14".to_owned(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 1,
                    lexeme: String::new(),
                    literal: String::new(),
                }
            ]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_identifiers() {
        let scanner = Scanner::new();
        let source = "foo bar".to_owned();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::Identifier,
                    line: 1,
                    lexeme: "foo".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Identifier,
                    line: 1,
                    lexeme: "bar".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 1,
                    lexeme: String::new(),
                    literal: String::new(),
                }
            ]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_keywords() {
        let source = "and
        class
        else
        false
        for
        fun
        if
        nil
        or
        print
        return
        super
        this
        true
        var
        while"
            .to_owned();

        let scanner = Scanner::new();
        assert_eq!(
            Ok(vec![
                Token {
                    t: TokenType::And,
                    line: 1,
                    lexeme: "and".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Class,
                    line: 2,
                    lexeme: "class".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Else,
                    line: 3,
                    lexeme: "else".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::False,
                    line: 4,
                    lexeme: "false".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::For,
                    line: 5,
                    lexeme: "for".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Fun,
                    line: 6,
                    lexeme: "fun".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::If,
                    line: 7,
                    lexeme: "if".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Nil,
                    line: 8,
                    lexeme: "nil".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Or,
                    line: 9,
                    lexeme: "or".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Print,
                    line: 10,
                    lexeme: "print".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Return,
                    line: 11,
                    lexeme: "return".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Super,
                    line: 12,
                    lexeme: "super".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::This,
                    line: 13,
                    lexeme: "this".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::True,
                    line: 14,
                    lexeme: "true".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Var,
                    line: 15,
                    lexeme: "var".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::While,
                    line: 16,
                    lexeme: "while".to_owned(),
                    literal: String::new(),
                },
                Token {
                    t: TokenType::Eof,
                    line: 16,
                    lexeme: String::new(),
                    literal: String::new(),
                },
            ]),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_unexpected_char() {
        let scanner = Scanner::new();
        let source = "?%".to_owned();
        assert_eq!(
            Err(Error::UnexpectedCharacterError { line: 1, c: '?' }),
            scanner.scan_tokens(source)
        );
    }

    #[test]
    fn test_unterminated_string() {
        let scanner = Scanner::new();
        let source = "\"foo".to_owned();
        assert_eq!(
            Err(Error::UnterminatedStringError { line: 1 }),
            scanner.scan_tokens(source)
        );
    }
}
