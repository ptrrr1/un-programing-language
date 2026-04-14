use crate::{
    enviroment::Enviroment,
    parser::{
        callable::{Callable, ExposedCallable},
        types::Value,
    },
};
use std::rc::Rc;

pub mod math;

pub fn globals() -> Enviroment {
    let globals = Enviroment::default();

    let (name, ret_val) = Print::definition();
    globals.define_var(&name, ret_val);

    globals
}

#[derive(Debug)]
pub struct Print;

impl Callable for Print {
    fn call(&self, args: Vec<Value>) -> Value {
        for arg in args {
            print!("{}", arg);
        }
        println!();

        Value::Nil
    }

    fn arity(&self) -> usize {
        254
    }

    fn is_variable_arity(&self) -> bool {
        true
    }
}

impl ExposedCallable for Print {
    fn definition() -> (String, Value) {
        ("@print".to_string(), Value::Callee(Rc::new(Self)))
    }
}
