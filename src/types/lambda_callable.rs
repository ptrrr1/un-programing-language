use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    enviroment::Enviroment,
    expr::Expr,
    interpreter::Interpreter,
    tokens::{Token, TokenType},
};

use super::{callable::Callable, value::Value};

#[derive(Debug)]
pub struct LambdaCallable {
    params: Vec<Token>,
    body: Rc<Expr>,
    env: Rc<RefCell<Enviroment>>,
}

impl LambdaCallable {
    pub fn new(params: Vec<Token>, body: Rc<Expr>, env: Rc<RefCell<Enviroment>>) -> Self {
        Self { params, body, env }
    }
}

impl Callable for LambdaCallable {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        let mut new_env = Enviroment::default();
        new_env.set_outer(self.env.clone());

        for (param, arg) in self.params.iter().zip(args) {
            if let TokenType::Identifier(s) = &param.token_type {
                new_env.define_var(s, arg)
            }
        }

        let rc_new_env = Rc::new(RefCell::new(new_env));

        self.body.eval(rc_new_env, interpreter)
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn is_variable_arity(&self) -> bool {
        false
    }
}

impl Display for LambdaCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn lambda>")
    }
}
