use std::any::Any;
use std::fmt::Debug;

use super::exec_signal::ExecSignal;
use super::interpreter::Interpreter;
use crate::crux::token::Object;

pub trait ReiCallable: Debug {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
    ) -> Result<Object, ExecSignal>;
    fn to_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
