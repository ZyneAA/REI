use std::collections::HashMap;

use crate::crux::token::{ Object, Token };
use super::runtime_error::RuntimeError;

pub struct Environment {

    values: HashMap<String, Object>

}

impl Environment {

    pub fn new() -> Self {
        let values: HashMap<String, Object> = HashMap::new();
        Environment { values }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<&Object, RuntimeError<Token>> {

        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap())
        }
        else {
            Err(RuntimeError::UndefinedVariable { token: name.clone() })
        }

    }

}
