use std::collections::HashMap;

use crate::crux::token::{ Object, Token };
use super::runtime_error::RuntimeError;

#[derive(Debug, Clone)]
pub struct Environment {

    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>

}

impl Environment {

    pub fn global(enclosing: Option<Box<Environment>>) -> Self {

        let values: HashMap<String, Object> = HashMap::new();
        Environment { values, enclosing }

    }

    pub fn from_enclosing(enclosing: Environment) -> Self {

        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }

    }

    pub fn define(&mut self, name: String, value: Object) -> Result<(), RuntimeError<Token>>{

        self.values.insert(name, value);
        Ok(())

    }

    pub fn get(&mut self, name: &Token) -> Result<&Object, RuntimeError<Token>> {

        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value)
        }
        else {
            if let Some(ref mut env) = self.enclosing {
                env.get(name)
            }
            else {
                Err(RuntimeError::UndefinedVariable {
                    token: name.clone(),
                })
            }
        }

    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), RuntimeError<Token>> {

        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        }
        else {
            if let Some(ref mut env) = self.enclosing {
                env.assign(name, value)?;
                Ok(())
            }
            else {
                Err(RuntimeError::UndefinedVariable {
                    token: name.clone(),
                })
            }
        }

    }

}
