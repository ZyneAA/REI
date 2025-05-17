use std::time::{ SystemTime, UNIX_EPOCH };
use std::fmt::Debug;
use std::rc::Rc;

use crate::crux::token::{ Token, Object };
use crate::backend::interpreter::Interpreter;
use crate::backend::runtime_error::RuntimeError;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::environment::Environment;

#[derive(Clone, Debug)]
pub struct TimeNow;
impl ReiCallable for TimeNow {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, RuntimeError<Token>> {
        Ok(self.time_now().unwrap())
    }
}
impl TimeNow {
    fn time_now(&self) -> Result<Object, String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs_f64();
        Ok(Object::Number(now))
    }
}

pub fn register(env: &mut Environment) -> Result<(), RuntimeError<Token>> {

    let time_now = TimeNow;
    let callable: Rc<dyn ReiCallable> = Rc::new(time_now.clone());
    let callable = Object::Callable(callable);
    env.define("time_now".to_string(), callable)

}
