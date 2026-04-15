// use std::{cell::RefCell, rc::Rc};

// use crate::{
//     enviroment::Enviroment,
//     parser::{
//         callable::{Callable, ExposedCallable},
//         types::Value,
//     },
// };

// pub fn math_globals() -> Enviroment {
//     let math_globals = Enviroment::default();

//     let (name, ret_val) = Mod::definition();
//     math_globals.define_var(&name, ret_val);

//     math_globals
// }

// #[derive(Debug)]
// pub struct Mod;

// impl Callable for Mod {
//     fn call(&self, args: Vec<Value>, _env: Rc<RefCell<Enviroment>>) -> Value {
//         let mut iter = args.iter();
//         let a = iter.next().unwrap();
//         let m = iter.next().unwrap();

//         match (a, m) {
//             (Value::Number(left), Value::Number(right)) => Value::Number(left % right),
//             _ => Value::Nil,
//         }
//     }

//     fn arity(&self) -> usize {
//         2
//     }

//     fn is_variable_arity(&self) -> bool {
//         false
//     }
// }

// impl ExposedCallable for Mod {
//     fn definition() -> (String, Value) {
//         ("@mod".to_string(), Value::Callee(Rc::new(Self)))
//     }
// }
