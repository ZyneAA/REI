use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use super::exec_signal::ExecSignal;

use super::interpreter::Interpreter;

use crate::crux::token::Object;

use crate::backend::stack_trace::ExecContext;

pub trait ReiCallable: Debug {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal>;
    fn to_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
