use crate::parser::types::Types;

use super::_Error;

#[derive(Debug)]
pub enum TypeError {
    Mismatch {
        expected: MismatchType,
        found: MismatchType,
    },
}

#[derive(Debug)]
pub enum MismatchType {
    Single(Vec<Types>),
    Multiple(Vec<(Types, Types)>),
}

impl _Error for TypeError {}
