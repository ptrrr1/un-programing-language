use std::fmt::Display;

pub mod parser_errors;
pub mod scanner_errors;

// TODO: use thiserror

#[derive(Debug)]
pub enum Pos {
    Known(usize, usize),
    EOF,
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Self::Known(value.0, value.1)
    }
}

#[derive(Debug)]
pub struct Error<T> {
    pos: Pos,
    err_type: T,
}

impl<T: std::error::Error> Error<T> {
    pub fn new(pos: Pos, err_type: T) -> Self {
        Self { pos, err_type }
    }
}

impl<T: Display> Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.pos {
            Pos::Known(l, c) => write!(f, "[({l},{c})]::ERROR: {}", self.err_type),
            Pos::EOF => write!(f, "[EOF]::ERROR: {}", self.err_type),
        }
    }
}
