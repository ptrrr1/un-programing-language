use std::fmt::Display;

use crate::tokens::TokenType;

#[derive(Debug)]
pub enum ParserError {
    InvalidToken(TokenType),
    UnexpectedEOF,
    UnclosedExpr, // Missing ')'
    InvalidPrint,
    UnterminatedStmt,
    InvalidAssignment,

    // Function Decl
    ExpectedLeftParenthesisFunDecl(TokenType),
    MissingRightParenthesisFunDecl(TokenType),
    ExcessiveArgumentsFunDecl(TokenType),
    ExpectedIdentifier,

    // Block
    ExpectedBeginBlock,
    UnterminatedBlock,
    // ExpectedExpr

    // For Loops
    MissingKeywordIn,
    MissingDoBlockStart,

    // Calls
    UnclosedCallExpr,
    ExcessiveArguments,

    // If Else Expr
    MissingThenToken,
    MissingElseToken,
    UnterminatedIfElseExpr,

    // Range
    ExpectedRangeStart,
    MissingRangeOperator,
    MissingRangeCondition,
    UnclosedRange,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser Error: ")?;
        match self {
            ParserError::InvalidToken(token_type) => write!(f, "Invalid Token({token_type})"),
            ParserError::UnexpectedEOF => write!(f, "Unexpected End of File"),
            ParserError::UnclosedExpr => write!(f, "Unclosed Expression"),
            ParserError::InvalidPrint => write!(f, "Invalid syntax for Print Statement"),
            ParserError::UnterminatedStmt => write!(f, "Unterminated Statementet, Missing ';'"),
            ParserError::InvalidAssignment => write!(f, "Invalid Assignment Target"),
            // Function Decl
            ParserError::ExpectedLeftParenthesisFunDecl(token_type) => {
                write!(
                    f,
                    "Expected '(' In Function Declaration <fn {}>",
                    token_type
                )
            }
            ParserError::MissingRightParenthesisFunDecl(token_type) => {
                write!(f, "Missing ')' in Function Declaration <fn {}>", token_type)
            }
            ParserError::ExcessiveArgumentsFunDecl(token_type) => write!(
                f,
                "Excessive arguments in Function Declaration <fn {}>, Limit is 255",
                token_type
            ),
            ParserError::ExpectedIdentifier => {
                write!(f, "Expected Identifier in Function Declaration")
            }
            // Block
            ParserError::ExpectedBeginBlock => write!(f, "Expected Block Starter 'begin'"),
            ParserError::UnterminatedBlock => write!(f, "Unterminated Block, Missing 'end'"),
            // For/While Loops
            ParserError::MissingKeywordIn => write!(f, "Missing Keyword 'in'"),
            ParserError::MissingDoBlockStart => write!(f, "Missing Block Starter 'do'"),
            // Calls
            ParserError::UnclosedCallExpr => write!(f, "Unclosed Call Expression"),
            ParserError::ExcessiveArguments => write!(f, "Excessive Arguments in Call Expression"),
            // If Else Expr
            ParserError::MissingThenToken => write!(f, "Missing 'then' in Conditional Expression"),
            ParserError::MissingElseToken => write!(f, "Missing 'else' in Conditional Expression"),
            ParserError::UnterminatedIfElseExpr => {
                write!(f, "Unterminanted Conditional Expression, Missing 'end'")
            }
            // Range
            ParserError::ExpectedRangeStart => write!(f, "Expected Range Starter '['"),
            ParserError::MissingRangeOperator => write!(f, "Missing Range Operator '..'"),
            ParserError::MissingRangeCondition => write!(f, "Missing Range Condition ('>' or '<')"),
            ParserError::UnclosedRange => write!(f, "Unclosed Range, Missing ']'"),
        }
    }
}

impl std::error::Error for ParserError {}
