use super::types::Value;

pub trait Callable: std::fmt::Debug {
    fn call(&self, args: Vec<Value>) -> Value;
    fn arity(&self) -> usize;
    fn is_variable_arity(&self) -> bool;
}

pub trait ExposedCallable: Callable {
    fn definition() -> (String, Value);
}
