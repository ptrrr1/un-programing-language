use core::fmt;
use std::fmt::Display;

use crate::tokens::TokenType;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Types {
    Bool,
    Float,
    String,
    Nil,
}

#[derive(Debug, Clone)]
pub enum TValues {
    Bool(bool),
    Float(f64),
    String(String),
    Nil,
}

impl TValues {
    pub fn as_type(self) -> Types {
        match self {
            TValues::Bool(_) => Types::Bool,
            TValues::Float(_) => Types::Float,
            TValues::String(_) => Types::String,
            TValues::Nil => Types::Nil,
        }
    }

    pub fn get_truthyness(self) -> bool {
        match self {
            TValues::Bool(v) => v,
            TValues::Float(v) => !(v == 0.0),
            TValues::String(v) => !v.is_empty(),
            TValues::Nil => false,
        }
    }
}

impl PartialEq for TValues {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TValues::Bool(l), TValues::Bool(r)) => l == r,
            (TValues::Float(l), TValues::Float(r)) => l == r,
            (TValues::String(l), TValues::String(r)) => l == r,
            (TValues::Nil, TValues::Nil) => true,
            _ => false,
        }
    }
}

impl PartialOrd for TValues {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (self, other) {
            (TValues::Bool(l), TValues::Bool(r)) => l.partial_cmp(r),
            (TValues::Float(l), TValues::Float(r)) => l.partial_cmp(r),
            (TValues::String(l), TValues::String(r)) => l.partial_cmp(r),
            (TValues::Nil, TValues::Nil) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

impl Display for TValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TValues::Bool(v) => write!(f, "{v}"),
            TValues::Float(v) => write!(f, "{v}"),
            TValues::String(v) => write!(f, "{v}"),
            TValues::Nil => write!(f, "Nil"),
        }
    }
}

impl TryFrom<TValues> for f64 {
    type Error = &'static str;

    fn try_from(value: TValues) -> Result<Self, Self::Error> {
        match value {
            TValues::Float(v) => Ok(v),
            _ => Err("Failed to convert"),
        }
    }
}

impl TryFrom<TokenType> for TValues {
    type Error = &'static str;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::True => Ok(TValues::Bool(true)),
            TokenType::False => Ok(TValues::Bool(false)),
            TokenType::Number(v) => Ok(TValues::Float(v)),
            TokenType::String(v) => Ok(TValues::String(v)),
            TokenType::Nil => Ok(TValues::Nil),
            _ => Err("Failed to convert"),
        }
    }
}
