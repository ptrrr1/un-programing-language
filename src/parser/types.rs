use core::fmt;
use std::fmt::Display;

use crate::tokens::TokenType;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Types {
    Bool,
    Number,
    String,
    Nil,
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Value {
    pub fn as_type(self) -> Types {
        match self {
            Value::Bool(_) => Types::Bool,
            Value::Number(_) => Types::Number,
            Value::String(_) => Types::String,
            Value::Nil => Types::Nil,
        }
    }

    pub fn get_truthyness(&self) -> bool {
        match self {
            Value::Bool(v) => *v,
            Value::Number(v) => !(*v == 0.0),
            Value::String(v) => !v.is_empty(),
            Value::Nil => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (self, other) {
            (Value::Bool(l), Value::Bool(r)) => l.partial_cmp(r),
            (Value::Number(l), Value::Number(r)) => l.partial_cmp(r),
            (Value::String(l), Value::String(r)) => l.partial_cmp(r),
            (Value::Nil, Value::Nil) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(v) => write!(f, "{v}"),
            Value::Number(v) => write!(f, "{v}"),
            Value::String(v) => write!(f, "{v}"),
            Value::Nil => write!(f, "Nil"),
        }
    }
}

impl TryFrom<TokenType> for Value {
    type Error = &'static str;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::True => Ok(Value::Bool(true)),
            TokenType::False => Ok(Value::Bool(false)),
            TokenType::Number(v) => Ok(Value::Number(v)),
            TokenType::String(v) => Ok(Value::String(v)),
            TokenType::Nil => Ok(Value::Nil),
            _ => Err("Failed to convert"),
        }
    }
}
