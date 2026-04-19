use crate::tokens::Token;
use std::rc::Rc;

pub mod eval;

#[derive(Debug, Clone)]
pub enum Expr {
    Assignment {
        target: Rc<Expr>,
        expr: Rc<Expr>,
    },
    Binary {
        left: Rc<Expr>,
        operator: Token,
        right: Rc<Expr>,
    },
    Unary {
        operator: Token,
        right: Rc<Expr>,
    },
    Grouping(Rc<Expr>),
    Literal(Token),
    Variable(Token),
    ExposedFn(Token),
    Conditional {
        condition: Rc<Expr>,
        true_branch: Rc<Expr>,
        false_branch: Rc<Expr>,
    },
    Call {
        callee: Rc<Expr>,
        paren: Token,
        args: Vec<Expr>,
    },
    Lambda {
        params: Vec<Token>,
        body: Rc<Expr>,
    },
}

impl Expr {
    pub fn assignment(target: Expr, expr: Expr) -> Expr {
        Expr::Assignment {
            target: Rc::new(target),
            expr: Rc::new(expr),
        }
    }

    pub fn binary(left: Expr, op: Token, right: Expr) -> Expr {
        Expr::Binary {
            left: Rc::new(left),
            operator: op,
            right: Rc::new(right),
        }
    }

    pub fn unary(op: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator: op,
            right: Rc::new(right),
        }
    }

    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping(Rc::new(expr))
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
            condition: Rc::new(condition),
            true_branch: Rc::new(true_branch),
            false_branch: Rc::new(false_branch),
        }
    }

    pub fn callable(callee: Expr, paren: Token, args: Vec<Expr>) -> Expr {
        Expr::Call {
            callee: Rc::new(callee),
            paren,
            args,
        }
    }

    pub fn lambda(params: Vec<Token>, body: Expr) -> Expr {
        Expr::Lambda {
            params,
            body: Rc::new(body),
        }
    }
}
