use super::{
    error::RuntimeError,
    expression::{walk_expr, Expression, Visitor},
    token::{Literal as TokenLiteral, Token, TokenType},
    value::Value,
};

pub struct Interpreter {}

impl Visitor for Interpreter {
    type Result = Result;

    fn visit_literal(&self, value: &TokenLiteral) -> Result {
        match value {
            TokenLiteral::Nil => Ok(Value::Nil),
            TokenLiteral::Boolean(b) => Ok(Value::Boolean(*b)),
            TokenLiteral::Number(num) => Ok(Value::Number(*num)),
            TokenLiteral::String(s) => Ok(Value::String(s.clone())),
            TokenLiteral::Identifier(_s) => todo!(),
        }
    }

    fn visit_grouping(&self, expr: &Expression) -> Result {
        self.evaluate(expr)
    }

    fn visit_unary(&self, operator: &Token, right: &Expression) -> Result {
        let right = self.evaluate(right)?;

        match operator.t {
            TokenType::Minus => {
                check_number_operand(&right, operator)?;
                Ok(Value::Number(-right.unwrap_number()))
            }
            TokenType::Bang => Ok(Value::Boolean(!is_truthy(&right))),
            _ => unreachable!(),
        }
    }

    fn visit_binary(&self, left: &Expression, operator: &Token, right: &Expression) -> Result {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.t {
            TokenType::Plus => {
                if left.is_number() && right.is_number() {
                    Ok(Value::Number(left.unwrap_number() + right.unwrap_number()))
                } else if left.is_string() && right.is_string() {
                    let left = left.unwrap_string();
                    let right = right.unwrap_string();
                    Ok(Value::String(format!("{}{}", left, right)))
                } else {
                    Err(RuntimeError::OperandsMustBeTwoNumbersOrTwoStrings {
                        token: operator.clone(),
                    })
                }
            }
            TokenType::Minus => {
                check_number_operands(&left, &right, operator)?;
                Ok(Value::Number(left.unwrap_number() - right.unwrap_number()))
            }
            TokenType::Slash => {
                check_number_operands(&left, &right, operator)?;
                Ok(Value::Number(left.unwrap_number() / right.unwrap_number()))
            }
            TokenType::Star => {
                check_number_operands(&left, &right, operator)?;
                Ok(Value::Number(left.unwrap_number() * right.unwrap_number()))
            }
            TokenType::Greater => {
                check_number_operands(&left, &right, operator)?;
                Ok(Value::Boolean(left.unwrap_number() > right.unwrap_number()))
            }
            TokenType::GreaterEqual => {
                check_number_operands(&left, &right, operator)?;
                Ok(Value::Boolean(
                    left.unwrap_number() >= right.unwrap_number(),
                ))
            }
            TokenType::Less => {
                check_number_operands(&left, &right, operator)?;
                Ok(Value::Boolean(left.unwrap_number() < right.unwrap_number()))
            }
            TokenType::LessEqual => {
                check_number_operands(&left, &right, operator)?;
                Ok(Value::Boolean(
                    left.unwrap_number() <= right.unwrap_number(),
                ))
            }
            TokenType::EqualEqual => Ok(Value::Boolean(is_equal(&left, &right))),
            TokenType::BangEqual => Ok(Value::Boolean(!is_equal(&left, &right))),
            _ => unreachable!(),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, expr: &Expression) -> Result {
        self.evaluate(expr)
    }

    fn evaluate(&self, expr: &Expression) -> Result {
        walk_expr(expr, self)
    }
}

type Result = std::result::Result<Value, RuntimeError>;

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Boolean(b) => *b,
        _ => true,
    }
}

#[allow(clippy::float_cmp)]
fn is_equal(left: &Value, right: &Value) -> bool {
    match left {
        Value::Nil => right.is_nil(),
        Value::Boolean(b) => right.is_boolean() && *b == right.unwrap_boolean(),
        Value::Number(num) => right.is_number() && *num == right.unwrap_number(),
        Value::String(s) => right.is_string() && s == right.unwrap_string(),
    }
}

