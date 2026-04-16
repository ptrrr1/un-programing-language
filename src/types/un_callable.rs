use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    enviroment::Enviroment,
    parser::{signal::Signal, stmt::Stmt},
    tokens::{Token, TokenType},
};

use super::{callable::Callable, value::Value};

#[derive(Debug)]
pub struct UnCallable {
    identifier: String,
    params: Vec<Token>,
    body: Vec<Stmt>,
    env: Rc<RefCell<Enviroment>>,
}

impl UnCallable {
    pub fn new(
        identifier: String,
        params: Vec<Token>,
        body: Vec<Stmt>,
        env: Rc<RefCell<Enviroment>>,
    ) -> Self {
        Self {
            identifier,
            params,
            body,
            env,
        }
    }
}

impl Callable for UnCallable {
    fn call(&self, args: Vec<Value>) -> Value {
        let mut new_env = Enviroment::default();
        new_env.set_outer(self.env.clone());

        for (param, arg) in self.params.iter().zip(args) {
            if let TokenType::Identifier(s) = &param.token_type {
                new_env.define_var(s, arg)
            }
        }

        let rc_new_env = Rc::new(RefCell::new(new_env));
        for stmt in &self.body {
            match stmt.eval(rc_new_env.clone()) {
                Signal::Normal => (),
                Signal::Return(value) => return value,
                Signal::Break | Signal::Continue => panic!("Handle Err"), // TODO: Handler Err
            };
        }

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
