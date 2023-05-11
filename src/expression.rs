use super::{token::Literal as TokenLiteral, token::Token};
use std::fmt::{self, Write};

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

pub fn pretty_print(expr: &Expression) -> String {
    walk_expr(expr, &AstPrinter {})
}

struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: &[&Expression]) -> <AstPrinter as Visitor>::Result {
        let mut s = String::new();

        write!(&mut s, "({}", name).unwrap();
        for expr in exprs {
            write!(&mut s, " {}", walk_expr(expr, self)).unwrap();
        }
        write!(&mut s, ")").unwrap();

        s
    }
}

impl Visitor for AstPrinter {
    type Result = String;

    fn visit_binary(
        &self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Self::Result {
        self.parenthesize(operator.lexeme.as_str(), vec![left, right].as_slice())
    }

    fn visit_grouping(&self, expr: &Expression) -> Self::Result {
        self.parenthesize("group", vec![expr].as_slice())
    }

    fn visit_literal(&self, value: &TokenLiteral) -> Self::Result {
        value.to_string()
    }

    fn visit_unary(&self, operator: &Token, right: &Expression) -> Self::Result {
        self.parenthesize(operator.lexeme.as_str(), vec![right].as_slice())
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
                    lexeme: "-".to_owned(),
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

    #[test]
    fn test_pretty_print() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Unary {
                operator: Token {
                    t: TokenType::Minus,
                    lexeme: "-".to_owned(),
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
        assert_eq!("(* (- 123) (group 45.67))", pretty_print(&expr));
    }
}
