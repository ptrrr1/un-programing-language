use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::types::Value;

#[derive(Debug, Default, Clone)]
pub struct Enviroment {
    variables: Rc<RefCell<HashMap<String, Value>>>,
    outer: Option<Rc<RefCell<Enviroment>>>,
}

impl Enviroment {
    pub fn define_var(&self, identifier: &str, val: Value) {
        self.variables
            .borrow_mut()
            .insert(identifier.to_string(), val);
    }

    pub fn get_var_val(&self, identifier: &String) -> Option<Value> {
        if let Some(v) = self.variables.borrow().get(identifier) {
            return Some(v.clone());
        }

        if let Some(outer) = self.outer.as_ref() {
            return outer.borrow().get_var_val(identifier);
        }

        None
    }

    pub fn update_var(&self, identifier: &str, val: Value) -> Option<Value> {
        if self.variables.borrow().contains_key(identifier) {
            return self
                .variables
                .borrow_mut()
                .insert(identifier.to_string(), val);
        }

        if let Some(outer) = self.outer.as_ref() {
            return outer.borrow().update_var(identifier, val);
        }

        // Variable doesn't exist anywhere
        panic!("Undefined variable")
    }

    pub fn set_outer(&mut self, outer: Rc<RefCell<Enviroment>>) {
        self.outer = Some(outer);
    }
}
