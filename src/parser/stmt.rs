use crate::tokens::Token;

use super::expr::Expr;

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
        true_branch: Box<Stmt>,
        false_branch: Option<Box<Stmt>>,
    },
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

    pub fn conditional(condition: Expr, true_branch: Stmt, false_branch: Option<Stmt>) -> Self {
        match false_branch {
            Some(stmt) => Self::Conditional {
                condition,
                true_branch: Box::new(true_branch),
                false_branch: Some(Box::new(stmt)),
            },
            None => Self::Conditional {
                condition,
                true_branch: Box::new(true_branch),
                false_branch: None,
            },
        }
    }
}
