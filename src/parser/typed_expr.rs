use crate::{
    errors::typed_parser_errors::{MismatchType, TypeError},
    parser::types::{TValues, Types},
    tokens::{Token, TokenType},
};

use super::expr::Expr;

// TODO: TypedExprResult
// Also It seems I'm losing position information (or at least not preserving)

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
                    Types::Float => match operator.token_type {
                        TokenType::Plus => {
                            TValues::Float(f64::try_from(l).unwrap() + f64::try_from(r).unwrap())
                        }
                        TokenType::Minus => {
                            TValues::Float(f64::try_from(l).unwrap() - f64::try_from(r).unwrap())
                        }
                        TokenType::Slash => {
                            TValues::Float(f64::try_from(l).unwrap() / f64::try_from(r).unwrap())
                        }
                        TokenType::Star => {
                            TValues::Float(f64::try_from(l).unwrap() * f64::try_from(r).unwrap())
                        }
                        _ => unreachable!(),
                    },
                    // Only one operation is possible with Strings
                    Types::String => {
                        let mut s = l.to_string();
                        s.push_str(&r.to_string());
                        TValues::String(s)
                    }
                    Types::Nil => unreachable!(),
                }
            }
            TypedExpr::Unary {
                operator, right, ..
            } => {
                let r = right.eval();

                match operator.token_type {
                    TokenType::Minus => match r {
                        TValues::Float(v) => TValues::Float(-v),
                        _ => unreachable!(),
                    },
                    // Nil, False, Empty String and 0.0 ARE FALSY
                    TokenType::Not => match r {
                        TValues::Bool(v) => TValues::Bool(!v),
                        TValues::Float(_) | TValues::String(_) => {
                            TValues::Bool(!r.get_truthyness())
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

    // TODO: Construct Error from TypeError
    fn try_from(expr: Expr) -> Result<Self, TypeError> {
        match expr {
            Expr::Literal(token) => {
                // Can just unwrap since I know what the expected value is
                let typed_value = TValues::try_from(token.token_type).unwrap();

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
                    TokenType::Minus => match r.get_type() {
                        Types::Float => r.get_type(),
                        _ => {
                            return Err(TypeError::Mismatch {
                                expected: MismatchType::Single(vec![Types::Float]),
                                found: MismatchType::Single(vec![r.get_type()]),
                            });
                        }
                    },
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
                        if !matches!((l.get_type(), r.get_type()), (Types::Float, Types::Float)) {
                            return Err(TypeError::Mismatch {
                                expected: MismatchType::Multiple(vec![(
                                    Types::Float,
                                    Types::Float,
                                )]),
                                found: MismatchType::Multiple(vec![(l.get_type(), r.get_type())]),
                            });
                        }

                        Types::Float
                    }
                    TokenType::Plus => match (l.get_type(), r.get_type()) {
                        (Types::Float, Types::Float) => Types::Float,
                        (Types::String, Types::String) => Types::String,
                        _ => {
                            return Err(TypeError::Mismatch {
                                expected: MismatchType::Multiple(vec![
                                    (Types::Float, Types::Float),
                                    (Types::String, Types::String),
                                ]),
                                found: MismatchType::Multiple(vec![(l.get_type(), r.get_type())]),
                            });
                        }
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
