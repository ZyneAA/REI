use std::time::{ SystemTime, UNIX_EPOCH };
use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;
use std::thread;
use std::time::{ Duration, Instant };

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

    fn as_any(&self) -> &dyn Any {
        self
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

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct FormatTime;
impl ReiCallable for FormatTime {

    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let now = chrono::Utc::now().to_rfc3339();
        Ok(Object::Str(now))

    }

    fn to_string(&self) -> String {
        "<native_fn>format_time".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct Measure;
impl ReiCallable for Measure {

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        if let Object::Callable(callable) = &arguments[0] {
            let now = Instant::now();
            callable.call(interpreter, &vec![])?;
            let elapsed = now.elapsed().as_secs_f64();
            Ok(Object::Str(format!("{:.6}", elapsed)))
        }
        else {
            Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "Expected a function as argument which return none to measure".to_string(),
            }))
        }

    }

    fn to_string(&self) -> String {
        "<native_fn>measure".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }


}

pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {

    let time_now: Rc<dyn ReiCallable> = Rc::new(TimeNow);
    env.define("_C_time_now".to_string(), Object::Callable(time_now))?;

    let sleep: Rc<dyn ReiCallable> = Rc::new(Sleep);
    env.define("_C_sleep".to_string(), Object::Callable(sleep))?;

    let format_time: Rc<dyn ReiCallable> = Rc::new(FormatTime);
    env.define("_C_format_time".to_string(), Object::Callable(format_time))?;

    let measure: Rc<dyn ReiCallable> = Rc::new(Measure);
    env.define("_C_measure".to_string(), Object::Callable(measure))?;

    Ok(())

}
