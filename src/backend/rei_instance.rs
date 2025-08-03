use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::exec_signal::runtime_error::{RuntimeError, RuntimeErrorType};
use super::exec_signal::ExecSignal;

use super::rei_callable::ReiCallable;

use super::rei_class::ReiClass;

use crate::crux::token::{Object, Token};

use crate::backend::stack_trace::ExecContext;

#[derive(Debug, Clone)]
pub struct ReiInstance {
    pub class: Rc<ReiClass>,
    pub fields: Rc<RefCell<HashMap<String, Object>>>,
    pub context: Rc<RefCell<ExecContext>>,
}

impl ReiInstance {
    pub fn new(class: ReiClass, context: Rc<RefCell<ExecContext>>) -> Self {
        let class = Rc::new(class);
        ReiInstance {
            class,
            fields: Rc::new(RefCell::new(HashMap::new())),
            context,
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, ExecSignal> {
        if let Some(value) = self.fields.borrow().get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(method) = self.class.find_method(&name.lexeme) {
            let bind = method.bind(self.clone(), self.context.clone())?;
            let method: Rc<dyn ReiCallable> = Rc::new(bind);
            return Ok(Object::Callable(method));
        }

        let err_type = RuntimeErrorType::UndefinedProperty {
            token: name.clone(),
        };
        Err(ExecSignal::RuntimeError(RuntimeError::new(
            err_type,
            self.context.clone(),
        )))
    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.fields.borrow_mut().insert(name.to_string(), value);
    }

    pub fn call(&self) -> Result<Object, ExecSignal> {
        Ok(Object::Instance(Rc::new(RefCell::new(self.clone()))))
    }

    pub fn to_string(&self) -> String {
        let mut properties = String::new();
        let mut methods = String::new();
        let mut static_methods = String::new();

        for i in self.fields.borrow().keys() {
            let s = format!(" {} ", i);
            properties.push_str(&s);
        }

        for i in self.class.methods.keys() {
            let s = format!(" {}() ", i);
            methods.push_str(&s);
        }

        for i in self.class.static_methods.keys() {
            let s = format!(" {}() ", i);
            static_methods.push_str(&s);
        }

        format!(
            "<Instance of {}>\n  properties --> {}\n  static methods --> {}\n  methods --> {}",
            self.class.to_string(),
            properties,
            static_methods,
            methods
        )
    }
}
