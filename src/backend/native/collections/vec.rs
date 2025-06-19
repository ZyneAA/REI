use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

use crate::crux::token::Object;
use crate::backend::interpreter::Interpreter;
use crate::backend::exec_signal::{ExecSignal, runtime_error::RuntimeError};
use crate::backend::rei_callable::ReiCallable;
use crate::backend::environment::Environment;

#[derive(Clone, Debug)]
pub struct NewVec;
impl ReiCallable for NewVec {

    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let vec = Rc::new(RefCell::new(Vec::new()));
        Ok(Object::Vec(vec))

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_new_vec".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct NewVecSized;
impl ReiCallable for NewVecSized {

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let size = match arguments.get(0) {
            Some(Object::Number(n)) => *n as usize,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected number".to_string(),
            })),
        };

        let vec = Rc::new(RefCell::new(Vec::with_capacity(size)));
        Ok(Object::Vec(vec))

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_new_vec_sized".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct PushToVec;
impl ReiCallable for PushToVec {

    fn arity(&self) -> usize {
        2
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let vec_ref = match arguments.get(0) {
            Some(Object::Vec(v)) => v,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected a Vec".to_string(),
            })),
        };

        let obj = match arguments.get(1) {
            Some(obj) => obj,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected an Object".to_string(),
            })),
        };

        vec_ref.borrow_mut().push(obj.clone());
        Ok(Object::Null)

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_push_to_vec".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct PopFromVec;
impl ReiCallable for PopFromVec {

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let vec_ref = match arguments.get(0) {
            Some(Object::Vec(v)) => v,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected a Vec".to_string(),
            })),
        };

        vec_ref.borrow_mut().pop();
        Ok(Object::Null)

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_pop_from_vec".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct AppendToVec;
impl ReiCallable for AppendToVec {

    fn arity(&self) -> usize {
        2
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let first_vec_ref = match arguments.get(0) {
            Some(Object::Vec(v)) => v,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected a Vec as first param".to_string(),
            })),
        };

        let second_vec_ref = match arguments.get(1) {
            Some(Object::Vec(v)) => v,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected a Vec as second param".to_string(),
            })),
        };

        let mut second_borrow = second_vec_ref.borrow_mut();
        let mut first_borrow = first_vec_ref.borrow_mut();

        first_borrow.append(&mut second_borrow);
        Ok(Object::Null)

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_append_to_vec".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct ClearVec;
impl ReiCallable for ClearVec {

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let vec_ref = match arguments.get(0) {
            Some(Object::Vec(v)) => v,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected a Vec".to_string(),
            })),
        };

        vec_ref.borrow_mut().clear();
        Ok(Object::Null)

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_clear_vec".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct VecLen;
impl ReiCallable for VecLen {

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let vec_ref = match arguments.get(0) {
            Some(Object::Vec(v)) => v,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected a Vec".to_string(),
            })),
        };

        let len = vec_ref.borrow().len();
        Ok(Object::Number(len as f64))

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_vec_len".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct VecEmptyCheck;
impl ReiCallable for VecEmptyCheck {

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let vec_ref = match arguments.get(0) {
            Some(Object::Vec(v)) => v,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected a Vec".to_string(),
            })),
        };

        let is_empty = vec_ref.borrow().is_empty();
        Ok(Object::Bool(is_empty))

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_vec_is_empty".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct SplitVec;
impl ReiCallable for SplitVec {

    fn arity(&self) -> usize {
        2
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let vec_ref = match arguments.get(0) {
            Some(Object::Vec(v)) => v,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected a Vec".to_string(),
            })),
        };

        let place = match arguments.get(1) {
            Some(Object::Number(n)) => *n as usize,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected number".to_string(),
            })),
        };

        if place > vec_ref.borrow().len() {
            return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: format!("split index {} out of bounds", place),
            }));
        }

        let mut vec_ref = vec_ref.borrow_mut();
        let vec = Rc::new(RefCell::new(vec_ref.split_off(place)));
        Ok(Object::Vec(vec))

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_split_vec".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Clone, Debug)]
pub struct VecGet;
impl ReiCallable for VecGet {

    fn arity(&self) -> usize {
        2
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let vec_ref = match arguments.get(0) {
            Some(Object::Vec(v)) => v,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected a Vec".to_string(),
            })),
        };

        let place = match arguments.get(1) {
            Some(Object::Number(n)) => *n as usize,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "expected number".to_string(),
            })),
        };

        if place > vec_ref.borrow().len() {
            return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: format!("split index {} out of bounds", place),
            }));
        }

        let val = vec_ref.borrow().get(place).unwrap().clone();
        Ok(val)

    }

    fn to_string(&self) -> String {
        "<native_fn>_Co_vec_get".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {

    env.define("_Co_new_vec".to_string(), Object::Callable(Rc::new(NewVec)))?;
    env.define("_Co_new_vec_sized".to_string(), Object::Callable(Rc::new(NewVecSized)))?;
    env.define("_Co_push_to_vec".to_string(), Object::Callable(Rc::new(PushToVec)))?;
    env.define("_Co_pop_from_vec".to_string(), Object::Callable(Rc::new(PopFromVec)))?;
    env.define("_Co_append_to_vec".to_string(), Object::Callable(Rc::new(AppendToVec)))?;
    env.define("_Co_clear_vec".to_string(), Object::Callable(Rc::new(ClearVec)))?;
    env.define("_Co_vec_len".to_string(), Object::Callable(Rc::new(VecLen)))?;
    env.define("_Co_vec_is_empty".to_string(), Object::Callable(Rc::new(VecEmptyCheck)))?;
    env.define("_Co_split_vec".to_string(), Object::Callable(Rc::new(SplitVec)))?;
    env.define("_Co_vec_get".to_string(), Object::Callable(Rc::new(VecGet)))?;

    Ok(())

}
