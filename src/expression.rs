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

trait Expression {}
