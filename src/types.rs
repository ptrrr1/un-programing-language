use crate::{
    errors::TypeError,
    parser::Expr,
    tokens::{Token, TokenType},
};

// TODO: Move to own file
#[derive(Debug, Copy, Clone)]
pub enum Types {
    Bool,
    Int,
    Float,
    String,
    Nil,
}

#[derive(Debug, Clone)]
pub enum TValues {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Nil,
}

impl TValues {
    pub fn as_type(self) -> Types {
        match self {
            TValues::Bool(_) => Types::Bool,
            TValues::Int(_) => Types::Int,
            TValues::Float(_) => Types::Float,
            TValues::String(_) => Types::String,
            TValues::Nil => Types::Nil,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypedValue {
    line: (usize, usize),
    type_t: TValues,
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
            TypedExpr::Literal(ref t) => t.type_t.clone().as_type(),
            TypedExpr::Binary { t, .. }
            | TypedExpr::Unary { t, .. }
            | TypedExpr::Grouping { t, .. } => t,
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
                    type_t: typed_value,
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