fn check_number_operand(
    operand: &Value,
    operator: &Token,
) -> std::result::Result<(), RuntimeError> {
    if operand.is_number() {
        Ok(())
    } else {
        Err(RuntimeError::OperandMustBeANumber {
            token: operator.clone(),
        })
    }
}

fn check_number_operands(
    left: &Value,
    right: &Value,
    operator: &Token,
) -> std::result::Result<(), RuntimeError> {
    if left.is_number() && right.is_number() {
        Ok(())
    } else {
        Err(RuntimeError::OperandsMustBeNumbers {
            token: operator.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interpret(expr: &Expression) -> Result {
        let interpreter = Interpreter::new();
        interpreter.interpret(expr)
    }

    #[test]
    fn interpret_literal() {
        let literals = vec![
            (TokenLiteral::Nil, Value::Nil),
            (TokenLiteral::Boolean(true), Value::Boolean(true)),
            (TokenLiteral::Number(4.0), Value::Number(4.0)),
            (
                TokenLiteral::String("foo".to_owned()),
                Value::String("foo".to_owned()),
            ),
        ];

        for (literal, value) in literals {
            let expr = Expression::Literal { value: literal };
            assert_eq!(Ok(value), interpret(&expr));
        }
    }

    #[test]
    fn interpret_number_negation() {
        let expr = Expression::Unary {
            operator: Token {
                t: TokenType::Minus,
                line: 1,
                lexeme: "-".to_owned(),
                literal: None,
            },
            right: Box::new(Expression::Literal {
                value: TokenLiteral::Number(2.0),
            }),
        };
        assert_eq!(Ok(Value::Number(-2.0)), interpret(&expr));
    }

    #[test]
    fn interpret_bool_negation() {
        let expr = Expression::Unary {
            operator: Token {
                t: TokenType::Bang,
                line: 1,
                lexeme: "!".to_owned(),
                literal: None,
            },
            right: Box::new(Expression::Literal {
                value: TokenLiteral::Boolean(true),
            }),
        };
        assert_eq!(Ok(Value::Boolean(false)), interpret(&expr));
    }

    #[test]
    fn interpret_negation_invalid_type() {
        let literals = vec![
            TokenLiteral::Nil,
            TokenLiteral::String("foo".to_owned()),
            TokenLiteral::Boolean(true),
        ];
        for literal in literals {
            let operator = Token {
                t: TokenType::Minus,
                line: 1,
                lexeme: String::new(),
                literal: None,
            };
            let expr = Expression::Unary {
                operator: operator.clone(),
                right: Box::new(Expression::Literal { value: literal }),
            };
            assert_eq!(
                Err(RuntimeError::OperandMustBeANumber {
                    token: operator.clone(),
                }),
                interpret(&expr)
            );
        }
    }

    #[test]
    fn interpret_bang() {
        let literals = vec![
            (TokenLiteral::Nil, true),
            (TokenLiteral::String("foo".to_owned()), false),
            (TokenLiteral::Number(2.0), false),
            (TokenLiteral::Boolean(true), false),
            (TokenLiteral::Boolean(false), true),
        ];
        for (literal, result) in literals {
            let expr = Expression::Unary {
                operator: Token {
                    t: TokenType::Bang,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Expression::Literal { value: literal }),
            };
            assert_eq!(Ok(Value::Boolean(result)), interpret(&expr));
        }
    }

    #[test]
    fn interpret_grouping() {
        let expr = Expression::Grouping {
            expr: Box::new(Expression::Unary {
                operator: Token {
                    t: TokenType::Bang,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Expression::Literal {
                    value: TokenLiteral::Boolean(true),
                }),
            }),
        };
        assert_eq!(Ok(Value::Boolean(false)), interpret(&expr));
    }

    #[test]
    fn interpret_numbers_operations() {
        let data = vec![
            (TokenType::Plus, 20.0),
            (TokenType::Minus, 10.0),
            (TokenType::Star, 75.0),
            (TokenType::Slash, 3.0),
        ];

        for (token_type, result) in data {
            let expr = Expression::Binary {
                left: Box::new(Expression::Literal {
                    value: TokenLiteral::Number(15.0),
                }),
                operator: Token {
                    t: token_type,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Expression::Literal {
                    value: TokenLiteral::Number(5.0),
                }),
            };
            assert_eq!(Ok(Value::Number(result)), interpret(&expr));
        }
    }

    #[test]
    fn interpret_numbers_operations_with_invalid_operand() {
        let data = vec![
            TokenType::Minus,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];

        for token_type in data {
            let operands = vec![
                (TokenLiteral::Number(15.0), TokenLiteral::Nil),
                (
                    TokenLiteral::Number(15.0),
                    TokenLiteral::String("foo".to_owned()),
                ),
                (TokenLiteral::Number(15.0), TokenLiteral::Boolean(true)),
            ];

            for (left, right) in operands {
                let operator = Token {
                    t: token_type,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                };
                let expr = Expression::Binary {
                    left: Box::new(Expression::Literal { value: left }),
                    operator: operator.clone(),
                    right: Box::new(Expression::Literal { value: right }),
                };
                assert_eq!(
                    Err(RuntimeError::OperandsMustBeNumbers {
                        token: operator.clone()
                    }),
                    interpret(&expr)
                );
            }
        }
    }

    #[test]
    fn interpret_addition_with_invalid_operand() {
        let operands = vec![
            // number with others
            (TokenLiteral::Number(15.0), TokenLiteral::Nil),
            (TokenLiteral::Number(15.0), TokenLiteral::Boolean(true)),
            (
                TokenLiteral::Number(15.0),
                TokenLiteral::String("foo".to_owned()),
            ),
            // string with others
            (
                TokenLiteral::String("foo".to_owned()),
                TokenLiteral::Boolean(true),
            ),
            (TokenLiteral::String("foo".to_owned()), TokenLiteral::Nil),
            (
                TokenLiteral::String("foo".to_owned()),
                TokenLiteral::Number(2.0),
            ),
        ];

        for (left, right) in operands {
            let operator = Token {
                t: TokenType::Plus,
                line: 1,
                lexeme: String::new(),
                literal: None,
            };
            let expr = Expression::Binary {
                left: Box::new(Expression::Literal { value: left }),
                operator: operator.clone(),
                right: Box::new(Expression::Literal { value: right }),
            };
            assert_eq!(
                Err(RuntimeError::OperandsMustBeTwoNumbersOrTwoStrings {
                    token: operator.clone()
                }),
                interpret(&expr)
            );
        }
    }

    #[test]
    fn interpret_numbers_comparsion() {
        let data = vec![
            (TokenType::Greater, 2.0, 3.0, false),
            (TokenType::Greater, 3.0, 3.0, false),
            (TokenType::Greater, 4.0, 3.0, true),
            (TokenType::GreaterEqual, 2.0, 3.0, false),
            (TokenType::GreaterEqual, 3.0, 3.0, true),
            (TokenType::GreaterEqual, 4.0, 3.0, true),
            (TokenType::Less, 2.0, 3.0, true),
            (TokenType::Less, 3.0, 3.0, false),
            (TokenType::Less, 4.0, 3.0, false),
            (TokenType::LessEqual, 2.0, 3.0, true),
            (TokenType::LessEqual, 3.0, 3.0, true),
            (TokenType::LessEqual, 4.0, 3.0, false),
            (TokenType::EqualEqual, 2.0, 3.0, false),
            (TokenType::EqualEqual, 3.0, 3.0, true),
            (TokenType::EqualEqual, 4.0, 3.0, false),
            (TokenType::BangEqual, 2.0, 3.0, true),
            (TokenType::BangEqual, 3.0, 3.0, false),
            (TokenType::BangEqual, 4.0, 3.0, true),
        ];

        for (token_type, left, right, result) in data {
            let expr = Expression::Binary {
                left: Box::new(Expression::Literal {
                    value: TokenLiteral::Number(left),
                }),
                operator: Token {
                    t: token_type,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Expression::Literal {
                    value: TokenLiteral::Number(right),
                }),
            };
            assert_eq!(Ok(Value::Boolean(result)), interpret(&expr));
        }
    }

    #[test]
    fn interpret_strings_addition() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal {
                value: TokenLiteral::String("foo".to_owned()),
            }),
            operator: Token {
                t: TokenType::Plus,
                line: 1,
                lexeme: "+".to_owned(),
                literal: None,
            },
            right: Box::new(Expression::Literal {
                value: TokenLiteral::String("bar".to_owned()),
            }),
        };
        assert_eq!(Ok(Value::String("foobar".to_owned())), interpret(&expr));
    }

    #[test]
    fn interpret_literal_equality() {
        let data = vec![
            // nil with others
            (TokenLiteral::Nil, TokenLiteral::Nil, true),
            (TokenLiteral::Nil, TokenLiteral::Boolean(true), false),
            (TokenLiteral::Nil, TokenLiteral::Number(2.0), false),
            (
                TokenLiteral::Nil,
                TokenLiteral::String("foo".to_owned()),
                false,
            ),
            // number with others
            (TokenLiteral::Number(2.0), TokenLiteral::Number(2.0), true),
            (TokenLiteral::Number(3.0), TokenLiteral::Number(2.0), false),
            (TokenLiteral::Number(2.0), TokenLiteral::Nil, false),
            (
                TokenLiteral::Number(2.0),
                TokenLiteral::Boolean(false),
                false,
            ),
            (
                TokenLiteral::Number(2.0),
                TokenLiteral::String("foo".to_owned()),
                false,
            ),
            (
                TokenLiteral::Boolean(true),
                TokenLiteral::Boolean(true),
                true,
            ),
            (
                TokenLiteral::Number(3.0),
                TokenLiteral::Boolean(true),
                false,
            ),
            // boolean with others
            (
                TokenLiteral::Boolean(true),
                TokenLiteral::Boolean(true),
                true,
            ),
            (
                TokenLiteral::Boolean(true),
                TokenLiteral::Boolean(false),
                false,
            ),
            (TokenLiteral::Boolean(true), TokenLiteral::Nil, false),
            (
                TokenLiteral::Boolean(true),
                TokenLiteral::Number(2.0),
                false,
            ),
            (
                TokenLiteral::Boolean(true),
                TokenLiteral::String("foo".to_owned()),
                false,
            ),
            // string with others
            (
                TokenLiteral::String("foo".to_owned()),
                TokenLiteral::String("foo".to_owned()),
                true,
            ),
            (
                TokenLiteral::String("foo".to_owned()),
                TokenLiteral::String("bar".to_owned()),
                false,
            ),
            (
                TokenLiteral::String("foo".to_owned()),
                TokenLiteral::Nil,
                false,
            ),
            (
                TokenLiteral::String("foo".to_owned()),
                TokenLiteral::Boolean(true),
                false,
            ),
            (
                TokenLiteral::String("foo".to_owned()),
                TokenLiteral::Number(2.0),
                false,
            ),
        ];

        for (left, right, true_result) in data {
            let operator = Token {
                t: TokenType::EqualEqual,
                line: 1,
                lexeme: String::new(),
                literal: None,
            };
            let expr = Expression::Binary {
                left: Box::new(Expression::Literal {
                    value: left.clone(),
                }),
                operator,
                right: Box::new(Expression::Literal {
                    value: right.clone(),
                }),
            };
            assert_eq!(Ok(Value::Boolean(true_result)), interpret(&expr));

            let operator = Token {
                t: TokenType::BangEqual,
                line: 1,
                lexeme: String::new(),
                literal: None,
            };
            let expr = Expression::Binary {
                left: Box::new(Expression::Literal { value: left }),
                operator,
                right: Box::new(Expression::Literal { value: right }),
            };
            assert_eq!(Ok(Value::Boolean(!true_result)), interpret(&expr));
        }
    }
}
