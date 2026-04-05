use super::_Error;

#[derive(Debug)]
pub enum ParserError {
    InvalidToken(crate::tokens::TokenType),
    UnexpectedEOF,
    UnclosedGrouping, // Missing ')'
}

impl _Error for ParserError {}
