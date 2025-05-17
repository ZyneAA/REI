use std::fmt::Debug;

use super::interpreter::Interpreter;
use crate::crux::token::{ Object, Token };
use super::runtime_error::RuntimeError;

pub trait ReiCallable: Debug {

    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, RuntimeError<Token>>;
    fn to_string(&self) -> String;

}
