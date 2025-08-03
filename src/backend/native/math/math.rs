use rand::Rng;
use std::any::Any;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

use crate::backend::environment::Environment;
use crate::backend::exec_signal::runtime_error::{RuntimeError, RuntimeErrorType};
use crate::backend::exec_signal::ExecSignal;
use crate::backend::interpreter::Interpreter;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::stack_trace::ExecContext;

use crate::crux::token::Object;

macro_rules! math_fn {
    ($name:ident, $rust_fn:expr) => {
        #[derive(Clone, Debug)]
        struct $name;

        impl ReiCallable for $name {
            fn arity(&self) -> usize {
                1
            }

            fn call(
                &self,
                _: &mut Interpreter,
                args: &Vec<Object>,
                context: Rc<RefCell<ExecContext>>,
            ) -> Result<Object, ExecSignal> {
                if let Object::Number(n) = args[0] {
                    Ok(Object::Number($rust_fn(n)))
                } else {
                    let err_type = RuntimeErrorType::ErrorInNativeFn {
                        msg: "Expected number".to_string(),
                    };
                    Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type, context,
                    )))
                }
            }

            fn to_string(&self) -> String {
                format!("<native_fn>{}", stringify!($name))
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

math_fn!(Sqrt, f64::sqrt);
math_fn!(Cbrt, f64::cbrt);
math_fn!(Abs, f64::abs);
math_fn!(Sign, f64::signum);
math_fn!(Floor, f64::floor);
math_fn!(Ceil, f64::ceil);
math_fn!(Round, f64::round);
math_fn!(Trunc, f64::trunc);
math_fn!(Sin, f64::sin);
math_fn!(Cos, f64::cos);
math_fn!(Tan, f64::tan);
math_fn!(Asin, f64::asin);
math_fn!(Acos, f64::acos);
math_fn!(Atan, f64::atan);
math_fn!(Log, f64::ln);
math_fn!(Log10, f64::log10);

#[derive(Clone, Debug)]
struct Pow;
impl ReiCallable for Pow {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        _: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        match (&args[0], &args[1]) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a.powf(*b))),
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected two numbers".to_string(),
                };
                Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type, context,
                )))
            }
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>pow".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
struct Clamp;
impl ReiCallable for Clamp {
    fn arity(&self) -> usize {
        3
    }

    fn call(
        &self,
        _: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        match (&args[0], &args[1], &args[2]) {
            (Object::Number(val), Object::Number(min), Object::Number(max)) => {
                Ok(Object::Number(val.max(*min).min(*max)))
            }
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected three numbers".to_string(),
                };
                Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type, context,
                )))
            }
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>clamp".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
struct ToRadians;
impl ReiCallable for ToRadians {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        if let Object::Number(deg) = args[0] {
            Ok(Object::Number(deg * PI / 180.0))
        } else {
            let err_type = RuntimeErrorType::ErrorInNativeFn {
                msg: "Expected one numbers".to_string(),
            };
            Err(ExecSignal::RuntimeError(RuntimeError::new(
                err_type, context,
            )))
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>to_radians".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
struct ToDegrees;
impl ReiCallable for ToDegrees {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        if let Object::Number(rad) = args[0] {
            Ok(Object::Number(rad * 180.0 / PI))
        } else {
            let err_type = RuntimeErrorType::ErrorInNativeFn {
                msg: "Expected one numbers".to_string(),
            };
            Err(ExecSignal::RuntimeError(RuntimeError::new(
                err_type, context,
            )))
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>to_degrees".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
struct Random;
impl ReiCallable for Random {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _: &mut Interpreter,
        _: &Vec<Object>,
        _context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        Ok(Object::Number(rand::rng().random::<f64>()))
    }

    fn to_string(&self) -> String {
        "<native_fn>random".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
struct RandomRange;
impl ReiCallable for RandomRange {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        _: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        match (&args[0], &args[1]) {
            (Object::Number(min), Object::Number(max)) => {
                let r = rand::rng().random_range(*min as i64..=*max as i64);
                Ok(Object::Number(r as f64))
            }
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected one numbers".to_string(),
                };
                Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type, context,
                )))
            }
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>random_int".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {
    env.define("_Ma_sqrt".to_string(), Object::Callable(Rc::new(Sqrt)))?;
    env.define("_Ma_cbrt".to_string(), Object::Callable(Rc::new(Cbrt)))?;
    env.define("_Ma_abs".to_string(), Object::Callable(Rc::new(Abs)))?;
    env.define("_Ma_sign".to_string(), Object::Callable(Rc::new(Sign)))?;
    env.define("_Ma_floor".to_string(), Object::Callable(Rc::new(Floor)))?;
    env.define("_Ma_ceil".to_string(), Object::Callable(Rc::new(Ceil)))?;
    env.define("_Ma_round".to_string(), Object::Callable(Rc::new(Round)))?;
    env.define("_Ma_trunc".to_string(), Object::Callable(Rc::new(Trunc)))?;
    env.define("_Ma_sin".to_string(), Object::Callable(Rc::new(Sin)))?;
    env.define("_Ma_cos".to_string(), Object::Callable(Rc::new(Cos)))?;
    env.define("_Ma_tan".to_string(), Object::Callable(Rc::new(Tan)))?;
    env.define("_Ma_asin".to_string(), Object::Callable(Rc::new(Asin)))?;
    env.define("_Ma_acos".to_string(), Object::Callable(Rc::new(Acos)))?;
    env.define("_Ma_atan".to_string(), Object::Callable(Rc::new(Atan)))?;
    env.define("_Ma_log".to_string(), Object::Callable(Rc::new(Log)))?;
    env.define("_Ma_log10".to_string(), Object::Callable(Rc::new(Log10)))?;
    env.define("_Ma_pow".to_string(), Object::Callable(Rc::new(Pow)))?;
    env.define("_Ma_clamp".to_string(), Object::Callable(Rc::new(Clamp)))?;
    env.define(
        "_Ma_to_radians".to_string(),
        Object::Callable(Rc::new(ToRadians)),
    )?;
    env.define(
        "_Ma_to_degrees".to_string(),
        Object::Callable(Rc::new(ToDegrees)),
    )?;
    env.define("_Ma_random".to_string(), Object::Callable(Rc::new(Random)))?;
    env.define(
        "_Ma_random_range".to_string(),
        Object::Callable(Rc::new(RandomRange)),
    )?;

    Ok(())
}
