use std::collections::HashMap;

use super::lox::{error, Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = source.chars().collect();
        Scanner {
            source: chars,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            keywords: keywords(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        self.tokens.clear();

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            t: TokenType::Eof,
            lexeme: String::new(),
            literal: String::new(),
            line: self.line,
        });

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let t = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t)
            }
            '=' => {
                let t = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t)
            }
            '<' => {
                let t = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t)
            }
            '>' => {
                let t = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t)
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.scan_string(),
            c if is_digit(c) => self.scan_number(),
            c if is_alpha(c) => self.scan_identifier(),
            _ => error(self.line, format!("unexpected character {:?}", c)),
        };
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn add_token(&mut self, t: TokenType) {
        self.add_literal_token(t, String::new())
    }

    fn add_literal_token(&mut self, t: TokenType, literal: String) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            line: self.line,
            t,
            lexeme,
            literal,
        })
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "unterminated string");
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        let s = value.iter().collect();
        self.add_literal_token(TokenType::String, s);
    }

    fn scan_number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let literal = self.source[self.start..self.current].iter().collect();
        self.add_literal_token(TokenType::Number, literal);
    }

    fn scan_identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let literal: String = self.source[self.start..self.current].iter().collect();
        let t = self
            .keywords
            .get(literal.as_str())
            .unwrap_or(&TokenType::Identifier)
            .clone();
        self.add_token(t);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_comment() {
        let mut scanner = Scanner::new("// foo".to_owned());
        assert_eq!(
            vec![Token {
                t: TokenType::Eof,
                line: 1,
                lexeme: String::new(),
                literal: String::new(),
            }],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_parans() {
        let mut scanner = Scanner::new("()".to_owned());
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_curly_braces() {
        let mut scanner = Scanner::new("{}".to_owned());
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_signs() {
        let mut scanner = Scanner::new("+-*/".to_owned());
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_comparators() {
        let mut scanner = Scanner::new("< <= > >= ! != = ==".to_owned());
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_punctuation() {
        let mut scanner = Scanner::new(".,;".to_owned());
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_string_literal() {
        let mut scanner = Scanner::new("\"foo\"".to_owned());
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_integer_number() {
        let mut scanner = Scanner::new("123".to_owned());
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_real_number() {
        let mut scanner = Scanner::new("3.14".to_owned());
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_identifiers() {
        let mut scanner = Scanner::new("foo bar".to_owned());
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }

    #[test]
    fn test_keywords() {
        let text = "and
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

        let mut scanner = Scanner::new(text);
        assert_eq!(
            vec![
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
            ],
            scanner.scan_tokens()
        );
    }
}
