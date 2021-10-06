use super::token::Token;
use std::fmt;

struct Binary {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>,
}

struct Grouping {
    expr: Box<dyn Expression>,
}

struct Literal {
    value: String,
}

struct Unary {
    operator: Token,
    right: Box<dyn Expression>,
}

trait Expression: fmt::Display {
    fn accept(&self, visitor: &dyn Visitor);
}

trait Visitor {
    fn visit_binary(&self, binary: &Binary);
    fn visit_grouping(&self, grouping: &Grouping);
    fn visit_literal(&self, literal: &Literal);
    fn visit_unary(&self, unary: &Unary);
}

impl Expression for Binary {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_binary(&self);
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

impl Expression for Grouping {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_grouping(&self);
    }
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(group {})", self.expr)
    }
}

impl Expression for Literal {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_literal(&self);
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Expression for Unary {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_unary(&self);
    }
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.operator.lexeme, self.right)
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
                value: "2".to_owned(),
            }),
            operator: Token {
                t: TokenType::Plus,
                lexeme: "+".to_owned(),
                literal: "+".to_owned(),
                line: 1,
            },
            right: Box::new(Literal {
                value: "4".to_owned(),
            }),
        };
        assert_eq!(r"(+ 2 4)", format!("{}", expr));
    }

    #[test]
    fn test_format_grouping() {
        let expr = Grouping {
            expr: Box::new(Literal {
                value: "2".to_owned(),
            }),
        };
        assert_eq!(r"(group 2)", format!("{}", expr));
    }

    #[test]
    fn test_format_literal() {
        let expr = Literal {
            value: "foo".to_owned(),
        };
        assert_eq!("foo", format!("{}", expr));
    }

    #[test]
    fn test_format_unary() {
        let expr = Unary {
            operator: Token {
                t: TokenType::Plus,
                lexeme: "-".to_owned(),
                literal: "-".to_owned(),
                line: 1,
            },
            right: Box::new(Literal {
                value: "2".to_owned(),
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
                    lexeme: "-".to_owned(),
                    literal: "-".to_owned(),
                    line: 1,
                },
                right: Box::new(Literal {
                    value: "123".to_owned(),
                }),
            }),
            operator: Token {
                t: TokenType::Star,
                lexeme: "*".to_owned(),
                literal: "*".to_owned(),
                line: 1,
            },
            right: Box::new(Grouping {
                expr: Box::new(Literal {
                    value: "45.67".to_owned(),
                }),
            }),
        };
        assert_eq!(r#"(* (- 123) (group 45.67))"#, format!("{}", expr));
    }
}
