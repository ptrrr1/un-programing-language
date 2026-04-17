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

    // Lambda
    // ExpectedLambdaBody,

    // Function Decl
    ExpectedLeftParenthesisFnDecl(TokenType),
    MissingRightParenthesisFnDecl(TokenType),
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
            ParserError::UnterminatedStmt => write!(f, "Unterminated Statement, Missing ';'"),
            ParserError::InvalidAssignment => write!(f, "Invalid Assignment Target"),
            // Lambda
            // ParserError::ExpectedLambdaBody => write!(f, "Expected Lambda Body"),
            // Function Decl
            ParserError::ExpectedLeftParenthesisFnDecl(token_type) => {
                match token_type {
                    TokenType::Identifier(s) => {
                        write!(f, "Expected '(' In Function Declaration <fn {}>", s)
                    }
                    // I'm only expecting TokenType::Fn, so anything else would be an err, but I don't really want to panic
                    _ => write!(f, "Expected '(' In Function Declaration <lambda>"),
                }
            }
            ParserError::MissingRightParenthesisFnDecl(token_type) => {
                match token_type {
                    TokenType::Identifier(s) => {
                        write!(f, "Missing ')' in Function Declaration <fn {}>", s)
                    }
                    // I'm only expecting TokenType::Fn, so anything else would be an err, but I don't really want to panic
                    _ => write!(f, "Missing ')' In Function Declaration <lambda>"),
                }
            }
            ParserError::ExcessiveArgumentsFunDecl(token_type) => {
                match token_type {
                    TokenType::Identifier(s) => {
                        write!(
                            f,
                            "Excessive arguments in Function Declaration <fn {}>, Limit is 255",
                            s
                        )
                    }
                    // I'm only expecting TokenType::Fn, so anything else would be an err, but I don't really want to panic
                    _ => write!(
                        f,
                        "Excessive arguments in Function Declaration <lambda>, Limit is 255",
                    ),
                }
            }
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
