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
