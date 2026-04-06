pub mod parser_errors;
pub mod scanner_errors;
pub mod typed_parser_errors;

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

impl<T: _Error> Error<T> {
    pub fn new(pos: Pos, err_type: T) -> Self {
        Self { pos, err_type }
    }
}

pub trait _Error {}
