pub mod parser_errors;
pub mod scanner_errors;
pub mod typed_parser_errors;

#[derive(Debug, Clone)]
pub struct Error<T> {
    pos: (usize, usize),
    err_type: T,
}

impl<T: _Error> Error<T> {
    pub fn new(pos: (usize, usize), err_type: T) -> Self {
        Self { pos, err_type }
    }
}

pub trait _Error {}
