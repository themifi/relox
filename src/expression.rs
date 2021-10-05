use super::token::Token;

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

trait Expression {
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

impl Expression for Grouping {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_grouping(&self);
    }
}

impl Expression for Literal {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_literal(&self);
    }
}

impl Expression for Unary {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_unary(&self);
    }
}
