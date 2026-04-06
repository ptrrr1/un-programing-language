use crate::errors::typed_parser_errors::TypeError;

use super::{expr::Expr, typed_expr::TypedExpr};

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStatement(Expr),
    PrintStmt(Expr),
}

pub enum TypedStmt {
    ExprStatement(TypedExpr),
    PrintStmt(TypedExpr),
}

impl TypedStmt {
    pub fn eval(&self) {
        match self {
            TypedStmt::ExprStatement(typed_expr) => println!(":: {:?}", typed_expr.eval()),
            TypedStmt::PrintStmt(typed_expr) => {
                println!(";; {:?}", typed_expr.eval().to_string())
            }
        }
    }
}

impl TryFrom<Stmt> for TypedStmt {
    type Error = TypeError;

    fn try_from(value: Stmt) -> Result<Self, Self::Error> {
        match value {
            Stmt::ExprStatement(expr) => match TypedExpr::try_from(expr) {
                Ok(expr) => Ok(TypedStmt::ExprStatement(expr)),
                Err(e) => Err(e),
            },
            Stmt::PrintStmt(expr) => match TypedExpr::try_from(expr) {
                Ok(expr) => Ok(TypedStmt::PrintStmt(expr)),
                Err(e) => Err(e),
            },
        }
    }
}
