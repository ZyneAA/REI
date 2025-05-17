
use std::alloc::{alloc, Layout};
use std::rc::Rc;

use crate::crux::token::{Token, Object};
use crate::backend::interpreter::Interpreter;
use crate::backend::runtime_error::RuntimeError;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::environment::Environment;

#[derive(Clone, Debug)]
pub struct ReiMalloc;
impl ReiCallable for ReiMalloc {
    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, RuntimeError<Token>> {
        let size = match arguments.get(0) {
            Some(Object::Number(n)) => *n as usize,
            _ => {
                return Err(RuntimeError::ErrorInNativeFn {
                    msg: "malloc: expected 1 number argument".to_string(),
                });
            }
        };

        let layout = Layout::from_size_align(size, 8).map_err(|e| RuntimeError::ErrorInNativeFn {
            msg: format!("malloc: invalid layout: {}", e),
        })?;

        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err(RuntimeError::ErrorInNativeFn {
                    msg: "malloc: allocation failed".to_string(),
                });
            }

            // We'll return the raw address as a number for now
            let addr = ptr as usize as f64;
            println!("{}", addr);
            Ok(Object::Null)
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>alloc".to_string()
    }
}

pub fn register(env: &mut Environment) -> Result<(), RuntimeError<Token>> {

    let callable: Rc<dyn ReiCallable> = Rc::new(ReiMalloc);
    env.define("_M_alloc".to_string(), Object::Callable(callable))?;

    Ok(())

}
