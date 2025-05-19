use std::alloc::{ alloc, Layout, dealloc };
use std::rc::Rc;

use crate::crux::token::Object;
use crate::backend::interpreter::Interpreter;
use crate::backend::exec_signal::ExecSignal;
use crate::backend::exec_signal::runtime_error::RuntimeError;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::environment::Environment;

#[derive(Clone, Debug)]
pub struct ReiMalloc;
impl ReiCallable for ReiMalloc {
    fn arity(&self) -> usize {
        1
    }
    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {
        let size = match arguments.get(0) {
            Some(Object::Number(n)) => *n as usize,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "malloc: expected number".to_string(),
            })),
        };

        let layout = Layout::from_size_align(size, 8).map_err(|e| ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
            msg: format!("malloc: invalid layout: {}", e),
        }))?;

        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                    msg: "malloc: allocation failed".to_string(),
                }));
            }

            Ok(Object::Range
                (ptr as usize as f64,
                size as f64)
            )
        }
    }
    fn to_string(&self) -> String {
        "<native_fn>alloc".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct ReiRead;
impl ReiCallable for ReiRead {
    fn arity(&self) -> usize {
        2
    }
    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {
        let (addr, size) = match arguments.get(0) {
            Some(Object::Range(start, end)) => (start.clone() as usize, end.clone() as usize),
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "read: expected Range(ptr, size) as first arg".to_string(),
            })),
        };

        let offset = match arguments.get(1) {
            Some(Object::Number(n)) => *n as usize,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "read: expected number offset as second arg".to_string(),
            })),
        };

        if offset >= size {
            return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: format!("read: offset {} out of bounds for size {}", offset, size),
            }));
        }

        unsafe {
            let ptr = addr as *const u8;
            let byte = *ptr.add(offset);
            Ok(Object::Number(byte as f64))
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>read".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct ReiWrite;
impl ReiCallable for ReiWrite {
    fn arity(&self) -> usize {
        3
    }
    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {
        let (addr, size) = match arguments.get(0) {
            Some(Object::Range(start, end)) => (start.clone() as usize, end.clone() as usize),
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "write: expected Range(ptr, size) as first arg".to_string(),
            })),
        };

        let offset = match arguments.get(1) {
            Some(Object::Number(n)) => *n as usize,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "write: expected number offset as second arg".to_string(),
            })),
        };

        let value = match arguments.get(2) {
            Some(Object::Number(n)) => *n as u8,
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "write: expected number value as third arg".to_string(),
            })),
        };

        if offset >= size {
            return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: format!("write: offset {} out of bounds for size {}", offset, size),
            }));
        }

        unsafe {
            let ptr = addr as *mut u8;
            *ptr.add(offset) = value;
        }

        Ok(Object::Null)
    }
    fn to_string(&self) -> String {
        "<native_fn>write".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct ReiFree;
impl ReiCallable for ReiFree {
    fn arity(&self) -> usize {
        1
    }
    fn call(&self, _interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {
        let (addr, size) = match arguments.get(0) {
            Some(Object::Range(start, end)) => (start.clone() as usize, end.clone() as usize),
            _ => return Err(ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
                msg: "free: expected a Range(ptr, size)".to_string(),
            })),
        };

        let layout = Layout::from_size_align(size, 8).map_err(|e| ExecSignal::RuntimeError(RuntimeError::ErrorInNativeFn {
            msg: format!("free: invalid layout: {}", e),
        }))?;

        unsafe {
            let ptr = addr as *mut u8;
            dealloc(ptr, layout);
        }

        Ok(Object::Null)
    }
    fn to_string(&self) -> String {
        "<native_fn>free".to_string()
    }
}

pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {

    let malloc: Rc<dyn ReiCallable> = Rc::new(ReiMalloc);
    env.define("_M_alloc".to_string(), Object::Callable(malloc))?;

    let write: Rc<dyn ReiCallable> = Rc::new(ReiWrite);
    env.define("_M_write".to_string(), Object::Callable(write))?;

    let read: Rc<dyn ReiCallable> = Rc::new(ReiRead);
    env.define("_M_read".to_string(), Object::Callable(read))?;

    let free: Rc<dyn ReiCallable> = Rc::new(ReiFree);
    env.define("_M_free".to_string(), Object::Callable(free))?;

    Ok(())

}
