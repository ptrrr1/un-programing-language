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
    errors::{Error, parser_errors::ParserError},
    tokens::{Token, TokenType},
};

pub mod expr;
pub mod typed_expr;
pub mod types;

#[derive(Debug)]
pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
    errors: Vec<Error<ParserError>>,
    expr: Vec<Expr>,
    // need_sync: bool,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Parser<I> {
        Parser {
            tokens: tokens.peekable(),
            errors: vec![],
            expr: vec![],
            // need_sync: false,
        }
    }

    pub fn parse_tokens(&mut self) {
        while self.tokens.peek().is_some() {
            // if self.need_sync {
            //     self.synchronize();
            //     self.need_sync = false;
            // }

            let expr = self.expression();
            self.expr.push(expr);
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while let Some(op) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::BangEqual | TokenType::EqualEqual))
        {
            let expr_r = self.comparison();
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while let Some(op) = self.tokens.next_if(|t| {
            matches!(
                t.token_type,
                TokenType::Lesser
                    | TokenType::LesserEqual
                    | TokenType::Greater
                    | TokenType::GreaterEqual
            )
        }) {
            let expr_r = self.term();
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while let Some(op) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::Minus | TokenType::Plus))
        {
            let expr_r = self.factor();
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while let Some(op) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::Slash | TokenType::Star))
        {
            let expr_r = self.unary();
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if let Some(op) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::Not | TokenType::Minus))
        {
            let expr_r = self.unary();
            return Expr::unary(op.clone(), expr_r);
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::LeftParentesis))
            .is_some()
        {
            let expr = self.expression();
            if self
                .tokens
                .next_if(|t| matches!(t.token_type, TokenType::RightParentesis))
                .is_none()
            {
                // TODO: find position
                // I have the line position in token, but i'm not consuming it here
                // there's a way to do it by storing the values whenever I consume but
                // fuckkkkkk that's stupid considering i'm doing it across multiple fuctions
                let pos = match self.tokens.peek() {
                    Some(t) => t.line, // Takes the next token's position...
                    None => (usize::MAX, usize::MIN),
                };

                self.errors
                    .push(Error::new(pos, ParserError::UnclosedGrouping));
            }
            return Expr::grouping(expr);
        } else if let Some(t) = self.tokens.next_if(|t| {
            matches!(
                t.token_type,
                TokenType::False
                    | TokenType::True
                    | TokenType::Nil
                    | TokenType::String(_)
                    | TokenType::NumberInt(_)
                    | TokenType::NumberFloat(_)
                    | TokenType::Identifier(_)
                    | TokenType::ExposedFunction(_)
            )
        }) {
            return Expr::literal(t);
        }

        // If not literal or grouping then error
        // self.need_sync = true;
        if let Some(t) = self.tokens.next() {
            // More accurate to say: expected expression
            self.errors
                .push(Error::new(t.line, ParserError::InvalidToken(t.token_type)));
        } else {
            self.errors.push(Error::new(
                (usize::MAX, usize::MIN),
                ParserError::UnexpectedEOF,
            ));
        }

        // TODO: This is clearly wrong... but idk what to do
        //  1 ++ 2 becomes -> binary(1, +, 2)
        // 1.++ 2 becomes -> literal(1), literal(1)
        self.expression()
    }

    fn synchronize(&mut self) {
        while self.tokens.peek().is_some_and(|t| {
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
            self.tokens.next();
        }
    }

    pub fn into_expr(self) -> Vec<Expr> {
        self.expr
    }
}
