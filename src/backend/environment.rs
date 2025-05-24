use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::crux::token::{ Object, Token };
use super::exec_signal::ExecSignal;
use super::exec_signal::runtime_error::RuntimeError;

pub type EnvRef = Rc<RefCell<Environment>>;

#[derive(Clone, Debug)]
pub struct Environment {

    pub values: HashMap<String, Object>,
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

    pub fn define(&mut self, name: String, value: Object) -> Result<(), ExecSignal>{

        self.values.insert(name, value);
        Ok(())

    }

    pub fn get(&self, name: &Token) -> Result<Object, ExecSignal> {

        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        }
        else if let Some(ref env) = self.enclosing {
            env.borrow().get(name)
        }
        else {
            Err(ExecSignal::RuntimeError(RuntimeError::UndefinedVariable {
                token: name.clone(),
            }))
        }

    }

    pub fn ancestor(env: &EnvRef, distance: usize) -> EnvRef {

        let mut current = Rc::clone(env);
        for _ in 0..distance {
            let next = current
                .borrow()
                .enclosing
                .as_ref()
                .cloned()
                .expect("No enclosing env at distance");
            current = next;
        }
        current

    }

    pub fn get_at(env: &EnvRef, distance: usize, name: &str) -> Result<Object, ExecSignal> {

        let ancestor_env = Environment::ancestor(env, distance);
        let obj = ancestor_env
            .borrow()
            .values
            .get(name)
            .cloned()
            .expect("Undefined variable.");
        Ok(obj)

    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), ExecSignal> {

        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        }
        else if let Some(ref mut env) = self.enclosing {
            env.borrow_mut().assign(name, value)
        }
        else {
            Err(ExecSignal::RuntimeError(RuntimeError::UndefinedVariable {
            token: name.clone(),
            }))
        }

    }

    pub fn assign_at(env: &EnvRef, distance: usize, name: &Token, value: Object) {

        let ancestor = Environment::ancestor(env, distance);
        ancestor.borrow_mut().values.insert(name.lexeme.clone(), value);

    }


}
