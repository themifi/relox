use super::{token::Literal as TokenLiteral, token::Token};
use std::fmt;

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expr: Box<Expression>,
    },
    Literal {
        value: TokenLiteral,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
}

pub fn walk_expr<V: Visitor>(expr: &Expression, v: &V) -> V::Result {
    match expr {
        Expression::Binary {
            left,
            operator,
            right,
        } => v.visit_binary(left, operator, right),
        Expression::Grouping { expr } => v.visit_grouping(expr),
        Expression::Literal { value } => v.visit_literal(value),
        Expression::Unary { operator, right } => v.visit_unary(operator, right),
    }
}

pub trait Visitor {
    type Result;

    fn visit_binary(&self, left: &Expression, operator: &Token, right: &Expression)
        -> Self::Result;
    fn visit_grouping(&self, expr: &Expression) -> Self::Result;
    fn visit_literal(&self, value: &TokenLiteral) -> Self::Result;
    fn visit_unary(&self, operator: &Token, right: &Expression) -> Self::Result;
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator.t, left, right),
            Expression::Grouping { expr } => write!(f, "(group {})", expr.as_ref()),
            Expression::Literal { value } => write!(f, "{}", value),
            Expression::Unary { operator, right } => write!(f, "({} {})", operator.t, right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::token::TokenType;
    use super::*;

    #[test]
    fn test_format_binary() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal {
                value: TokenLiteral::Number(2.0),
            }),
            operator: Token {
                t: TokenType::Plus,
                lexeme: "+".to_owned(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expression::Literal {
                value: TokenLiteral::Number(4.0),
            }),
        };
        assert_eq!(r"(+ 2 4)", format!("{}", expr));
    }

    #[test]
    fn test_format_grouping() {
        let expr = Expression::Grouping {
            expr: Box::new(Expression::Literal {
                value: TokenLiteral::Number(2.0),
            }),
        };
        assert_eq!(r"(group 2)", format!("{}", expr));
    }

    #[test]
    fn test_format_literal() {
        let expr = Expression::Literal {
            value: TokenLiteral::Identifier("foo".to_owned()),
        };
        assert_eq!("foo", format!("{}", expr));
    }

    #[test]
    fn test_format_unary() {
        let expr = Expression::Unary {
            operator: Token {
                t: TokenType::Minus,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expression::Literal {
                value: TokenLiteral::Number(2.0),
            }),
        };
        assert_eq!("(- 2)", format!("{}", expr));
    }

    #[test]
    fn test_format_composite_expression() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Unary {
                operator: Token {
                    t: TokenType::Minus,
                    lexeme: String::new(),
                    literal: None,
                    line: 1,
                },
                right: Box::new(Expression::Literal {
                    value: TokenLiteral::Number(123.0),
                }),
            }),
            operator: Token {
                t: TokenType::Star,
                lexeme: "*".to_owned(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expression::Grouping {
                expr: Box::new(Expression::Literal {
                    value: TokenLiteral::Number(45.67),
                }),
            }),
        };
        assert_eq!(r#"(* (- 123) (group 45.67))"#, format!("{}", expr));
    }
}
