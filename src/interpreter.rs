use super::{
    error::RuntimeError,
    expression::{Binary, Expression, Grouping, Literal, Unary, Visitor as ExpressionVisitor},
    statement::{ExpressionStatement, Print, Statement, Visitor as StatementVisitor},
    token::{Literal as TokenLiteral, Token, TokenType},
    value::Value,
};
use std::io;

pub struct Interpreter<'a> {
    output: &'a mut dyn std::io::Write,
}

impl ExpressionVisitor for Interpreter<'_> {
    fn visit_literal(&self, literal: &Literal) -> ValueResult {
        match &literal.value {
            TokenLiteral::Nil => Ok(Value::Nil),
            TokenLiteral::Boolean(b) => Ok(Value::Boolean(*b)),
            TokenLiteral::Number(num) => Ok(Value::Number(*num)),
            TokenLiteral::String(s) => Ok(Value::String(s.clone())),
            TokenLiteral::Identifier(_s) => todo!(),
        }
    }

    fn visit_grouping(&self, grouping: &Grouping) -> ValueResult {
        self.evaluate(grouping.expr.as_ref())
    }

    fn visit_unary(&self, unary: &Unary) -> ValueResult {
        let right = self.evaluate(unary.right.as_ref())?;

        match unary.operator.t {
            TokenType::Minus => {
                check_number_operand(&right, &unary.operator)?;
                Ok(Value::Number(-right.unwrap_number()))
            }
            TokenType::Bang => Ok(Value::Boolean(!is_truthy(&right))),
            _ => unreachable!(),
        }
    }

    fn visit_binary(&self, binary: &Binary) -> ValueResult {
        let left = self.evaluate(binary.left.as_ref())?;
        let right = self.evaluate(binary.right.as_ref())?;

        match binary.operator.t {
            TokenType::Plus => {
                if left.is_number() && right.is_number() {
                    Ok(Value::Number(left.unwrap_number() + right.unwrap_number()))
                } else if left.is_string() && right.is_string() {
                    let left = left.unwrap_string();
                    let right = right.unwrap_string();
                    Ok(Value::String(format!("{}{}", left, right)))
                } else {
                    Err(RuntimeError::OperandsMustBeTwoNumbersOrTwoStrings {
                        token: binary.operator.clone(),
                    })
                }
            }
            TokenType::Minus => {
                check_number_operands(&left, &right, &binary.operator)?;
                Ok(Value::Number(left.unwrap_number() - right.unwrap_number()))
            }
            TokenType::Slash => {
                check_number_operands(&left, &right, &binary.operator)?;
                Ok(Value::Number(left.unwrap_number() / right.unwrap_number()))
            }
            TokenType::Star => {
                check_number_operands(&left, &right, &binary.operator)?;
                Ok(Value::Number(left.unwrap_number() * right.unwrap_number()))
            }
            TokenType::Greater => {
                check_number_operands(&left, &right, &binary.operator)?;
                Ok(Value::Boolean(left.unwrap_number() > right.unwrap_number()))
            }
            TokenType::GreaterEqual => {
                check_number_operands(&left, &right, &binary.operator)?;
                Ok(Value::Boolean(
                    left.unwrap_number() >= right.unwrap_number(),
                ))
            }
            TokenType::Less => {
                check_number_operands(&left, &right, &binary.operator)?;
                Ok(Value::Boolean(left.unwrap_number() < right.unwrap_number()))
            }
            TokenType::LessEqual => {
                check_number_operands(&left, &right, &binary.operator)?;
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

impl StatementVisitor for Interpreter<'_> {
    fn visit_expression_statement(&self, expr: &ExpressionStatement) -> Result {
        self.evaluate(expr.expr.as_ref())?;
        Ok(())
    }

    fn visit_print(&mut self, print: &Print) -> Result {
        let value = self.evaluate(print.expr.as_ref())?;
        writeln!(&mut self.output, "{}", value).unwrap();
        self.output.flush().unwrap();
        Ok(())
    }
}

type ValueResult = std::result::Result<Value, RuntimeError>;
type Result = std::result::Result<(), RuntimeError>;

impl<'a> Interpreter<'a> {
    pub fn new<'b: 'a>(output: &'b mut dyn io::Write) -> Self {
        Self { output }
    }

    pub fn interpret(&mut self, statements: Vec<Box<dyn Statement>>) -> Result {
        for statement in statements {
            self.execute(statement.as_ref())?;
        }
        Ok(())
    }

    fn execute(&mut self, expr: &dyn Statement) -> Result {
        expr.accept(self)
    }

    fn evaluate(&self, expr: &dyn Expression) -> ValueResult {
        expr.accept(self)
    }
}

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

    fn eval(expr: &dyn Expression) -> ValueResult {
        let mut output = io::sink();
        let interpreter = Interpreter::new(&mut output);
        interpreter.evaluate(expr)
    }

    #[test]
    fn eval_literal() {
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
            let expr = Literal { value: literal };
            assert_eq!(Ok(value), eval(&expr));
        }
    }

    #[test]
    fn eval_number_negation() {
        let expr = Unary {
            operator: Token {
                t: TokenType::Minus,
                line: 1,
                lexeme: "-".to_owned(),
                literal: None,
            },
            right: Box::new(Literal {
                value: TokenLiteral::Number(2.0),
            }),
        };
        assert_eq!(Ok(Value::Number(-2.0)), eval(&expr));
    }

    #[test]
    fn eval_bool_negation() {
        let expr = Unary {
            operator: Token {
                t: TokenType::Bang,
                line: 1,
                lexeme: "!".to_owned(),
                literal: None,
            },
            right: Box::new(Literal {
                value: TokenLiteral::Boolean(true),
            }),
        };
        assert_eq!(Ok(Value::Boolean(false)), eval(&expr));
    }

    #[test]
    fn eval_negation_invalid_type() {
        let literals = vec![
            TokenLiteral::Nil,
            TokenLiteral::String("foo".to_owned()),
            TokenLiteral::Boolean(true),
        ];
        for literal in literals {
            let expr = Unary {
                operator: Token {
                    t: TokenType::Minus,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Literal { value: literal }),
            };
            assert_eq!(
                Err(RuntimeError::OperandMustBeANumber {
                    token: expr.operator.clone(),
                }),
                eval(&expr)
            );
        }
    }

    #[test]
    fn eval_bang() {
        let literals = vec![
            (TokenLiteral::Nil, true),
            (TokenLiteral::String("foo".to_owned()), false),
            (TokenLiteral::Number(2.0), false),
            (TokenLiteral::Boolean(true), false),
            (TokenLiteral::Boolean(false), true),
        ];
        for (literal, result) in literals {
            let expr = Unary {
                operator: Token {
                    t: TokenType::Bang,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Literal { value: literal }),
            };
            assert_eq!(Ok(Value::Boolean(result)), eval(&expr));
        }
    }

    #[test]
    fn eval_grouping() {
        let expr = Grouping {
            expr: Box::new(Unary {
                operator: Token {
                    t: TokenType::Bang,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Literal {
                    value: TokenLiteral::Boolean(true),
                }),
            }),
        };
        assert_eq!(Ok(Value::Boolean(false)), eval(&expr));
    }

    #[test]
    fn eval_numbers_operations() {
        let data = vec![
            (TokenType::Plus, 20.0),
            (TokenType::Minus, 10.0),
            (TokenType::Star, 75.0),
            (TokenType::Slash, 3.0),
        ];

        for (token_type, result) in data {
            let expr = Binary {
                left: Box::new(Literal {
                    value: TokenLiteral::Number(15.0),
                }),
                operator: Token {
                    t: token_type,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Literal {
                    value: TokenLiteral::Number(5.0),
                }),
            };
            assert_eq!(Ok(Value::Number(result)), eval(&expr));
        }
    }

    #[test]
    fn eval_numbers_operations_with_invalid_operand() {
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
                let expr = Binary {
                    left: Box::new(Literal { value: left }),
                    operator: Token {
                        t: token_type,
                        line: 1,
                        lexeme: String::new(),
                        literal: None,
                    },
                    right: Box::new(Literal { value: right }),
                };
                assert_eq!(
                    Err(RuntimeError::OperandsMustBeNumbers {
                        token: expr.operator.clone()
                    }),
                    eval(&expr)
                );
            }
        }
    }

    #[test]
    fn eval_addition_with_invalid_operand() {
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
            let expr = Binary {
                left: Box::new(Literal { value: left }),
                operator: Token {
                    t: TokenType::Plus,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Literal { value: right }),
            };
            assert_eq!(
                Err(RuntimeError::OperandsMustBeTwoNumbersOrTwoStrings {
                    token: expr.operator.clone()
                }),
                eval(&expr)
            );
        }
    }

    #[test]
    fn eval_numbers_comparsion() {
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
            let expr = Binary {
                left: Box::new(Literal {
                    value: TokenLiteral::Number(left),
                }),
                operator: Token {
                    t: token_type,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Literal {
                    value: TokenLiteral::Number(right),
                }),
            };
            assert_eq!(Ok(Value::Boolean(result)), eval(&expr));
        }
    }

    #[test]
    fn eval_strings_addition() {
        let expr = Binary {
            left: Box::new(Literal {
                value: TokenLiteral::String("foo".to_owned()),
            }),
            operator: Token {
                t: TokenType::Plus,
                line: 1,
                lexeme: "+".to_owned(),
                literal: None,
            },
            right: Box::new(Literal {
                value: TokenLiteral::String("bar".to_owned()),
            }),
        };
        assert_eq!(Ok(Value::String("foobar".to_owned())), eval(&expr));
    }

    #[test]
    fn eval_literal_equality() {
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
            let mut expr = Binary {
                left: Box::new(Literal { value: left }),
                operator: Token {
                    t: TokenType::EqualEqual,
                    line: 1,
                    lexeme: String::new(),
                    literal: None,
                },
                right: Box::new(Literal { value: right }),
            };
            assert_eq!(Ok(Value::Boolean(true_result)), eval(&expr));

            expr.operator.t = TokenType::BangEqual;
            assert_eq!(Ok(Value::Boolean(!true_result)), eval(&expr));
        }
    }

    #[test]
    fn execute_expression_statement() {
        let mut output = io::sink();
        let mut interpreter = Interpreter::new(&mut output);
        let statement = ExpressionStatement {
            expr: Box::new(Literal {
                value: TokenLiteral::Boolean(true),
            }),
        };
        assert_eq!(Ok(()), interpreter.execute(&statement));
    }

    #[test]
    fn execute_statements() {
        let mut output = io::sink();
        let mut interpreter = Interpreter::new(&mut output);
        let statements: Vec<Box<dyn Statement>> = vec![
            Box::new(ExpressionStatement {
                expr: Box::new(Literal {
                    value: TokenLiteral::Boolean(true),
                }),
            }),
            Box::new(Print {
                expr: Box::new(Literal {
                    value: TokenLiteral::Number(42.0),
                }),
            }),
        ];
        assert_eq!(Ok(()), interpreter.interpret(statements));
    }

    #[test]
    fn execute_print() {
        let mut output = Vec::new();
        let mut interpreter = Interpreter::new(&mut output);
        let statement = Print {
            expr: Box::new(Literal {
                value: TokenLiteral::Boolean(true),
            }),
        };
        assert_eq!(Ok(()), interpreter.execute(&statement));
        let output_string = std::str::from_utf8(&output).unwrap();
        assert_eq!("true\n", output_string);
    }
}
