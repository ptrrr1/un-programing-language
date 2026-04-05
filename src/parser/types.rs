#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Types {
    Bool,
    Int,
    Float,
    String,
    Nil,
}

#[derive(Debug, Clone)]
pub enum TValues {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Nil,
}

impl TValues {
    pub fn as_type(self) -> Types {
        match self {
            TValues::Bool(_) => Types::Bool,
            TValues::Int(_) => Types::Int,
            TValues::Float(_) => Types::Float,
            TValues::String(_) => Types::String,
            TValues::Nil => Types::Nil,
        }
    }
}

impl PartialEq for TValues {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TValues::Bool(l), TValues::Bool(r)) => l == r,
            (TValues::Int(l), TValues::Int(r)) => l == r,
            (TValues::Float(l), TValues::Float(r)) => l == r,
            (TValues::String(l), TValues::String(r)) => l == r,
            (TValues::Nil, TValues::Nil) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for TValues {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (self, other) {
            (TValues::Bool(l), TValues::Bool(r)) => l.partial_cmp(r),
            (TValues::Int(l), TValues::Int(r)) => l.partial_cmp(r),
            (TValues::Float(l), TValues::Float(r)) => l.partial_cmp(r),
            (TValues::String(l), TValues::String(r)) => l.partial_cmp(r),
            (TValues::Nil, TValues::Nil) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

impl ToString for TValues {
    fn to_string(&self) -> String {
        match self {
            TValues::Bool(v) => v.to_string(),
            TValues::Int(v) => v.to_string(),
            TValues::Float(v) => v.to_string(),
            TValues::String(v) => v.clone(), // TODO: Remove this clone
            TValues::Nil => "Nil".to_string(),
        }
    }
}

impl TryFrom<TValues> for f32 {
    type Error = &'static str;

    fn try_from(value: TValues) -> Result<Self, Self::Error> {
        match value {
            TValues::Int(v) => Ok(v as f32),
            TValues::Float(v) => Ok(v),
            _ => Err("Failed to convert"),
        }
    }
}

impl TryFrom<TValues> for i32 {
    type Error = &'static str;

    fn try_from(value: TValues) -> Result<Self, Self::Error> {
        match value {
            TValues::Int(v) => Ok(v),
            _ => Err("Failed to convert"),
        }
    }
}
