use crate::tokens::Token;

use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
}
