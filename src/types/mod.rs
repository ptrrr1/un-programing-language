pub mod callable;
pub mod lambda_callable;
pub mod un_callable;
pub mod value;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Types {
    Bool,
    Number,
    String,
    Callable,
    Nil,
}
