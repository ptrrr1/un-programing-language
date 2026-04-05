use crate::{
    errors::typed_parser_errors::TypeError,
    parser::types::{TValues, Types},
    tokens::{Token, TokenType},
};

use super::expr::Expr;

#[derive(Debug, Clone)]
pub struct TypedValue {
    line: (usize, usize),
    t_val: TValues,
}

#[derive(Debug, Clone)]
pub enum TypedExpr {
    Binary {
        left: Box<TypedExpr>,
        operator: Token,
        right: Box<TypedExpr>,
        t: Types,
    },
    Unary {
        operator: Token,
        right: Box<TypedExpr>,
        t: Types,
    },
    Literal(TypedValue),
    Grouping {
        inner: Box<TypedExpr>,
        t: Types,
    },
}

impl TypedExpr {
    pub fn get_type(&self) -> Types {
        match *self {
            TypedExpr::Literal(ref t) => t.t_val.clone().as_type(),
            TypedExpr::Binary { t, .. }
            | TypedExpr::Unary { t, .. }
            | TypedExpr::Grouping { t, .. } => t,
        }
    }

    pub fn eval(&self) -> TValues {
        match self {
            TypedExpr::Binary {
                left,
                operator,
                right,
                t,
            } => {
                let l = left.eval();
                let r = right.eval();

                match t {
                    Types::Bool => match operator.token_type {
                        TokenType::EqualEqual => TValues::Bool(l == r),
                        TokenType::BangEqual => TValues::Bool(l != r),
                        TokenType::Lesser => TValues::Bool(l < r),
                        TokenType::LesserEqual => TValues::Bool(l <= r),
                        TokenType::Greater => TValues::Bool(l > r),
                        TokenType::GreaterEqual => TValues::Bool(l >= r),
                        _ => unreachable!(),
                    },
                    Types::Int => match operator.token_type {
                        TokenType::Plus => {
                            TValues::Int(i32::try_from(l).unwrap() + i32::try_from(r).unwrap())
                        }
                        TokenType::Minus => {
                            TValues::Int(i32::try_from(l).unwrap() - i32::try_from(r).unwrap())
                        }
                        TokenType::Star => {
                            TValues::Int(i32::try_from(l).unwrap() * i32::try_from(r).unwrap())
                        }
                        _ => unreachable!(),
                    },
                    Types::Float => match operator.token_type {
                        TokenType::Plus => {
                            TValues::Float(f32::try_from(l).unwrap() + f32::try_from(r).unwrap())
                        }
                        TokenType::Minus => {
                            TValues::Float(f32::try_from(l).unwrap() - f32::try_from(r).unwrap())
                        }
                        TokenType::Slash => {
                            TValues::Float(f32::try_from(l).unwrap() / f32::try_from(r).unwrap())
                        }
                        TokenType::Star => {
                            TValues::Float(f32::try_from(l).unwrap() * f32::try_from(r).unwrap())
                        }
                        _ => unreachable!(),
                    },
                    // Only one operation is possible with Strings
                    Types::String => TValues::String(format!("{}{}", l.to_string(), r.to_string())),
                    Types::Nil => unreachable!(),
                }
            }
            TypedExpr::Unary {
                operator, right, ..
            } => {
                let r = right.eval();

                match operator.token_type {
                    TokenType::Minus => match r {
                        TValues::Int(v) => TValues::Int(-v),
                        TValues::Float(v) => TValues::Float(-v),
                        _ => unreachable!(),
                    },
                    // TODO: Decide if i want this configuration:
                    // everything is truthy except for Nil and False
                    // Ideally: 0 and "" (empty string) are falsy
                    TokenType::Not => match r {
                        TValues::Bool(v) => TValues::Bool(!v),
                        TValues::Int(_) | TValues::Float(_) | TValues::String(_) => {
                            TValues::Bool(false)
                        }
                        TValues::Nil => TValues::Bool(true),
                    },
                    _ => unreachable!(),
                }
            }
            TypedExpr::Literal(typed_value) => typed_value.t_val.clone(),
            TypedExpr::Grouping { inner, .. } => inner.eval(),
        }
    }
}

impl TryFrom<Expr> for TypedExpr {
    type Error = TypeError;

    fn try_from(expr: Expr) -> Result<Self, TypeError> {
        match expr {
            Expr::Literal(token) => {
                let typed_value = match token.token_type {
                    TokenType::True => TValues::Bool(true),
                    TokenType::False => TValues::Bool(false),
                    TokenType::NumberInt(v) => TValues::Int(v),
                    TokenType::NumberFloat(v) => TValues::Float(v),
                    TokenType::String(v) => TValues::String(v),
                    TokenType::Nil => TValues::Nil,
                    // TODO: Missing identifier and ExposedFunction
                    _ => unreachable!(),
                };

                Ok(TypedExpr::Literal(TypedValue {
                    line: token.line,
                    t_val: typed_value,
                }))
            }
            Expr::Grouping(expr) => {
                let inner = TypedExpr::try_from(*expr)?;

                Ok(TypedExpr::Grouping {
                    t: inner.get_type(),
                    inner: Box::new(inner),
                })
            }
            Expr::Unary { operator, right } => {
                let r = TypedExpr::try_from(*right)?;

                let t = match operator.token_type {
                    TokenType::Minus => {
                        match r.get_type() {
                            Types::Int | Types::Float => r.get_type(),
                            _ => return Err(TypeError::Mismatch), // TODO: add message for expected an found
                        }
                    }
                    TokenType::Not => Types::Bool,
                    _ => unreachable!(),
                };

                Ok(TypedExpr::Unary {
                    operator,
                    right: Box::new(r),
                    t,
                })
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l = TypedExpr::try_from(*left)?;
                let r = TypedExpr::try_from(*right)?;

                let t = match operator.token_type {
                    TokenType::Lesser
                    | TokenType::LesserEqual
                    | TokenType::Greater
                    | TokenType::GreaterEqual
                    | TokenType::EqualEqual
                    | TokenType::BangEqual => Types::Bool,
                    TokenType::Minus | TokenType::Slash | TokenType::Star => {
                        if !matches!(l.get_type(), Types::Int | Types::Float)
                            || !matches!(r.get_type(), Types::Int | Types::Float)
                        {
                            return Err(TypeError::Mismatch);
                        }

                        if matches!(l.get_type(), Types::Int)
                            && matches!(r.get_type(), Types::Int)
                            && !matches!(operator.token_type, TokenType::Slash)
                        {
                            Types::Int
                        } else {
                            Types::Float
                        }
                    }
                    TokenType::Plus => match l.get_type() {
                        Types::Int if matches!(r.get_type(), Types::Int) => Types::Int,
                        Types::Int if matches!(r.get_type(), Types::Float) => Types::Float,
                        Types::Float => Types::Float,
                        Types::String if matches!(r.get_type(), Types::String) => Types::String,
                        _ => return Err(TypeError::Mismatch),
                    },
                    _ => unreachable!(),
                };

                Ok(TypedExpr::Binary {
                    left: Box::new(l),
                    operator,
                    right: Box::new(r),
                    t,
                })
            }
        }
    }
}
