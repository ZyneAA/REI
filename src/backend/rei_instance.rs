use super::interpreter::Interpreter;
use super::environment::Environment;
use super::rei_callable::ReiCallable;
use super::rei_class::ReiClass;
use super::exec_signal::control_flow::ControlFlow;
use super::exec_signal::ExecSignal;
use crate::crux::token::Object;

#[derive(Debug)]
pub struct ReiInstance {

    class: ReiClass

}

impl ReiInstance {

    pub fn new(class: ReiClass) -> Self {
        ReiInstance { class }
    }

}

impl ReiCallable for ReiInstance {

    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let value = Object::Str(self.to_string());
        Ok(value)

    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        format!("<instance of {}>", self.class.to_string())
    }

}
