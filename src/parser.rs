/*
Precedence (Lowest to Highest)
== !=
> >= < <=
- +
/ *
- not

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

use crate::{
    errors::{Error, Pos, parser_errors::ParserError},
    tokens::{Token, TokenType},
};

pub mod expr;
pub mod typed_expr;
pub mod types;

#[derive(Debug, Default)]
pub struct ParserResult {
    errors: Vec<Error<ParserError>>,
    expr: Vec<Expr>,
}

impl ParserResult {
    pub fn into_expr(self) -> Vec<Expr> {
        self.expr
    }

    pub fn has_err(self) -> bool {
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
            match Self::expression(&mut t) {
                Ok(expr) => parser_result.expr.push(expr),
                Err(e) => {
                    parser_result.errors.push(e);
                    Self::synchronize(&mut t);
                }
            }
        }

        parser_result
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
            let expr_r = Self::unary(tokens)?;
            return Ok(Expr::unary(op.clone(), expr_r));
        }

        Self::primary(tokens)
    }

    fn primary<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        if tokens
            .next_if(|t| matches!(t.token_type, TokenType::LeftParenthesis))
            .is_some()
        {
            let expr = Self::expression(tokens)?;

            if tokens
                .next_if(|t| matches!(t.token_type, TokenType::RightParenthesis))
                .is_none()
            {
                // TODO: find position
                // I have the line position in token, but i'm not consuming it here
                // there's a way to do it by storing the values whenever I consume but
                // fuckkkkkk that's stupid considering i'm doing it across multiple fuctions
                let pos = match tokens.peek() {
                    Some(t) => Pos::from(t.line), // Takes the next token's position...
                    None => Pos::EOF,
                };

                return Err(Error::new(pos, ParserError::UnclosedGrouping));
            }
            return Ok(Expr::grouping(expr));
        } else if let Some(t) = tokens.next_if(|t| {
            matches!(
                t.token_type,
                TokenType::False
                    | TokenType::True
                    | TokenType::Nil
                    | TokenType::String(_)
                    | TokenType::Number(_) // | TokenType::Identifier(_)
                                           // | TokenType::ExposedFunction(_)
            )
        }) {
            return Ok(Expr::literal(t));
        }

        if let Some(t) = tokens.next() {
            Err(Error::new(
                Pos::from(t.line),
                ParserError::InvalidToken(t.token_type),
            ))
        } else {
            Err(Error::new(Pos::EOF, ParserError::UnexpectedEOF))
        }
    }

    fn synchronize<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) {
        while tokens.peek().is_some_and(|t| {
            !matches!(
                t.token_type,
                TokenType::Semicolon
                    | TokenType::Fun
                    | TokenType::Return
                    | TokenType::Let
                    | TokenType::For
                    | TokenType::While
                    | TokenType::If
            )
        }) {
            tokens.next();
        }
    }
}
