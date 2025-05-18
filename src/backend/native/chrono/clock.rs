use std::time::{ SystemTime, UNIX_EPOCH };
use std::fmt::Debug;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::crux::token::Object;
use crate::backend::interpreter::Interpreter;
use crate::backend::exec_signal::ExecSignal;
use crate::backend::exec_signal::runtime_error::RuntimeError;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::environment::Environment;

#[derive(Clone, Debug)]
pub struct TimeNow;
impl ReiCallable for TimeNow {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, _interpreter: &mut Interpreter, _arguments: &Vec<Object>) -> Result<Object, ExecSignal> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| (ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn { msg: e.to_string() })))?
            .as_secs_f64();
        Ok(Object::Number(now))
    }
    fn to_string(&self) -> String {
        String::from("<native_fn>time_now")
    }
}

#[derive(Clone, Debug)]
pub struct Sleep;
impl ReiCallable for Sleep {
    fn arity(&self) -> usize {
        1
    }
    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {
        let duration = match &arguments[0] {
            Object::Number(ms) => *ms,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "Expected number (milliseconds) as argument to <sleep>".to_string(),
            })),
        };

        thread::sleep(Duration::from_millis(duration as u64));
        Ok(Object::Null)
    }
    fn to_string(&self) -> String {
        String::from("<native_fn>sleep")
    }
}

pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {

    let callable: Rc<dyn ReiCallable> = Rc::new(TimeNow);
    env.define("_C_time_now".to_string(), Object::Callable(callable))?;

    let sleep_fn: Rc<dyn ReiCallable> = Rc::new(Sleep);
    env.define("_C_sleep".to_string(), Object::Callable(sleep_fn))?;

    Ok(())

}
