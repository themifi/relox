use super::{
    error::{runtime_error, RuntimeError},
    expression::{Binary, Expression, Grouping, Literal, Unary, Visitor},
    token::{Literal as TokenLiteral, Token, TokenType},
    value::Value,
};

pub struct Interpreter {}

impl Visitor for Interpreter {
    fn visit_literal(&self, literal: &Literal) -> Result<Value, RuntimeError> {
        match &literal.value {
            TokenLiteral::Nil => Ok(Value::Nil),
            TokenLiteral::Boolean(b) => Ok(Value::Boolean(*b)),
            TokenLiteral::Number(num) => Ok(Value::Number(*num)),
            TokenLiteral::String(s) => Ok(Value::String(s.clone())),
            TokenLiteral::Identifier(_s) => todo!(),
        }
    }

    fn visit_grouping(&self, grouping: &Grouping) -> Result<Value, RuntimeError> {
        self.evaluate(grouping.expr.as_ref())
    }

    fn visit_unary(&self, unary: &Unary) -> Result<Value, RuntimeError> {
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

    fn visit_binary(&self, binary: &Binary) -> Result<Value, RuntimeError> {
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

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, expr: &dyn Expression) {
        let interpreter = Interpreter {};
        match interpreter.evaluate(expr) {
            Ok(value) => println!("{}", value),
            Err(e) => runtime_error(e),
        }
    }

    fn evaluate(&self, expr: &dyn Expression) -> Result<Value, RuntimeError> {
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

fn is_equal(left: &Value, right: &Value) -> bool {
    match left {
        Value::Nil => right.is_nil(),
        Value::Boolean(b) => right.is_boolean() && *b == right.unwrap_boolean(),
        Value::Number(num) => right.is_number() && *num == right.unwrap_number(),
        Value::String(s) => right.is_string() && s == right.unwrap_string(),
    }
}

fn check_number_operand(operand: &Value, operator: &Token) -> Result<(), RuntimeError> {
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
) -> Result<(), RuntimeError> {
    if left.is_number() && right.is_number() {
        Ok(())
    } else {
        Err(RuntimeError::OperandsMustBeNumbers {
            token: operator.clone(),
        })
    }
}
