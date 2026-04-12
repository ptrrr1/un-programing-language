use std::{cell::RefCell, rc::Rc};

use crate::{
    enviroment::Enviroment,
    tokens::{Token, TokenType},
};

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

    pub fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Result<(), &'static str> {
        match self {
            Stmt::Expr(expr) => {
                expr.eval(env)?;
                Ok(())
            }

            Stmt::Print(expr) => {
                println!("{}", expr.eval(env)?);

                Ok(())
            }

            Stmt::Var { target, expr } => match &target.token_type {
                TokenType::Identifier(s) => {
                    let val = expr.eval(env.clone())?;
                    env.borrow_mut().clone().define_var(s, val);
                    dbg!(env);
                    Ok(())
                }
                _ => unreachable!(),
            },

            Stmt::Block(stmts) => {
                let new_env = Rc::new(RefCell::new(Enviroment::default()));
                new_env.borrow_mut().set_outer(env);

                for stmt in stmts {
                    stmt.eval(new_env.clone())?;
                }

                Ok(())
            }

            Stmt::Conditional {
                condition,
                true_branch,
                false_branch,
            } => {
                let c = condition.eval(env.clone())?;
                if c.get_truthyness() {
                    for stmt in true_branch {
                        stmt.eval(env.clone())?;
                    }
                } else if let Some(f) = false_branch {
                    for stmt in f {
                        stmt.eval(env.clone())?;
                    }
                }

                Ok(())
            }

            Stmt::While { condition, stmts } => {
                while condition.eval(env.clone())?.get_truthyness() {
                    for stmt in stmts {
                        stmt.eval(env.clone())?;
                    }
                }
                Ok(())
            }
        }
    }
}
