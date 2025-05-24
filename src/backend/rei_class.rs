use super::interpreter::Interpreter;
use super::environment::Environment;
use super::rei_callable::ReiCallable;
use super::rei_instance::ReiInstance;
use super::exec_signal::ExecSignal;
use crate::crux::token::Object;

#[derive(Debug, Clone)]
pub struct ReiClass {

    name: String

}

impl ReiClass {

    pub fn new(name: String) -> Self {
        ReiClass { name }
    }

}

impl ReiCallable for ReiClass {

    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let instance = ReiInstance::new(self.clone());
        instance.call()

    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        format!("{}", self.name)
    }

}

