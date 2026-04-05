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
