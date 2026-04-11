use std::fmt::Display;

#[derive(Debug)]
pub enum ParserError {
    InvalidToken(crate::tokens::TokenType),
    UnexpectedEOF,
    UnclosedExpr, // Missing ')'
    InvalidPrint,
    UnterminatedStmt,
    UnterminatedBlock,
    InvalidAssignment,
    // ExpectedExpr,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scanner Error: ")?;
        match self {
            ParserError::InvalidToken(token_type) => write!(f, "Invalid Token({token_type})"),
            ParserError::UnexpectedEOF => write!(f, "Unexpected End of File"),
            ParserError::UnclosedExpr => write!(f, "Unclosed Expression"),
            ParserError::InvalidPrint => write!(f, "Invalid syntax for Print Statement"),
            ParserError::UnterminatedStmt => write!(f, "Unterminated Statementet, Missing ';'"),
            ParserError::UnterminatedBlock => write!(f, "Unterminated Block, Missing 'end'"),
            ParserError::InvalidAssignment => write!(f, "Invalid Assignment Target"),
        }
    }
}

impl std::error::Error for ParserError {}
