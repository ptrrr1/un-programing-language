use crate::{errors::typed_parser_errors::TypeError, tokens::Token};

use super::{expr::Expr, typed_expr::TypedExpr};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(Token, Expr),
}

pub enum TypedStmt {
    Expr(TypedExpr),
    Print(TypedExpr),
    Var(Token, TypedExpr),
}

impl TypedStmt {
    pub fn eval(&self) {
        match self {
            TypedStmt::Expr(typed_expr) => println!(":: {:?}", typed_expr.eval()),
            TypedStmt::Print(typed_expr) => {
                println!(";; {:?}", typed_expr.eval().to_string())
            }
            TypedStmt::Var(token, typed_expr) => {
                println!(":: {:?} := {:?}", token.token_type, typed_expr.eval())
            }
        }
    }
}

impl TryFrom<Stmt> for TypedStmt {
    type Error = TypeError;

    fn try_from(value: Stmt) -> Result<Self, Self::Error> {
        match value {
            Stmt::Expr(expr) => match TypedExpr::try_from(expr) {
                Ok(expr) => Ok(TypedStmt::Expr(expr)),
                Err(e) => Err(e),
            },
            Stmt::Print(expr) => match TypedExpr::try_from(expr) {
                Ok(expr) => Ok(TypedStmt::Print(expr)),
                Err(e) => Err(e),
            },
            Stmt::Var(token, expr) => match TypedExpr::try_from(expr) {
                Ok(expr) => Ok(TypedStmt::Var(token, expr)),
                Err(e) => Err(e),
            },
        }
    }
}
