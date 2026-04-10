use super::_Error;

#[derive(Debug)]
pub enum ParserError {
    InvalidToken(crate::tokens::TokenType),
    UnexpectedEOF,
    UnclosedExpr, // Missing ')'
    InvalidPrint,
    UnterminatedStmt,
    InvalidAssignment,
    // ExpectedExpr,
}

impl _Error for ParserError {}
