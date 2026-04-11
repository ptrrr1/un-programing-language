use std::fmt::Display;

use crate::parser::types::Types;

#[derive(Debug)]
pub enum TypeError {
    Mismatch {
        expected: MismatchType,
        found: MismatchType,
    },
}

#[derive(Debug)]
pub enum MismatchType {
    // TODO: Remake this!!!
    Single(Vec<Types>),
    Multiple(Vec<(Types, Types)>),
}

impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for TypeError {}
