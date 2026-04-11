use std::fmt::Display;

#[derive(Debug)]
pub enum ScannerError {
    InvalidToken(String),
    MissingSeparation, // More generic with: MissingSeparation (space, comma, etc)
    MultipleDecimalDivider,
    UnclosedString,
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scanner Error: ")?;
        match self {
            ScannerError::InvalidToken(t) => write!(f, "Invalid Token: '{t}'"),
            ScannerError::MissingSeparation => {
                write!(f, "Missing Separation, Add a space after a number")
            }
            ScannerError::MultipleDecimalDivider => {
                write!(f, "Multiple Decimal Divider, Use a single '.'")
            }
            ScannerError::UnclosedString => write!(f, "Unclosed String, Missing '\"'"),
        }
    }
}

impl std::error::Error for ScannerError {}
