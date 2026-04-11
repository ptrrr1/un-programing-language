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
        true_branch: Vec<Stmt>,
        false_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        stmts: Vec<Stmt>,
    },
    // For {
    //     identifier: Token,
    //     start: i64,
    //     end: i64,
    //     step: i64,
    //     stmts: Vec<Stmt>
    // }
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
}
