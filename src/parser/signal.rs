use super::types::Value;

#[derive(Debug, Clone)]
pub enum Signal {
    Normal,
    Return(Value),
    Break,
    Continue,
    // RuntimeError
}

impl Signal {
    pub fn unwrap_return(self) -> Value {
        match self {
            Self::Return(value) => value,
            _ => panic!("Not a return"),
        }
    }
}
