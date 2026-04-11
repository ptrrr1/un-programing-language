use std::collections::HashMap;

use crate::parser::types::TValues;

#[derive(Debug, Default)]
pub struct Enviroment {
    variables: HashMap<String, TValues>,
    outer: Option<Box<Enviroment>>,
}

impl Enviroment {
    pub fn define_var(&mut self, identifier: String, val: TValues) {
        self.variables.insert(identifier, val);
    }

    pub fn get_var_val(&self, identifier: &String) -> Option<&TValues> {
        if let Some(outer) = self.outer.as_ref() {
            return outer.get_var_val(identifier);
        }

        self.variables.get(identifier)
    }
}
