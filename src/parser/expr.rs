use std::{cell::RefCell, rc::Rc};

use crate::{
    enviroment::Enviroment,
    tokens::{Token, TokenType},
};

use super::types::Value;

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
    Conditional {
        condition: Rc<Expr>,
        true_case: Rc<Expr>,
        false_case: Rc<Expr>,
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

    pub fn conditional(condition: Expr, true_case: Expr, false_case: Expr) -> Expr {
        Expr::Conditional {
            condition: Rc::new(condition),
            true_case: Rc::new(true_case),
            false_case: Rc::new(false_case),
        }
    }

    pub fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Result<Value, &'static str> {
        match self {
            Expr::Assignment { target, expr } => {
                let val = expr.eval(env.clone())?;

                match target.as_ref() {
                    Expr::Variable(token) => match &token.token_type {
                        TokenType::Identifier(s) => {
                            env.borrow().clone().update_var(s, val.clone())?;
                            Ok(val)
                        }
                        _ => unreachable!(),
                    },
                    _ => Err("Invalid assignment target"),
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l = left.eval(env.clone())?;

                match operator.token_type {
                    TokenType::Or => {
                        if l.get_truthyness() {
                            Ok(l)
                        } else {
                            let r = right.eval(env.clone())?;
                            Ok(r)
                        }
                    }
                    TokenType::And => {
                        if l.get_truthyness() {
                            let r = right.eval(env)?;
                            Ok(r)
                        } else {
                            Ok(l)
                        }
                    }
                    TokenType::EqualEqual => {
                        let r = right.eval(env)?;

                        Ok(Value::Bool(l == r))
                    }
                    TokenType::BangEqual => {
                        let r = right.eval(env)?;

                        Ok(Value::Bool(l != r))
                    }
                    TokenType::Lesser => {
                        let r = right.eval(env)?;

                        if l.get_type() != r.get_type() {
                            return Err("PartialOrd for Different Types");
                        }

                        Ok(Value::Bool(l < r))
                    }
                    TokenType::LesserEqual => {
                        let r = right.eval(env)?;

                        if l.get_type() != r.get_type() {
                            return Err("PartialOrd for Different Types");
                        }

                        Ok(Value::Bool(l <= r))
                    }
                    TokenType::Greater => {
                        let r = right.eval(env)?;

                        if l.get_type() != r.get_type() {
                            return Err("PartialOrd for Different Types");
                        }

                        Ok(Value::Bool(l > r))
                    }
                    TokenType::GreaterEqual => {
                        let r = right.eval(env)?;

                        if l.get_type() != r.get_type() {
                            return Err("PartialOrd for Different Types");
                        }

                        Ok(Value::Bool(l >= r))
                    }
                    TokenType::Plus => {
                        let r = right.eval(env)?;

                        match (l, r) {
                            (Value::Number(left), Value::Number(right)) => {
                                Ok(Value::Number(left + right))
                            }
                            (Value::String(left), Value::String(right)) => {
                                Ok(Value::String(left + &right))
                            }
                            _ => Err(
                                "Invalid Type for Binary Operation 'Addition', Expected only Number or String",
                            ),
                        }
                    }
                    TokenType::Minus => {
                        let r = right.eval(env)?;

                        match (l, r) {
                            (Value::Number(left), Value::Number(right)) => {
                                Ok(Value::Number(left - right))
                            }
                            _ => Err(
                                "Invalid Type for Binary Operation 'Subtraction', Expected Number",
                            ),
                        }
                    }
                    TokenType::Star => {
                        let r = right.eval(env)?;

                        match (l, r) {
                            (Value::Number(left), Value::Number(right)) => {
                                Ok(Value::Number(left * right))
                            }
                            _ => Err(
                                "Invalid Type for Binary Operation 'Multiplication', Expected Number",
                            ),
                        }
                    }
                    TokenType::Slash => {
                        let r = right.eval(env)?;

                        match (l, r) {
                            (Value::Number(left), Value::Number(right)) => {
                                Ok(Value::Number(left / right))
                            }
                            _ => {
                                Err("Invalid Type for Binary Operation 'Division', Expected Number")
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            Expr::Unary { operator, right } => {
                let r = right.eval(env)?;

                match operator.token_type {
                    TokenType::Minus => match r {
                        Value::Number(v) => Ok(Value::Number(-v)),
                        _ => Err("Invalid Type for Unary, Expected Number"),
                    },
                    TokenType::Not => Ok(Value::Bool(!r.get_truthyness())),
                    _ => unreachable!(),
                }
            }

            Expr::Grouping(expr) => expr.eval(env),

            Expr::Literal(token) => Value::try_from(token.token_type.clone()),

            Expr::Variable(token) => match token.token_type.clone() {
                TokenType::Identifier(s) => match env.borrow().get_var_val(&s) {
                    Some(v) => Ok(v),
                    None => Err("Undefined Variable"),
                },
                _ => unreachable!(),
            },

            Expr::Conditional {
                condition,
                true_case,
                false_case,
            } => {
                let c = condition.eval(env.clone())?;
                if c.get_truthyness() {
                    true_case.eval(env.clone())
                } else {
                    false_case.eval(env.clone())
                }
            }
        }
    }
}
