use std::collections::HashMap;

use crate::parser::types::TValues;

pub struct Enviroment {
    variables: HashMap<String, TValues>,
}

impl Enviroment {
    pub fn define_var(&mut self, identifier: String, val: TValues) {
        self.variables.insert(identifier, val);
    }

    pub fn get_var_val(&self, identifier: &String) -> Option<&TValues> {
        self.variables.get(identifier)
    }
}
