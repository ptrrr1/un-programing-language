use std::{cell::RefCell, rc::Rc};

use crate::{
    expr::Expr,
    stmt::eval::StmtVisitor,
    tokens::{Token, TokenType},
};

pub mod eval;
pub mod resolver;
pub mod signal;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var {
        target: Token,
        expr: Box<Expr>,
    },
    Block(Vec<Stmt>),
    Conditional {
        condition: Box<Expr>,
        true_branch: Vec<Stmt>,
        false_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Box<Expr>,
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
        Self::Var {
            target,
            expr: Box::new(expr),
        }
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
            condition: Box::new(condition),
            true_branch,
            false_branch,
        }
    }

    pub fn while_stmt(condition: Expr, stmts: Vec<Stmt>) -> Self {
        Self::While {
            condition: Box::new(condition),
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

    pub fn accept<R, E>(&self, env: Rc<RefCell<E>>, visitor: &mut impl StmtVisitor<R, E>) -> R {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr(env, expr),
            Stmt::Print(expr) => visitor.visit_print(env, expr),
            Stmt::Var { target, expr } => visitor.visit_var(env, target, expr),
            Stmt::Block(stmts) => visitor.visit_block(env, stmts),
            Stmt::Conditional {
                condition,
                true_branch,
                false_branch,
            } => visitor.visit_conditional(env, condition, true_branch, false_branch),
            Stmt::While { condition, stmts } => visitor.visit_while(env, condition, stmts),
            Stmt::Function {
                identifier,
                params,
                body,
            } => visitor.visit_function(env, identifier, params, body),
            Stmt::Return(expr) => visitor.visit_return(env, expr),
            Stmt::Break => visitor.visit_break(),
        }
    }
}
