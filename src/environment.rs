use super::{error::RuntimeError, token::Token, value::Value};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &Token, value: Value) {
        let name = unwrap_identifier(name);
        self.values.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &Token) -> Result<&Value, RuntimeError> {
        let str_name = unwrap_identifier(name);
        match self.values.get(str_name) {
            Some(v) => Ok(v),
            None => Err(RuntimeError::UndefinedVariable {
                token: name.clone(),
            }),
        }
    }
}

fn unwrap_identifier(t: &Token) -> &str {
    t.literal.as_ref().unwrap().unwrap_identifier()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Literal, TokenType};

    #[test]
    fn can_define_var() {
        let mut env = Environment::new();
        let val = Value::Number(2.0);
        let t = Token {
            t: TokenType::Identifier,
            lexeme: "foo".to_string(),
            literal: Some(Literal::Identifier("foo".to_string())),
            line: 1,
        };

        env.define(&t, val.clone());

        assert_eq!(Ok(&val), env.get(&t));
    }

    #[test]
    fn can_get_undefined_var() {
        let env = Environment::new();
        let t = Token {
            t: TokenType::Identifier,
            lexeme: "foo".to_string(),
            literal: Some(Literal::Identifier("foo".to_string())),
            line: 1,
        };

        assert_eq!(
            Err(RuntimeError::UndefinedVariable { token: t.clone() }),
            env.get(&t)
        );
    }
}
