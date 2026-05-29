use crate::types::value::Value;

#[derive(Debug, Clone)]
pub enum Signal {
    Normal,
    Return(Value),
    Break,
    // Continue,
    // RuntimeError
}
