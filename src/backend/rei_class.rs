use std::collections::HashMap;

use super::interpreter::Interpreter;
use super::environment::Environment;
use super::rei_callable::ReiCallable;
use super::rei_instance::ReiInstance;
use super::rei_function::ReiFunction;
use super::exec_signal::ExecSignal;
use crate::crux::token::Object;

#[derive(Debug, Clone)]
pub struct ReiClass {

    name: String,
    pub methods: HashMap<String, ReiFunction>

}

impl ReiClass {

    pub fn new(name: String, methods: HashMap<String, ReiFunction>) -> Self {
        ReiClass { name, methods }
    }

    pub fn find_method(&self, name: &str) -> Option<ReiFunction> {

        if let Some(value) = self.methods.get(name) {
            Some(value.clone())
        }
        else {
            None
        }

    }

}

impl ReiCallable for ReiClass {

    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let instance = ReiInstance::new(self.clone());

        let init = self.find_method("init");
        match init {
            Some(i) => {
                i.bind(instance.clone())?.call(interpreter, arguments)?;
            },
            None => {}
        }

        instance.call()

    }

    fn arity(&self) -> usize {

        let init = self.find_method("init");
        match init {
            Some(i) => {
                i.arity()
            },
            None => 0
        }

    }

    fn to_string(&self) -> String {
        format!("{}", self.name)
    }

}

