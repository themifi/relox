use super::{error::RuntimeError, token::Token, value::Value};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: &Token, value: Value) {
        let name = unwrap_identifier(name);
        self.values.insert(name.to_owned(), value);
    }

    pub fn assign(&mut self, token: &Token, value: Value) -> Result<(), RuntimeError> {
        let name = unwrap_identifier(token);
        if self.values.contains_key(name) {
            self.values.insert(name.to_owned(), value);
            Ok(())
        } else {
            if let Some(env) = &mut self.enclosing {
                env.assign(token, value)
            } else {
                Err(RuntimeError::UndefinedVariable {
                    token: token.clone(),
                })
            }
        }
    }

    pub fn get(&self, name: &Token) -> Result<&Value, RuntimeError> {
        let str_name = unwrap_identifier(name);
        match self.values.get(str_name) {
            Some(v) => Ok(v),
            None => {
                if let Some(env) = &self.enclosing {
                    env.get(name)
                } else {
                    Err(RuntimeError::UndefinedVariable {
                        token: name.clone(),
                    })
                }
            }
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

    #[test]
    fn get_var_in_nested_env() {
        let mut enclosing = Environment::new();
        let t = Token {
            t: TokenType::Identifier,
            lexeme: "foo".to_string(),
            literal: Some(Literal::Identifier("foo".to_string())),
            line: 1,
        };
        enclosing.define(&t, Value::Number(2.0));

        let global = Environment::new_with_enclosing(enclosing);

        assert_eq!(Ok(&Value::Number(2.0)), global.get(&t));
    }

    #[test]
    fn get_assign_in_nested_env() {
        let mut enclosing = Environment::new();
        let t = Token {
            t: TokenType::Identifier,
            lexeme: "foo".to_string(),
            literal: Some(Literal::Identifier("foo".to_string())),
            line: 1,
        };
        enclosing.define(&t, Value::Number(2.0));

        let mut global = Environment::new_with_enclosing(enclosing);

        assert_eq!(Ok(()), global.assign(&t, Value::Number(3.0)));
        assert_eq!(Ok(&Value::Number(3.0)), global.get(&t));
    }
}
