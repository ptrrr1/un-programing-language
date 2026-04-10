use crate::tokens::Token;

#[derive(Debug, Clone)]
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
    Assignment {
        target: Box<Expr>,
        expr: Box<Expr>,
    },
    Conditional {
        condition: Box<Expr>,
        true_case: Box<Expr>,
        false_case: Box<Expr>,
    },
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

    pub fn assignment(target: Expr, expr: Expr) -> Expr {
        Expr::Assignment {
            target: Box::new(target),
            expr: Box::new(expr),
        }
    }

    pub fn conditional(condition: Expr, true_case: Expr, false_case: Expr) -> Expr {
        Expr::Conditional {
            condition: Box::new(condition),
            true_case: Box::new(true_case),
            false_case: Box::new(false_case),
        }
    }
}
