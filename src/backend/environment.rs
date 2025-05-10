use std::collections::HashMap;

use crate::crux::token::{ Object, Token };
use super::runtime_error::RuntimeError;

#[derive(Debug)]
pub struct Environment {

    values: HashMap<String, Object>

}

impl Environment {

    pub fn new() -> Self {

        let values: HashMap<String, Object> = HashMap::new();
        Environment { values }

    }

    pub fn define(&mut self, name: String, value: Object) -> Result<(), RuntimeError<Token>>{

        self.values.insert(name, value);
        Ok(())

    }

    pub fn get(&mut self, name: &Token) -> Result<&Object, RuntimeError<Token>> {

        self.values
            .get(&name.lexeme)
            .ok_or(RuntimeError::UndefinedVariable {
                token: name.clone(),
        })

    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), RuntimeError<Token>> {

        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        }
        else {
            Err(RuntimeError::UndefinedVariable {
                token: name.clone(),
            })
        }

    }

}
