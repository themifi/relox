use super::lox::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = source.chars().collect();
        Scanner {
            source: chars,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        tokens.push(Token {
            t: TokenType::EOF,
            lexeme: String::new(),
            literal: String::new(),
            line: self.line,
        });
        tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&self) {
        todo!();
    }
}
