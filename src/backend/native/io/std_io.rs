use std::any::Any;
use std::io::{ self, Write };
use std::rc::Rc;

use crate::crux::token::Object;
use crate::backend::interpreter::Interpreter;
use crate::backend::exec_signal::ExecSignal;
use crate::backend::exec_signal::runtime_error::RuntimeError;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::environment::Environment;

#[derive(Clone, Debug)]
pub struct ReadLine;
impl ReiCallable for ReadLine {

    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        print!("");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim_end_matches(&['\n', '\r'][..]).to_string();
                Ok(Object::Str(trimmed))
            }
            Err(_) => Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "Failed to read input from stdin".to_string(),
            })),
        }

    }

    fn to_string(&self) -> String {
        "<native_fn>read_line".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct Read;
impl ReiCallable for Read {

    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim_end_matches(&['\n', '\r'][..]).to_string();
                Ok(Object::Str(trimmed))
            }
            Err(_) => Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "Failed to read input from stdin".to_string(),
            })),
        }

    }

    fn to_string(&self) -> String {
        "<native_fn>read_line".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {

    let read_line: Rc<dyn ReiCallable> = Rc::new(ReadLine);
    env.define("_IO_read_line".to_string(), Object::Callable(read_line))?;

    let read: Rc<dyn ReiCallable> = Rc::new(Read);
    env.define("_IO_read".to_string(), Object::Callable(read))?;

    Ok(())

}

