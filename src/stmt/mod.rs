use crate::{
    expr::Expr,
    tokens::{Token, TokenType},
};

pub mod eval;
pub mod signal;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var {
        target: Token,
        expr: Expr,
    },
    Block(Vec<Stmt>),
    Conditional {
        condition: Expr,
        true_branch: Vec<Stmt>,
        false_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        stmts: Vec<Stmt>,
    },
    For {
        identifier: Token,
        start: Expr,
        end: Expr,
        step: Expr,
        condition: Token,
        stmts: Vec<Stmt>,
    },
    Function {
        identifier: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Return(Expr),
    Break,
    // Continue,
}

impl Stmt {
    pub fn expr(expr: Expr) -> Self {
        Self::Expr(expr)
    }

    pub fn print(expr: Expr) -> Self {
        Self::Print(expr)
    }

    pub fn var(target: Token, expr: Expr) -> Self {
        Self::Var { target, expr }
    }

    pub fn block(stmts: Vec<Stmt>) -> Self {
        Self::Block(stmts)
    }

    pub fn conditional(
        condition: Expr,
        true_branch: Vec<Stmt>,
        false_branch: Option<Vec<Stmt>>,
    ) -> Self {
        Self::Conditional {
            condition,
            true_branch,
            false_branch,
        }
    }

    pub fn while_stmt(condition: Expr, stmts: Vec<Stmt>) -> Self {
        Self::While { condition, stmts }
    }

    pub fn for_stmt(
        identifier: Token,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        condition: Token,
        stmts: Vec<Stmt>,
    ) -> Self {
        let s = match step {
            Some(t) => t,
            None if matches!(condition.token_type, TokenType::Greater) => {
                Expr::literal(Token::new(TokenType::Number(-1.0), condition.line))
            }
            None => Expr::literal(Token::new(TokenType::Number(1.0), condition.line)),
        };

        Self::For {
            identifier,
            start,
            end,
            step: s,
            condition,
            stmts,
        }
    }

    pub fn function(identifier: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self::Function {
            identifier,
            params,
            body,
        }
    }

    pub fn return_stmt(expr: Option<Expr>) -> Self {
        match expr {
            Some(e) => Self::Return(e),
            // TODO: Fix position
            None => Self::Return(Expr::literal(Token::new(TokenType::Nil, (0, 0)))),
        }
    }

    pub fn break_stmt() -> Self {
        Self::Break
    }

    // pub fn continue_stmt() -> Self {
    //     Self::Continue
    // }
}
