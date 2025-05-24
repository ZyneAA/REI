use std::collections::HashMap;
use std::rc::Rc;

use super::interpreter::Interpreter;
use super::environment::Environment;
use super::rei_callable::ReiCallable;
use super::rei_class::ReiClass;
use super::exec_signal::control_flow::ControlFlow;
use super::exec_signal::ExecSignal;
use super::exec_signal::runtime_error::RuntimeError;
use crate::crux::token::{ Object, Token };

#[derive(Debug)]
pub struct ReiInstance {

    class: Rc<ReiClass>,
    pub fields: HashMap<String, Object>,

}

impl ReiInstance {

    pub fn new(class: ReiClass) -> Self {
        let class = Rc::new(class);
        ReiInstance {
            class,
            fields: HashMap::new()
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, ExecSignal> {

        if let Some(value) = self.fields.get(&name.lexeme) {
            return Ok(value.clone());
        }

       // if let Some(method) = self.class.find_method(&name.lexeme) {
       //     return Ok(Object::Callable(Rc::new(method.bind(Rc::new(RefCell::new(self.clone()))))));
       // }

        Err(ExecSignal::RuntimeError(RuntimeError::UndefinedProperty{ token: name.clone() }))

    }

    pub fn call(&self) -> Result<Object, ExecSignal> {

        let obj = Object::Str(self.to_string());
        Ok(obj)

    }

    pub fn to_string(&self) -> String {
        format!("<Instance of {}>", self.class.to_string())
    }

}
