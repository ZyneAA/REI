use std::{any::Any, cell::RefCell, env, process, rc::Rc, thread, time::Duration};

use crate::backend::environment::Environment;
use crate::backend::exec_signal::{
    runtime_error::{RuntimeError, RuntimeErrorType},
    ExecSignal,
};
use crate::backend::interpreter::Interpreter;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::stack_trace::ExecContext;

use crate::crux::token::Object;

#[derive(Clone, Debug)]
pub struct ProcExit;
impl ReiCallable for ProcExit {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        _context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        if let Some(Object::Number(code)) = arguments.get(0) {
            process::exit(*code as i32);
        }

        let err = RuntimeErrorType::ErrorInNativeFn {
            msg: "expected Number as exit code".to_string(),
        };
        Err(ExecSignal::RuntimeError(RuntimeError::new(err, _context)))
    }

    fn to_string(&self) -> String {
        "<native_fn>_Proc_exit".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ProcPid;
impl ReiCallable for ProcPid {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: &Vec<Object>,
        _context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        Ok(Object::Number(process::id() as f64))
    }

    fn to_string(&self) -> String {
        "<native_fn>_Proc_pid".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ProcSleep;
impl ReiCallable for ProcSleep {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        _context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        if let Some(Object::Number(ms)) = arguments.get(0) {
            thread::sleep(Duration::from_millis(*ms as u64));
            return Ok(Object::Null);
        }

        let err = RuntimeErrorType::ErrorInNativeFn {
            msg: "expected Number (milliseconds)".to_string(),
        };
        Err(ExecSignal::RuntimeError(RuntimeError::new(err, _context)))
    }

    fn to_string(&self) -> String {
        "<native_fn>_Proc_sleep".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ProcCurrentDir;
impl ReiCallable for ProcCurrentDir {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: &Vec<Object>,
        _context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        match env::current_dir() {
            Ok(path) => Ok(Object::Str(path.to_string_lossy().to_string())),
            Err(e) => {
                let err = RuntimeErrorType::ErrorInNativeFn {
                    msg: format!("failed to get current dir: {}", e),
                };
                Err(ExecSignal::RuntimeError(RuntimeError::new(err, _context)))
            }
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>_Proc_current_dir".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ProcSetDir;
impl ReiCallable for ProcSetDir {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        _context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        if let Some(Object::Str(path)) = arguments.get(0) {
            match env::set_current_dir(path) {
                Ok(_) => Ok(Object::Null),
                Err(e) => {
                    let err = RuntimeErrorType::ErrorInNativeFn {
                        msg: format!("failed to set dir: {}", e),
                    };
                    Err(ExecSignal::RuntimeError(RuntimeError::new(err, _context)))
                }
            }
        } else {
            let err = RuntimeErrorType::ErrorInNativeFn {
                msg: "expected Str (path)".to_string(),
            };
            Err(ExecSignal::RuntimeError(RuntimeError::new(err, _context)))
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>_Proc_set_dir".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {
    env.define(
        "_Proc_exit".to_string(),
        Object::Callable(Rc::new(ProcExit)),
    )?;
    env.define("_Proc_pid".to_string(), Object::Callable(Rc::new(ProcPid)))?;
    env.define(
        "_Proc_sleep".to_string(),
        Object::Callable(Rc::new(ProcSleep)),
    )?;
    env.define(
        "_Proc_current_dir".to_string(),
        Object::Callable(Rc::new(ProcCurrentDir)),
    )?;
    env.define(
        "_Proc_set_dir".to_string(),
        Object::Callable(Rc::new(ProcSetDir)),
    )?;

    Ok(())
}
