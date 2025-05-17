use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::crux::token::{ Object, Token };
use super::runtime_error::RuntimeError;

pub type EnvRef = Rc<RefCell<Environment>>;

#[derive(Clone, Debug)]
pub struct Environment {

    values: HashMap<String, Object>,
    enclosing: Option<EnvRef>

}

impl Environment {

    pub fn global() -> EnvRef {

        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: None,
        }))

    }

    pub fn from_enclosing(enclosing: EnvRef) -> EnvRef {

        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }))

    }

    pub fn define(&mut self, name: String, value: Object) -> Result<(), RuntimeError<Token>>{

        self.values.insert(name, value);
        Ok(())

    }

    pub fn get(&mut self, name: &Token) -> Result<Object, RuntimeError<Token>> {

        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        }
        else if let Some(ref env) = self.enclosing {
            env.borrow_mut().get(name)
        }
        else {
            Err(RuntimeError::UndefinedVariable {
                token: name.clone(),
            })
        }

    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), RuntimeError<Token>> {

        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        }
        else if let Some(ref mut env) = self.enclosing {
            env.borrow_mut().assign(name, value)
        }
        else {
            Err(RuntimeError::UndefinedVariable {
            token: name.clone(),
            })
        }
    }

}
