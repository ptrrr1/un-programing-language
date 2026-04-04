#[derive(Debug, Clone)]
pub struct Error<T> {
    pos: (usize, usize),
    err_type: T,
    //val: ScannerErrorType,
}

impl<T: _Error> Error<T> {
    pub fn new(pos: (usize, usize), err_type: T) -> Self {
        Self { pos, err_type }
    }
}

#[derive(Debug)]
pub enum ScannerError {
    InvalidToken(String),
    MissingWhitespace, // More generic with: MissingSeparation (space, comma, etc)
    MultipleDecimalDivider,
    UnclosedString,
}

#[derive(Debug)]
pub enum ParserError {}

pub trait _Error {}

impl _Error for ScannerError {}

impl _Error for ParserError {}
