/*
Precedence (Lowest to Highest)
== !=
> >= < <=
- +
/ *
- not

PROGRAM -> STATEMENTS* EOF

STATEMENT -> EXPR_STATEMENT | PRINT_STATEMENT

PRINT_STATEMENT -> "print""("EXPRESSION ")"";"

EXPR_STATEMENT -> EXPRESSION ";""

EXPRESSION -> EQUALITY
EQUALITY -> COMPARISON ( ( "==" | "!=" ) COMPARISON )*
COMPARSION TERM ( ( "<" | "<=" | ">" | ">=" ) TERM )*
TERM -> FACTOR ( ( "+" | "-" ) FACTOR )*
FACTOR -> UNARY ( ( "/" | "*" ) UNARY )*
UNARY -> ( "not" | "-" ) UNARY | PRIMARY
PRIMARY -> LITERAL | STRING | BOOL | NIL | "(" EXPRESSION ")"
*/

use std::iter::Peekable;

use expr::Expr;
use stmt::Stmt;

use crate::{
    errors::{Error, Pos, parser_errors::ParserError},
    tokens::{Token, TokenType},
};

pub mod expr;
pub mod stmt;
pub mod typed_expr;
pub mod types;

#[derive(Debug, Default)]
pub struct ParserResult {
    errors: Vec<Error<ParserError>>,
    stmt: Vec<Stmt>,
}

impl ParserResult {
    pub fn into_stmt(self) -> Vec<Stmt> {
        self.stmt
    }

    pub fn into_err(self) -> Vec<Error<ParserError>> {
        self.errors
    }

