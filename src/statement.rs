use super::{error::RuntimeError, expression::Expression};
use std::fmt;

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expr: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct Print {
    pub expr: Box<dyn Expression>,
}

pub trait Statement: fmt::Display + fmt::Debug {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), RuntimeError>;
}

pub trait Visitor {
    fn visit_expression_statement(&self, expr: &ExpressionStatement) -> Result<(), RuntimeError>;
    fn visit_print(&mut self, print: &Print) -> Result<(), RuntimeError>;
}

impl Statement for ExpressionStatement {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), RuntimeError> {
        visitor.visit_expression_statement(self)
    }
}

impl Statement for Print {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), RuntimeError> {
        visitor.visit_print(self)
    }
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(expression statement)")
    }
}

impl fmt::Display for Print {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(print statement)")
    }
}
