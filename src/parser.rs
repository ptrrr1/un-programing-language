use std::iter::Peekable;

use expr::Expr;
use stmt::Stmt;

use crate::{
    errors::{Error, Pos, parser_errors::ParserError},
    tokens::{Token, TokenType},
};

pub mod callable;
pub mod expr;
pub mod stmt;
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
        let _let = tokens.next().unwrap(); // I know next is LET

        let identifer = Self::primary(tokens)?;

        Self::consume(
            tokens,
            vec![TokenType::ColonEqual],
            ParserError::UnterminatedStmt,
        )?;

        let expr = Self::expression(tokens)?;

        Self::consume(
            tokens,
            vec![TokenType::Semicolon],
            ParserError::UnterminatedStmt,
        )?;

        Ok(Stmt::var(identifer, expr))
    }

    fn statement<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        if let Some(t) = tokens.peek() {
            match t.token_type {
                TokenType::Print => Self::print_stmt(tokens),
                TokenType::If => Self::conditional_stmt(tokens),
                TokenType::While => Self::while_stmt(tokens),
                TokenType::For => Self::for_stmt(tokens),
                TokenType::Begin => {
                    // According to the book, this will be reused for functions!
                    let _begin = tokens.next().unwrap(); // I know next is BEGIN
                    Ok(Stmt::block(Self::block(tokens)?))
                }
                _ => Self::expr_statement(tokens),
            }
        } else {
            Err(Error::new(Pos::EOF, ParserError::UnexpectedEOF))
        }
    }

    fn print_stmt<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        let _print = tokens.next().unwrap(); // I know next is PRINT

        Self::consume(
            tokens,
            vec![TokenType::LeftParenthesis],
            ParserError::InvalidPrint,
        )?;

        // Check if next token is ')'
        // A print() statement is not wrong, but maybe it should have a warning
        let expr = Self::expression(tokens)?;

        Self::consume(
            tokens,
            vec![TokenType::RightParenthesis],
            ParserError::UnclosedExpr,
        )?;

        Self::consume(
            tokens,
            vec![TokenType::Semicolon],
            ParserError::UnterminatedStmt,
        )?;

        Ok(Stmt::print(expr))
    }

    fn conditional_stmt<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        let _if = tokens.next().unwrap();

        let condition = Self::equality(tokens)?;

        // TODO: Add correct err
        Self::consume(tokens, vec![TokenType::Then], ParserError::UnexpectedEOF)?;

        let true_branch = Self::block_helper(tokens, vec![TokenType::End, TokenType::Else])?;

        if tokens
            .next_if(|t| matches!(t.token_type, TokenType::Else))
            .is_none()
        {
            Self::consume(tokens, vec![TokenType::End], ParserError::UnexpectedEOF)?;

            return Ok(Stmt::conditional(condition, true_branch, None));
        }

        // Don't need a Self::consume for 'else' because of "next_if", it
        // consumes if it finds it

        let false_branch = Self::block(tokens)?;

        Ok(Stmt::conditional(
            condition,
            true_branch,
            Some(false_branch),
        ))
    }

    fn while_stmt<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        let _while = tokens.next().unwrap();

        let condition = Self::or(tokens)?;

        Self::consume(
            tokens,
            vec![TokenType::Do],
            ParserError::UnterminatedBlock, // TODO: Actual Err
        )?;

        let stmts = Self::block(tokens)?;

        Ok(Stmt::while_stmt(condition, stmts))
    }

    fn for_stmt<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        let _for = tokens.next().unwrap();

        let identifier = Self::primary(tokens)?;

        Self::consume(
            tokens,
            vec![TokenType::In],
            ParserError::UnterminatedBlock, // TODO: Actual Err
        )?;

        let (start, condition, end, step) = Self::helper_range(tokens)?;

        Self::consume(
            tokens,
            vec![TokenType::Do],
            ParserError::UnterminatedBlock, // TODO: Actual Err
        )?;

        let stmts = Self::block(tokens)?;

        Ok(Stmt::for_stmt(
            identifier, start, end, step, condition, stmts,
        ))
    }

    fn block<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Vec<Stmt>, Error<ParserError>> {
        let stmts = Self::block_helper(tokens, vec![TokenType::End])?;

        Self::consume(tokens, vec![TokenType::End], ParserError::UnterminatedBlock)?;

        Ok(stmts)
    }

    fn expr_statement<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Stmt, Error<ParserError>> {
        let expr = Self::expression(tokens)?;
        Self::consume(
            tokens,
            vec![TokenType::Semicolon],
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
        let expr = Self::or(tokens)?;

        if let Some(eq) = tokens.next_if(|t| matches!(t.token_type, TokenType::Equal)) {
            // NOTE: This allows something like:
            // x = y = z = ... = LITERAL;
            // While not bad, just seems weird I guess
            // Might be kept
            let val = Self::assignment(tokens)?;

            if matches!(expr, Expr::Variable(_)) {
                return Ok(Expr::assignment(expr, val));
            }

            // This synchronizes but the book says not to
            return Err(Error::new(
                Pos::from(eq.line),
                ParserError::InvalidAssignment,
            ));
        }

        Ok(expr)
    }

    fn or<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Expr, Error<ParserError>> {
        let mut expr = Self::and(tokens)?;

        while let Some(op) = tokens.next_if(|t| matches!(t.token_type, TokenType::Or)) {
            let expr_r = Self::and(tokens)?;
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        Ok(expr)
    }

    fn and<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        let mut expr = Self::equality(tokens)?;

        while let Some(op) = tokens.next_if(|t| matches!(t.token_type, TokenType::And)) {
            let expr_r = Self::equality(tokens)?;
            expr = Expr::binary(expr, op.clone(), expr_r);
        }

        Ok(expr)
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

        Self::call(tokens)
    }

    fn call<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        let mut callee = Self::primary(tokens)?;

        while tokens
            .next_if(|t| matches!(t.token_type, TokenType::LeftParenthesis))
            .is_some()
        {
            let (paren, args) = Self::finish_call(tokens)?;
            callee = Expr::callable(callee, paren, args);
        }

        Ok(callee)
    }

    fn finish_call<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<(Token, Vec<Expr>), Error<ParserError>> {
        let mut args: Vec<Expr> = vec![];
        if tokens
            .peek()
            .is_some_and(|t| !matches!(t.token_type, TokenType::RightParenthesis))
        {
            // Makeshift do while Loop
            loop {
                if args.len() >= 255 {
                    // TODO: Actual Err + Position
                    // TODO: Find a better way to do this
                    // Book says not to synchronize, just report
                    return Err(Error::new(Pos::EOF, ParserError::UnexpectedEOF));
                    // eprintln!("Can't have more than 255 args");
                }
                args.push(Self::or(tokens)?);

                if tokens
                    .next_if(|t| matches!(t.token_type, TokenType::Comma))
                    .is_none()
                {
                    break;
                }
            }
        }

        // TODO: Fix err
        let paren = Self::consume(
            tokens,
            vec![TokenType::RightParenthesis],
            ParserError::UnclosedExpr,
        )?;

        Ok((paren, args))
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

            Self::consume(
                tokens,
                vec![TokenType::RightParenthesis],
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
            )
        }) {
            return Ok(Expr::literal(t));
        }

        // Exposed Fn
        if let Some(t) = tokens.next_if(|t| matches!(t.token_type, TokenType::ExposedFunction(_))) {
            return Ok(Expr::exposed_fn(t));
        }

        // Identifier
        if let Some(t) = tokens.next_if(|t| matches!(t.token_type, TokenType::Identifier(_))) {
            return Ok(Expr::variable(t));
        }

        // Conditional
        if tokens
            .next_if(|t| matches!(t.token_type, TokenType::If))
            .is_some()
        {
            return Self::conditional_expr(tokens);
        }

        // Neither Literal nor Grouping nor Conditional then Err
        match tokens.peek() {
            Some(t) => Err(Error::new(
                Pos::from(t.line),
                ParserError::InvalidToken(t.token_type.clone()),
            )),
            None => Err(Error::new(Pos::EOF, ParserError::UnexpectedEOF)),
        }
    }

    fn conditional_expr<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Expr, Error<ParserError>> {
        let condition = Self::or(tokens)?;

        // TODO: Add correct err
        Self::consume(tokens, vec![TokenType::Then], ParserError::UnexpectedEOF)?;
        let true_branch = Self::or(tokens)?;

        // TODO: Add correct err
        Self::consume(tokens, vec![TokenType::Else], ParserError::UnexpectedEOF)?;

        let false_branch = Self::or(tokens)?;

        // NOTE: Might remove for conditional expression...
        Self::consume(tokens, vec![TokenType::End], ParserError::UnexpectedEOF)?;

        Ok(Expr::conditional(condition, true_branch, false_branch))
    }

    fn synchronize<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) {
        while let Some(t) = tokens.peek() {
            match t.token_type {
                TokenType::Semicolon | TokenType::RightParenthesis => {
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
        expected: Vec<TokenType>,
        err: ParserError,
    ) -> Result<Token, Error<ParserError>> {
        match tokens.next() {
            Some(t) if expected.contains(&t.token_type) => Ok(t),
            Some(t) => Err(Error::new(Pos::from(t.line), err)),
            None => Err(Error::new(Pos::EOF, err)),
        }
    }

    fn block_helper<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
        not_endpoint: Vec<TokenType>,
    ) -> Result<Vec<Stmt>, Error<ParserError>> {
        let mut stmts: Vec<Stmt> = vec![];

        while tokens
            .peek()
            .is_some_and(|t| !not_endpoint.contains(&t.token_type))
        {
            let stmt = Self::declaration(tokens)?;
            stmts.push(stmt);
        }

        Ok(stmts)
    }

    fn helper_range<I: Iterator<Item = Token>>(
        tokens: &mut Peekable<I>,
    ) -> Result<(Expr, Token, Expr, Option<Expr>), Error<ParserError>> {
        Self::consume(
            tokens,
            vec![TokenType::LeftBracket],
            ParserError::UnterminatedBlock, // TODO: Actual Err
        )?;

        let start = Self::or(tokens)?;

        Self::consume(
            tokens,
            vec![TokenType::DotDot],
            ParserError::UnterminatedBlock, // TODO: Actual Err
        )?;

        let condition = Self::consume(
            tokens,
            vec![TokenType::Lesser, TokenType::Greater],
            ParserError::UnterminatedBlock, // TODO: Actual Err
        )?;

        let end = Self::or(tokens)?;

        let mut step: Option<Expr> = None;

        if tokens
            .next_if(|t| matches!(t.token_type, TokenType::Semicolon))
            .is_some()
        {
            step = Some(Self::or(tokens)?);
        }

        Self::consume(
            tokens,
            vec![TokenType::RightBracket],
            ParserError::UnterminatedBlock, // TODO: Actual Err
        )?;

        Ok((start, condition, end, step))
    }
}
