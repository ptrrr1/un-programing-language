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
        target: Expr,
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
    For {
        identifier: Expr,
        start: Expr,
        end: Expr,
        step: Expr,
        condition: Token,
        stmts: Vec<Stmt>,
    },
}

impl Stmt {
    pub fn expr(expr: Expr) -> Self {
        Self::Expr(expr)
    }

    pub fn print(expr: Expr) -> Self {
        Self::Print(expr)
    }

    pub fn var(target: Expr, expr: Expr) -> Self {
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

    pub fn for_stmt(
        identifier: Expr,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        condition: Token,
        stmts: Vec<Stmt>,
    ) -> Self {
        let s = match step {
            Some(t) => t,
            // TODO: Find the position I guess...
            None => Expr::literal(Token::new(TokenType::Number(1.0), (0, 0))),
        };

        Self::For {
            identifier,
            start,
            end,
            step: s,
            condition,
            stmts,
        }
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

            Stmt::Var { target, expr } => match target {
                Expr::Variable(t) => {
                    if let TokenType::Identifier(s) = &t.token_type {
                        let val = expr.eval(env.clone())?;
                        env.borrow_mut().clone().define_var(s, val);
                    }

                    Ok(())
                }
                _ => Err("Invalid target for variable"),
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

            Stmt::For {
                identifier,
                start,
                end,
                step,
                condition,
                stmts,
            } => {
                let var_decl = Self::var(identifier.clone(), start.clone());

                let st = Self::expr(Expr::assignment(
                    identifier.clone(),
                    Expr::binary(
                        identifier.clone(),
                        Token::new(TokenType::Plus, (0, 0)),
                        step.clone(),
                    ),
                ));

                let mut stmts_cl = stmts.clone();
                stmts_cl.push(st);

                let condition = Expr::binary(identifier.clone(), condition.clone(), end.clone());

                let while_stmt = Self::while_stmt(condition, stmts_cl);

                Self::block(vec![var_decl, while_stmt]).eval(env)
            }
        }
    }
}
