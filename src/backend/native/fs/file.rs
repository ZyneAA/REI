use std::fs;
use std::any::Any;
use std::rc::Rc;

use crate::crux::token::Object;
use crate::backend::interpreter::Interpreter;
use crate::backend::exec_signal::{ExecSignal, runtime_error::RuntimeError};
use crate::backend::rei_callable::ReiCallable;
use crate::backend::environment::Environment;

#[derive(Clone, Debug)]
pub struct BuildDir;
impl ReiCallable for BuildDir {

    fn arity(&self) -> usize {
        2
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let path = match arguments.get(0) {
            Some(Object::Str(n)) => n,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected Str".to_string(),
            })),
        };
        let is_recursive = match arguments.get(1) {
            Some(Object::Bool(n)) => n,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected Bool".to_string(),
            })),
        };

        if *is_recursive {
            if let Err(e) = fs::DirBuilder::new().recursive(true).create(path) {
                return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                    msg: format!("Failed to create directory: {}", e),
                }));
            }
        }
        else {
            if let Err(e) = fs::DirBuilder::new().create(path) {
                return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                    msg: format!("Failed to create directory: {}", e),
                }));
            }
        }

        Ok(Object::Null)

    }

    fn to_string(&self) -> String {
        "<native_fn>_FS_build_dir".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}
pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {

    let malloc: Rc<dyn ReiCallable> = Rc::new(BuildDir);
    env.define("_FS_build_dir".to_string(), Object::Callable(malloc))?;

    Ok(())

}
