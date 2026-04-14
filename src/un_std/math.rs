use std::rc::Rc;

use crate::parser::{
    callable::{Callable, ExposedCallable},
    types::Value,
};

#[derive(Debug)]
pub struct Mod;

impl Callable for Mod {
    fn call(&self, args: Vec<Value>) -> Value {
        let mut iter = args.iter();
        let a = iter.next().unwrap();
        let m = iter.next().unwrap();

        match (a, m) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left % right),
            _ => Value::Nil,
        }
    }

    fn arity(&self) -> usize {
        2
    }
}

impl ExposedCallable for Mod {
    fn definition() -> (String, Value) {
        ("@mod".to_string(), Value::Callee(Rc::new(Self)))
    }
}
