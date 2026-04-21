use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::types::value::Value;

#[derive(Debug, Default, Clone)]
pub struct Enviroment {
    variables: Rc<RefCell<HashMap<String, Value>>>,
    outer: Option<Rc<RefCell<Enviroment>>>,
}

impl Enviroment {
    pub fn with_outer(env: Enviroment) -> Self {
        Enviroment {
            variables: Rc::new(RefCell::new(HashMap::new())),
            outer: Some(Rc::new(RefCell::new(env))),
        }
    }

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

    pub fn get_at(env: Rc<RefCell<Enviroment>>, identifier: &str, depth: usize) -> Value {
        let env = Self::ancestor(env, depth);
        env.borrow()
            .variables
            .borrow()
            .get(identifier)
            .unwrap()
            .clone() // TODO: Reexamine again...
    }

    pub fn define_at(env: Rc<RefCell<Enviroment>>, identifier: &str, v: Value, depth: usize) {
        let env = Self::ancestor(env, depth);
        env.borrow()
            .variables
            .borrow_mut()
            .insert(identifier.to_string(), v);
    }

    fn ancestor(env: Rc<RefCell<Enviroment>>, depth: usize) -> Rc<RefCell<Enviroment>> {
        let mut cur = env;

        for _ in 0..depth {
            let next = {
                let borrowed = cur.borrow();
                borrowed.outer.clone()
            };

            // TODO: Fix
            cur = next.expect("Idk it's late");
        }

        cur
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

    pub fn outer(&self) -> Option<&Rc<RefCell<Enviroment>>> {
        self.outer.as_ref()
    }
}
