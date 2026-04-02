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

use std::{
    iter::{Filter, Peekable},
    vec::IntoIter,
};

use crate::{
    errors::Error,
    tokens::{Token, TokenType},
};

#[derive(Debug)]
pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
    errors: Vec<Error>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Parser<I> {
        Parser {
            tokens: tokens.peekable(),
            errors: vec![],
        }
    }

    pub fn parse_tokens(&mut self) {
        while self.tokens.peek().is_some() {
            let expr = self.expression();
            println!("{:#?}", expr);
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
                .is_some()
            {
                // TODO: write it better
            } else {
                eprintln!("Error missing right parenthesis")
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
            )
        }) {
            return Expr::literal(t);
        }

        panic!("Malformed code");
    }
}

// Binary   : Expr Operator Expr
// Grouping : Expr
// Literal  : Value
// Unary    : Operator Expr

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal(Token),
    Grouping(Box<Expr>),
}

impl Expr {
    pub fn binary(left: Expr, op: Token, right: Expr) -> Expr {
        Expr::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        }
    }

    pub fn unary(op: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator: op,
            right: Box::new(right),
        }
    }

    pub fn literal(token: Token) -> Expr {
        Expr::Literal(token)
    }

    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping(Box::new(expr))
    }
}
