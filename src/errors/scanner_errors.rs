use super::_Error;

#[derive(Debug)]
pub enum ScannerError {
    InvalidToken(String), // TODO: Change to str
    MissingWhitespace,    // More generic with: MissingSeparation (space, comma, etc)
    MultipleDecimalDivider,
    UnclosedString,
}

impl _Error for ScannerError {}
