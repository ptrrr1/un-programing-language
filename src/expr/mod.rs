use crate::{expr::eval::ExprVisitor, tokens::Token};
use std::{cell::RefCell, rc::Rc};

pub mod eval;

#[derive(Debug, Clone)]
pub enum Expr {
    Assignment {
        target: Box<Expr>,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Token),
    Variable(Token),
    ExposedFn(Token),
    Conditional {
        condition: Box<Expr>,
        true_branch: Box<Expr>,
        false_branch: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        args: Vec<Expr>,
    },
    Lambda {
        params: Vec<Token>,
        body: Box<Expr>,
    },
}

impl Expr {
    pub fn assignment(target: Expr, expr: Expr) -> Expr {
        Expr::Assignment {
            target: Box::new(target),
            expr: Box::new(expr),
        }
    }

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

    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping(Box::new(expr))
    }

    pub fn literal(token: Token) -> Expr {
        Expr::Literal(token)
    }

    pub fn variable(token: Token) -> Expr {
        Expr::Variable(token)
    }

    pub fn exposed_fn(token: Token) -> Expr {
        Expr::ExposedFn(token)
    }

    pub fn conditional(condition: Expr, true_branch: Expr, false_branch: Expr) -> Expr {
        Expr::Conditional {
            condition: Box::new(condition),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        }
    }

    pub fn callable(callee: Expr, paren: Token, args: Vec<Expr>) -> Expr {
        Expr::Call {
            callee: Box::new(callee),
            paren,
            args,
        }
    }

    pub fn lambda(params: Vec<Token>, body: Expr) -> Expr {
        Expr::Lambda {
            params,
            body: Box::new(body),
        }
    }

    pub fn accept<R, E>(&self, env: Rc<RefCell<E>>, visitor: &mut impl ExprVisitor<R, E>) -> R {
        match self {
            Expr::Assignment { target, expr } => visitor.visit_assignment(env, target, expr),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(env, left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary(env, operator, right),
            Expr::Grouping(expr) => visitor.visit_grouping(env, expr),
            Expr::Literal(token) => visitor.visit_literal(env, token),
            Expr::Variable(token) => visitor.visit_variable(env, token),
            Expr::ExposedFn(token) => visitor.visit_exposed_fn(env, token),
            Expr::Conditional {
                condition,
                true_branch,
                false_branch,
            } => visitor.visit_conditional(env, condition, true_branch, false_branch),
            Expr::Call {
                callee,
                paren,
                args,
            } => visitor.visit_call(env, callee, paren, args),
            Expr::Lambda { params, body } => visitor.visit_lambda(env, params, body),
        }
    }
}
