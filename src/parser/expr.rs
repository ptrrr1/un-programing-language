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
    ExposedFn(Token),
    Conditional {
        condition: Rc<Expr>,
        true_case: Rc<Expr>,
        false_case: Rc<Expr>,
    },
    Call {
        callee: Rc<Expr>,
        paren: Token,
        args: Vec<Expr>,
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

    pub fn conditional(condition: Expr, true_case: Expr, false_case: Expr) -> Expr {
        Expr::Conditional {
            condition: Rc::new(condition),
            true_case: Rc::new(true_case),
            false_case: Rc::new(false_case),
        }
    }

    pub fn callable(callee: Expr, paren: Token, args: Vec<Expr>) -> Expr {
        Expr::Call {
            callee: Rc::new(callee),
            paren,
            args,
        }
    }

    // TODO: Remove panics, add RuntimeError
    pub fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Value {
        match self {
            Expr::Assignment { target, expr } => {
                let val = expr.eval(env.clone());

                match target.as_ref() {
                    Expr::Variable(token) => match &token.token_type {
                        TokenType::Identifier(s) => {
                            env.borrow().clone().update_var(s, val.clone());
                            val
                        }
                        _ => unreachable!(),
                    },
                    _ => panic!("Invalid assignment target"),
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l = left.eval(env.clone());

                match operator.token_type {
                    TokenType::Or => {
                        if l.get_truthyness() {
                            l
                        } else {
                            right.eval(env.clone())
                        }
                    }
                    TokenType::And => {
                        if l.get_truthyness() {
                            right.eval(env)
                        } else {
                            l
                        }
                    }
                    TokenType::EqualEqual => {
                        let r = right.eval(env);

                        Value::Bool(l == r)
                    }
                    TokenType::BangEqual => {
                        let r = right.eval(env);

                        Value::Bool(l != r)
                    }
                    TokenType::Lesser => {
                        let r = right.eval(env);

                        if l.get_type() != r.get_type() {
                            panic!("PartialOrd for Different Types");
                        }

                        Value::Bool(l < r)
                    }
                    TokenType::LesserEqual => {
                        let r = right.eval(env);

                        if l.get_type() != r.get_type() {
                            panic!("PartialOrd for Different Types");
                        }

                        Value::Bool(l <= r)
                    }
                    TokenType::Greater => {
                        let r = right.eval(env);

                        if l.get_type() != r.get_type() {
                            panic!("PartialOrd for Different Types");
                        }

                        Value::Bool(l > r)
                    }
                    TokenType::GreaterEqual => {
                        let r = right.eval(env);

                        if l.get_type() != r.get_type() {
                            panic!("PartialOrd for Different Types");
                        }

                        Value::Bool(l >= r)
                    }
                    TokenType::Plus => {
                        let r = right.eval(env);

                        match (l, r) {
                            (Value::Number(left), Value::Number(right)) => {
                                Value::Number(left + right)
                            }
                            (Value::String(left), Value::String(right)) => {
                                Value::String(left + &right)
                            }
                            _ => panic!(
                                "Invalid Type for Binary Operation 'Addition', Expected only Number or String",
                            ),
                        }
                    }
                    TokenType::Minus => {
                        let r = right.eval(env);

                        match (l, r) {
                            (Value::Number(left), Value::Number(right)) => {
                                Value::Number(left - right)
                            }
                            _ => panic!(
                                "Invalid Type for Binary Operation 'Subtraction', Expected Number",
                            ),
                        }
                    }
                    TokenType::Star => {
                        let r = right.eval(env);

                        match (l, r) {
                            (Value::Number(left), Value::Number(right)) => {
                                Value::Number(left * right)
                            }
                            _ => panic!(
                                "Invalid Type for Binary Operation 'Multiplication', Expected Number",
                            ),
                        }
                    }
                    TokenType::Slash => {
                        let r = right.eval(env);

                        match (l, r) {
                            (Value::Number(left), Value::Number(right)) => {
                                Value::Number(left / right)
                            }
                            _ => {
                                panic!(
                                    "Invalid Type for Binary Operation 'Division', Expected Number"
                                )
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            Expr::Unary { operator, right } => {
                let r = right.eval(env);

                match operator.token_type {
                    TokenType::Minus => match r {
                        Value::Number(v) => Value::Number(-v),
                        _ => panic!("Invalid Type for Unary, Expected Number"),
                    },
                    TokenType::Not => Value::Bool(!r.get_truthyness()),
                    _ => unreachable!(),
                }
            }

            Expr::Grouping(expr) => expr.eval(env),

            Expr::Literal(token) => Value::try_from(token.token_type.clone()).unwrap(),

            Expr::Variable(token) => match token.token_type.clone() {
                TokenType::Identifier(s) => match env.borrow().get_var_val(&s) {
                    Some(v) => v,
                    None => panic!("Undefined Variable"),
                },
                _ => unreachable!(),
            },

            Expr::ExposedFn(token) => match token.token_type.clone() {
                TokenType::ExposedFunction(s) => match env.borrow().get_var_val(&s) {
                    Some(v) => v,
                    None => panic!("Undefined Variable"),
                },
                _ => unreachable!(),
            },
            Expr::Conditional {
                condition,
                true_case,
                false_case,
            } => {
                let c = condition.eval(env.clone());
                if c.get_truthyness() {
                    true_case.eval(env.clone())
                } else {
                    false_case.eval(env.clone())
                }
            }

            Expr::Call {
                callee,
                paren,
                args,
            } => {
                let eval_callee = callee.eval(env.clone());

                let mut eval_args: Vec<Value> = vec![];
                for arg in args {
                    eval_args.push(arg.eval(env.clone()));
                }

                match eval_callee {
                    Value::Callee(f) => {
                        if f.arity() == eval_args.len()
                            || f.arity() >= eval_args.len() && f.is_variable_arity()
                        {
                            return f.call(eval_args);
                        }

                        panic!("Wrong number of arguments") // TODO: Expand err
                    }
                    _ => panic!("Can only call functions"),
                }
            }
        }
    }
}
