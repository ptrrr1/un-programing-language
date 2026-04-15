use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    enviroment::Enviroment,
    tokens::{Token, TokenType},
};

use super::{stmt::Stmt, types::Value};

pub trait Callable: Debug + Display {
    fn call(&self, args: Vec<Value>, env: Rc<RefCell<Enviroment>>) -> Value;
    fn arity(&self) -> usize;
    fn is_variable_arity(&self) -> bool;
}

pub trait ExposedCallable: Callable {
    fn definition() -> (String, Value);
}

#[derive(Debug)]
pub struct UnCallable {
    identifier: String,
    params: Vec<Token>,
    body: Rc<Stmt>,
}

impl UnCallable {
    pub fn new(identifier: String, params: Vec<Token>, body: Rc<Stmt>) -> Self {
        Self {
            identifier,
            params,
            body,
        }
    }
}

impl Callable for UnCallable {
    fn call(&self, args: Vec<Value>, env: Rc<RefCell<Enviroment>>) -> Value {
        let mut new_env = Enviroment::default();
        new_env.set_outer(env.clone());

        args.iter()
            .enumerate()
            .for_each(|(i, v)| match &self.params[i].token_type {
                TokenType::Identifier(s) => new_env.define_var(s, v.clone()),
                _ => panic!("Invalid assignment target in function"),
            });

        self.body.eval(Rc::new(RefCell::new(new_env)));

        Value::Nil
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn is_variable_arity(&self) -> bool {
        false
    }
}

impl Display for UnCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.identifier)
    }
}
