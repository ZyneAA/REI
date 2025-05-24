use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::interpreter::Interpreter;
use super::environment::Environment;
use super::rei_callable::ReiCallable;
use super::rei_function::ReiFunction;
use super::rei_class::ReiClass;
use super::exec_signal::control_flow::ControlFlow;
use super::exec_signal::ExecSignal;
use super::exec_signal::runtime_error::RuntimeError;
use crate::crux::token::{ Object, Token };

#[derive(Debug, Clone)]
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

        if let Some(method) = self.class.find_method(&name.lexeme) {
            let bind = method.bind(self.clone())?;
            let method: Rc<dyn ReiCallable> = Rc::new(bind);
            return Ok(Object::Callable(method))
        }

        Err(ExecSignal::RuntimeError(RuntimeError::UndefinedProperty{ token: name.clone() }))

    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.fields.insert(name.to_string(), value);
    }

    pub fn call(&self) -> Result<Object, ExecSignal> {
        Ok(Object::Instance(Rc::new(RefCell::new(self.clone()))))
    }

    pub fn to_string(&self) -> String {

        let mut properties = String::new();
        let mut methods = String::new();

        for i in self.fields.keys() {
            let s = format!(" {} ", i);
            properties.push_str(&s);
        }

        for i in self.class.methods.keys() {
            let s = format!(" {}() ", i);
            methods.push_str(&s);
        }

        format!("<Instance of {}>\n  properties --> {}\n  methods --> {}", self.class.to_string(), properties, methods)

    }

}
