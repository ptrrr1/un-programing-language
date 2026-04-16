use std::fmt::{Debug, Display};

use super::value::Value;

pub trait Callable: Debug + Display {
    fn call(&self, args: Vec<Value>) -> Value;
    fn arity(&self) -> usize;
    fn is_variable_arity(&self) -> bool;
}

pub trait ExposedCallable: Callable {
    fn definition() -> (String, Value);
}
