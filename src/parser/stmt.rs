use std::{cell::RefCell, rc::Rc};

use crate::{
    enviroment::Enviroment,
    tokens::{Token, TokenType},
};

use super::{expr::Expr, signal::Signal};
use crate::types::{un_callable::UnCallable, value::Value};

#[derive(Debug, Clone)]
pub enum Stmt {
    // TODO: Resolve issues with memory size
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
    For {
        identifier: Token,
        start: Expr,
        end: Expr,
        step: Expr,
        condition: Token,
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
        identifier: Token,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        condition: Token,
        stmts: Vec<Stmt>,
    ) -> Self {
        let s = match step {
            Some(t) => t,
            None if matches!(condition.token_type, TokenType::Greater) => {
                Expr::literal(Token::new(TokenType::Number(-1.0), condition.line))
            }
            None => Expr::literal(Token::new(TokenType::Number(1.0), condition.line)),
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

    // TODO: Remove panics and reprocess Errors at top level of interpreter
    pub fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Signal {
        match self {
            Stmt::Expr(expr) => {
                expr.eval(env);

                Signal::Normal
            }

            Stmt::Print(expr) => {
                println!("{}", expr.eval(env));

                Signal::Normal
            }

            Stmt::Var { target, expr } => {
                if let TokenType::Identifier(s) = &target.token_type {
                    let val = expr.eval(env.clone());
                    env.borrow_mut().define_var(s, val);
                }

                Signal::Normal
            }

            Stmt::Block(stmts) => {
                let new_env = Rc::new(RefCell::new(Enviroment::default()));
                new_env.borrow_mut().set_outer(env.clone());

                for stmt in stmts {
                    let r = stmt.eval(new_env.clone());

                    if !matches!(r, Signal::Normal) {
                        return r;
                    }
                }

                Signal::Normal
            }

            Stmt::Conditional {
                condition,
                true_branch,
                false_branch,
            } => {
                let c = condition.eval(env.clone());
                if c.get_truthyness() {
                    for stmt in true_branch {
                        let r = stmt.eval(env.clone());

                        if !matches!(r, Signal::Normal) {
                            return r;
                        }
                    }
                } else if let Some(f) = false_branch {
                    for stmt in f {
                        let r = stmt.eval(env.clone());

                        if !matches!(r, Signal::Normal) {
                            return r;
                        }
                    }
                }

                Signal::Normal
            }

            Stmt::While { condition, stmts } => {
                'outer: while condition.eval(env.clone()).get_truthyness() {
                    for stmt in stmts {
                        let r = stmt.eval(env.clone());

                        match r {
                            Signal::Return(_) => return r,
                            Signal::Break => break 'outer,
                            // Signal::Continue => continue, // TODO: Continue; doesn't work
                            Signal::Normal => (),
                        }
                    }
                }

                Signal::Normal
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
                    Expr::variable(identifier.clone()),
                    Expr::binary(
                        Expr::variable(identifier.clone()),
                        Token::new(TokenType::Plus, (0, 0)), // TODO: Handle this pos
                        step.clone(),
                    ),
                ));

                let mut stmts_cl = stmts.clone();
                stmts_cl.push(st);

                let condition = Expr::binary(
                    Expr::variable(identifier.clone()),
                    condition.clone(),
                    end.clone(),
                );

                let while_stmt = Self::while_stmt(condition, stmts_cl);

                Self::block(vec![var_decl, while_stmt]).eval(env)
            }

            Stmt::Function {
                identifier,
                params,
                body,
            } => {
                match &identifier.token_type {
                    TokenType::Identifier(s) => {
                        let un_callable =
                            UnCallable::new(s.clone(), params.clone(), body.clone(), env.clone());
                        let val = Value::Callee(Rc::new(un_callable));

                        env.borrow_mut().define_var(s, val);
                    }
                    _ => panic!("Not an identifier for a function"),
                }

                Signal::Normal
            }

            Stmt::Return(expr) => {
                let v = expr.eval(env.clone());

                Signal::Return(v)
            }
            Stmt::Break => Signal::Break,
            // Stmt::Continue => Signal::Continue,
        }
    }
}
