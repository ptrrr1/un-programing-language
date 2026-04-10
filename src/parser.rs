/*
Precedence (Lowest to Highest)
== !=
> >= < <=
- +
/ *
- not

PROGRAM -> DECLARATION* EOF

DECLARATION -> VAR_DECL | STATEMENT

VAR_DECL -> let IDENTIFIER ":=" EXPR ";"

STATEMENT -> EXPR_STATEMENT | PRINT_STATEMENT

PRINT_STATEMENT -> "print""("EXPRESSION ")"";"

EXPR_STATEMENT -> EXPRESSION";""
EXPRESSION -> ASSIGNMENT
ASSIGNMENT -> IDENTIFIER "=" EXPRESSION | CONDITIONAL

CONDITIONAL -> "IF" EQUALITY "THEN" EQUALITY "ELSE" CONDITIONAL | EQUALITY

EQUALITY -> COMPARISON ( ( "==" | "!=" ) COMPARISON )*
COMPARSION -> TERM ( ( "<" | "<=" | ">" | ">=" ) TERM )*
TERM -> FACTOR ( ( "+" | "-" ) FACTOR )*
FACTOR -> UNARY ( ( "/" | "*" ) UNARY )*
UNARY -> ( "not" | "-" ) UNARY | PRIMARY
PRIMARY -> LITERAL | STRING | BOOL | NIL | "(" EXPRESSION ")" | IDENTIFIER
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
            match Self::declaration(&mut t) {
                Ok(stmt) => parser_result.stmt.push(stmt),
                Err(e) => {
                    parser_result.errors.push(e);
                    Self::synchronize(&mut t);
                }
            }
        }

        parser_result
    }

    fn declaration<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        if let Some(t) = tokens.peek() {
            match t.token_type {
                TokenType::Let => Self::var_declaration(tokens),
                _ => Self::statement(tokens),
            }
        } else {
            Err(Error::new(Pos::EOF, ParserError::UnexpectedEOF))
        }
    }

    fn var_declaration<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        // TODO: after adding type keywords, accept variable declaration without initialization
        let _let = tokens.next().unwrap(); // I know next is LET

        let identifer = Self::consume(
            tokens,
            |t| matches!(t, TokenType::Identifier(_)),
            ParserError::UnterminatedStmt,
        )?; // TODO: new err

        let _ = Self::consume(
            tokens,
            |t| matches!(t, TokenType::ColonEqual),
            ParserError::UnterminatedStmt,
        )?;

        let expr = Self::expression(tokens)?;

        let _ = Self::consume(
            tokens,
            |t| matches!(t, TokenType::Semicolon),
            ParserError::UnterminatedStmt,
        )?;

        Ok(Stmt::Var(identifer, expr))
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

        Ok(Stmt::Print(expr))
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
        Ok(Stmt::Expr(expr))
    }

    fn expression<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        Self::assignment(tokens)
    }

    fn assignment<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        let expr = Self::conditional_expr(tokens)?;

        if let Some(eq) = tokens.next_if(|t| matches!(t.token_type, TokenType::Equal)) {
            let val = Self::assignment(tokens)?;

            if matches!(expr, Expr::Literal(_)) {
                return Ok(Expr::assignment(expr, val));
            }

            return Err(Error::new(
                Pos::from(eq.line),
                ParserError::InvalidAssignment,
            ));
        }

        Ok(expr)
    }

    fn conditional_expr<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        if tokens
            .next_if(|t| matches!(t.token_type, TokenType::If))
            .is_none()
        {
            return Self::equality(tokens);
        }

        let condition = Self::equality(tokens)?;

        // TODO: Add correct err
        let _ = Self::consume(
            tokens,
            |t| matches!(t, TokenType::Then),
            ParserError::UnexpectedEOF,
        );
        let true_branch = Self::equality(tokens)?;

        // TODO: Add correct err
        let _ = Self::consume(
            tokens,
            |t| matches!(t, TokenType::Else),
            ParserError::UnexpectedEOF,
        );

        let false_branch = Self::conditional_expr(tokens)?;

        let _ = Self::consume(
            tokens,
            |t| matches!(t, TokenType::End),
            ParserError::UnexpectedEOF,
        );

        Ok(Expr::conditional(condition, true_branch, false_branch))
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
        if tokens
            .next_if(|t| matches!(t.token_type, TokenType::LeftParenthesis))
            .is_some()
        {
            let expr = Self::expression(tokens)?;

            let _ = Self::consume(
                tokens,
                |t| matches!(t, TokenType::RightParenthesis),
                ParserError::UnclosedExpr,
            )?;

            return Ok(Expr::grouping(expr));
        }

        // Literal
        if let Some(t) = tokens.next_if(|t| {
            matches!(
                t.token_type,
                TokenType::False
                    | TokenType::True
                    | TokenType::Nil
                    | TokenType::String(_)
                    | TokenType::Number(_)
                    | TokenType::Identifier(_)
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
}
