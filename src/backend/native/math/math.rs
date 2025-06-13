use std::any::Any;
use std::f64::consts::PI;
use std::rc::Rc;
use rand::Rng;

use crate::crux::token::Object;
use crate::backend::interpreter::Interpreter;
use crate::backend::exec_signal::ExecSignal;
use crate::backend::exec_signal::runtime_error::RuntimeError;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::environment::Environment;

macro_rules! math_fn {

    ($name:ident, $rust_fn:expr) => {
        #[derive(Clone, Debug)]
        struct $name;

        impl ReiCallable for $name {

            fn arity(&self) -> usize { 1 }

            fn call(&self, _: &mut Interpreter, args: &Vec<Object>) -> Result<Object, ExecSignal> {

                if let Object::Number(n) = args[0] {
                    Ok(Object::Number($rust_fn(n)))
                }
                else {
                    Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                        msg: "Expected number".to_string(),
                    }))
                }

            }

            fn to_string(&self) -> String { format!("<native_fn>{}" , stringify!($name)) }

            fn as_any(&self) -> &dyn Any { self }

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

    fn call(&self, _: &mut Interpreter, args: &Vec<Object>) -> Result<Object, ExecSignal> {

        match (&args[0], &args[1]) {
            (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a.powf(*b))),
            _ => Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "Expected two numbers".to_string(),
            }))
        }

    }

    fn to_string(&self) -> String {
        "<native_fn>pow".to_string()
    }

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Clone, Debug)]
struct Clamp;
impl ReiCallable for Clamp {

    fn arity(&self) -> usize {
        3
    }

    fn call(&self, _: &mut Interpreter, args: &Vec<Object>) -> Result<Object, ExecSignal> {

        match (&args[0], &args[1], &args[2]) {
            (Object::Number(val), Object::Number(min), Object::Number(max)) => Ok(Object::Number(val.max(*min).min(*max))),
            _ => Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "Expected three numbers".to_string(),
            }))
        }

    }

    fn to_string(&self) -> String {
        "<native_fn>clamp".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}
