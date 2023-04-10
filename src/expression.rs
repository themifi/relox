use crate::error::RuntimeError;

use super::{token::Literal as TokenLiteral, token::Token, value::Value};
use std::fmt;

#[derive(Debug)]
pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct Literal {
    pub value: TokenLiteral,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct Variable {
    pub name: Token,
}

#[derive(Debug)]
pub struct Assign {
    pub name: Token,
    pub value: Box<dyn Expression>,
}

pub trait Expression: fmt::Display + fmt::Debug {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<Value, RuntimeError>;

    fn is_variable(&self) -> bool {
        false
    }

    fn unwrap_variable(&self) -> &Variable {
        panic!("called unwrap_variable on non variable expression")
    }
}

pub trait Visitor {
    fn visit_binary(&mut self, binary: &Binary) -> Result<Value, RuntimeError>;
    fn visit_grouping(&mut self, grouping: &Grouping) -> Result<Value, RuntimeError>;
    fn visit_literal(&mut self, literal: &Literal) -> Result<Value, RuntimeError>;
    fn visit_unary(&mut self, unary: &Unary) -> Result<Value, RuntimeError>;
    fn visit_variable(&mut self, var: &Variable) -> Result<Value, RuntimeError>;
    fn visit_assign(&mut self, assign: &Assign) -> Result<Value, RuntimeError>;
}

impl Expression for Binary {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<Value, RuntimeError> {
        visitor.visit_binary(self)
    }
}

impl Expression for Grouping {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<Value, RuntimeError> {
        visitor.visit_grouping(self)
    }
}

impl Expression for Literal {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<Value, RuntimeError> {
        visitor.visit_literal(self)
    }
}

impl Expression for Unary {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<Value, RuntimeError> {
        visitor.visit_unary(self)
    }
}

impl Expression for Variable {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<Value, RuntimeError> {
        visitor.visit_variable(self)
    }

    fn is_variable(&self) -> bool {
        true
    }

    fn unwrap_variable(&self) -> &Variable {
        self
    }
}

impl Expression for Assign {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<Value, RuntimeError> {
        visitor.visit_assign(self)
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.operator.t, self.left, self.right)
    }
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(group {})", self.expr)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.operator.t, self.right)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(var {})", self.name.lexeme)
    }
}

impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(assign {} = {})", self.name.lexeme, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::super::token::TokenType;
    use super::*;

    #[test]
    fn test_format_binary() {
        let expr = Binary {
            left: Box::new(Literal {
                value: TokenLiteral::Number(2.0),
            }),
            operator: Token {
                t: TokenType::Plus,
                lexeme: "+".to_owned(),
                literal: None,
                line: 1,
            },
            right: Box::new(Literal {
                value: TokenLiteral::Number(4.0),
            }),
        };
        assert_eq!(r"(+ 2 4)", format!("{}", expr));
    }

    #[test]
    fn test_format_grouping() {
        let expr = Grouping {
            expr: Box::new(Literal {
                value: TokenLiteral::Number(2.0),
            }),
        };
        assert_eq!(r"(group 2)", format!("{}", expr));
    }

    #[test]
    fn test_format_literal() {
        let expr = Literal {
            value: TokenLiteral::Identifier("foo".to_owned()),
        };
        assert_eq!("(var foo)", format!("{}", expr));
    }

    #[test]
    fn test_format_unary() {
        let expr = Unary {
            operator: Token {
                t: TokenType::Minus,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            right: Box::new(Literal {
                value: TokenLiteral::Number(2.0),
            }),
        };
        assert_eq!("(- 2)", format!("{}", expr));
    }

    #[test]
    fn test_format_composite_expression() {
        let expr = Binary {
            left: Box::new(Unary {
                operator: Token {
                    t: TokenType::Minus,
                    lexeme: String::new(),
                    literal: None,
                    line: 1,
                },
                right: Box::new(Literal {
                    value: TokenLiteral::Number(123.0),
                }),
            }),
            operator: Token {
                t: TokenType::Star,
                lexeme: "*".to_owned(),
                literal: None,
                line: 1,
            },
            right: Box::new(Grouping {
                expr: Box::new(Literal {
                    value: TokenLiteral::Number(45.67),
                }),
            }),
        };
        assert_eq!(r#"(* (- 123) (group 45.67))"#, format!("{}", expr));
    }
}
