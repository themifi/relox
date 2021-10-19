use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(num) => write!(f, "{}", num),
            Value::String(ref s) => write!(f, "{:?}", s),
        }
    }
}

impl Value {
    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_boolean(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => panic!("unwrapping a value failed: value is {}", self),
        }
    }

    pub fn unwrap_number(&self) -> f64 {
        match self {
            Value::Number(num) => *num,
            _ => panic!("unwrapping a value failed: value is {}", self),
        }
    }

    pub fn unwrap_string(&self) -> &str {
        match self {
            Value::String(s) => s,
            _ => panic!("unwrapping a value failed: value is {}", self),
        }
    }
}