    pub fn has_err(&self) -> bool {
        !self.errors.is_empty()
    }
}

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn parse_tokens<I: Iterator<Item = Token>>(tokens: I) -> ParserResult {
        let mut parser_result = ParserResult::default();

        let mut t = tokens.peekable();
        while t.peek().is_some() {
            match Self::statement(&mut t) {
                Ok(stmt) => parser_result.stmt.push(stmt),
                Err(e) => {
                    parser_result.errors.push(e);
                    Self::synchronize(&mut t);
                }
            }
        }

        parser_result
    }

    fn statement<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        if let Some(t) = tokens.peek() {
            match t.token_type {
                TokenType::Print => Self::print_statement(tokens),
                _ => Self::expr_statement(tokens),
            }
        } else {
            Err(Error::new(Pos::EOF, ParserError::UnexpectedEOF))
        }
    }

    fn print_statement<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        let _print = tokens.next().unwrap(); // I know next is PRINT

        let _ = Self::consume(
            tokens,
            |t| matches!(t, TokenType::LeftParenthesis),
            ParserError::InvalidPrint,
        )?;

        // Check if next token is ')'
        // A print() statement is not wrong, but maybe it should have a warning
        let expr = Self::expression(tokens)?;

        let _ = Self::consume(
            tokens,
            |t| matches!(t, TokenType::RightParenthesis),
            ParserError::UnclosedExpr,
        )?;

        let _ = Self::consume(
            tokens,
            |t| matches!(t, TokenType::Semicolon),
            ParserError::UnterminatedStmt,
        )?;

        Ok(Stmt::PrintStmt(expr))
    }

    fn expr_statement<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        let expr = Self::expression(tokens)?;
        let _ = Self::consume(
            tokens,
            |t| matches!(t, TokenType::Semicolon),
            ParserError::UnterminatedStmt,
        )?;
        Ok(Stmt::ExprStatement(expr))
    }

    fn expression<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        Self::equality(tokens)
    }

    fn equality<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        let mut expr = Self::comparison(tokens)?;

        while let Some(op) =
            tokens.next_if(|t| matches!(t.token_type, TokenType::BangEqual | TokenType::EqualEqual))
        {
            let expr_r = Self::comparison(tokens)?;
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        Ok(expr)
    }

    fn comparison<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        let mut expr = Self::term(tokens)?;

        while let Some(op) = tokens.next_if(|t| {
            matches!(
                t.token_type,
                TokenType::Lesser
                    | TokenType::LesserEqual
                    | TokenType::Greater
                    | TokenType::GreaterEqual
            )
        }) {
            let expr_r = Self::term(tokens)?;
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        Ok(expr)
    }

    fn term<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        let mut expr = Self::factor(tokens)?;

        while let Some(op) =
            tokens.next_if(|t| matches!(t.token_type, TokenType::Minus | TokenType::Plus))
        {
            let expr_r = Self::factor(tokens)?;
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        Ok(expr)
    }

    fn factor<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        let mut expr = Self::unary(tokens)?;

        while let Some(op) =
            tokens.next_if(|t| matches!(t.token_type, TokenType::Slash | TokenType::Star))
        {
            let expr_r = Self::unary(tokens)?;
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        Ok(expr)
    }

    fn unary<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        if let Some(op) =
            tokens.next_if(|t| matches!(t.token_type, TokenType::Not | TokenType::Minus))
        {
            // For '- ;' expressions like this but eh, might remove
            // if Self::check(tokens, |t| {
            //     matches!(
            //         t,
            //         TokenType::LeftParenthesis
            //             | TokenType::Minus
            //             | TokenType::Not
            //             | TokenType::True
            //             | TokenType::False
            //             | TokenType::Nil
            //             | TokenType::Number(_)
            //             | TokenType::String(_)
            //     )
            // })
            // .is_some()
            // {
            let expr_r = Self::unary(tokens)?;
            return Ok(Expr::unary(op.clone(), expr_r));
            // }

            // let pos = tokens.peek().map_or(Pos::EOF, |t| Pos::from(t.line));
            // return Err(Error::new(pos, ParserError::ExpectedExpr));
        }

        Self::primary(tokens)
    }

    fn primary<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        // Grouping Start
        if Self::check(tokens, |t| matches!(t, TokenType::LeftParenthesis)).is_some() {
            let expr = Self::expression(tokens)?;

            let _ = Self::consume(
                tokens,
                |t| matches!(t, TokenType::RightParenthesis),
                ParserError::UnclosedExpr,
            )?;

            return Ok(Expr::grouping(expr));
        }

        // Literal
        if let Some(t) = Self::check(tokens, |t| {
            matches!(
                t,
                TokenType::False
                    | TokenType::True
                    | TokenType::Nil
                    | TokenType::String(_)
                    | TokenType::Number(_)
            )
        }) {
            return Ok(Expr::literal(t));
        }

        // Neither Literal nor Grouping then Err
        match tokens.peek() {
            Some(t) => Err(Error::new(
                Pos::from(t.line),
                ParserError::InvalidToken(t.token_type.clone()),
            )),
            None => Err(Error::new(Pos::EOF, ParserError::UnexpectedEOF)),
        }
    }

    fn synchronize<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) {
        while let Some(t) = tokens.peek() {
            match t.token_type {
                TokenType::Semicolon => {
                    tokens.next();
                    break;
                }
                TokenType::Fun
                | TokenType::Return
                | TokenType::Let
                | TokenType::For
                | TokenType::While
                | TokenType::If => break,
                _ => {
                    tokens.next();
                }
            }
        }
    }

    // Could be replaced with some abstraction that stores the current + tokens but eh... it works
    fn consume<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
        expected: impl Fn(&TokenType) -> bool,
        err: ParserError,
    ) -> Result<Token, Error<ParserError>> {
        match tokens.next() {
            Some(t) if expected(&t.token_type) => Ok(t),
            Some(t) => Err(Error::new(Pos::from(t.line), err)),
            None => Err(Error::new(Pos::EOF, err)),
        }
    }

    fn check<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
        expected: impl Fn(&TokenType) -> bool,
    ) -> Option<Token> {
        match tokens.peek() {
            Some(t) if expected(&t.token_type) => tokens.next(),
            _ => None,
        }
    }
}
