use super::lox::{error, Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
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
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        self.tokens.clear();

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            t: TokenType::EOF,
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
            _ => error(self.line, format!("unexpected character {:?}", c)),
        };
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, t: TokenType) {
        self.add_literal_token(t, String::new())
    }

    fn add_literal_token(&mut self, t: TokenType, literal: String) {
        let lexeme = self.source[self.start..self.current].into_iter().collect();
        self.tokens.push(Token {
            line: self.line,
            t,
            lexeme,
            literal,
        })
    }
}
