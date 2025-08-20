use std::alloc::{alloc, dealloc, Layout};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::environment::Environment;
use crate::backend::exec_signal::runtime_error::RuntimeErrorType;
use crate::backend::exec_signal::{runtime_error::RuntimeError, ExecSignal};
use crate::backend::interpreter::Interpreter;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::rei_instance::ReiInstance;
use crate::backend::stack_trace::ExecContext;

use crate::crux::token::Object;
use crate::crux::token::Token;

#[derive(Clone, Debug)]
pub struct ReiMalloc;
impl ReiCallable for ReiMalloc {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let size = match arguments.get(0) {
            Some(Object::Number(n)) => *n as usize,
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected a Number".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    context.clone(),
                )));
            }
        };

        let layout = Layout::from_size_align(size, 8).map_err(|e| {
            let err_type = RuntimeErrorType::ErrorInNativeFn {
                msg: format!("invalid layout: {}", e),
            };
            ExecSignal::RuntimeError(RuntimeError::new(err_type, context.clone()))
        })?;

        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Allocation failed".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    context.clone(),
                )));
            }
            Ok(Object::MBlock(ptr, size))
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>_M_alloc".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ReiRead;
impl ReiCallable for ReiRead {
    fn arity(&self) -> usize {
        4
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let (ptr, size) = match arguments.get(0) {
            Some(Object::MBlock(p, s)) => (*p, *s),
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected MBlock as first arg".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    context.clone(),
                )));
            }
        };

        let offset = match arguments.get(1) {
            Some(Object::Number(n)) => *n as usize,
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected number offset as second arg".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    context.clone(),
                )));
            }
        };

        let length = match arguments.get(2) {
            Some(Object::Number(n)) => *n as usize,
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected Number as third arg".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    context.clone(),
                )));
            }
        };

        let mode = match arguments.get(3) {
            Some(Object::Bool(n)) => n,
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected a Bool as fourth arg".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    context.clone(),
                )));
            }
        };

        if offset + length > size {
            let err_type = RuntimeErrorType::ErrorInNativeFn {
                msg: format!("Offset {} out of bounds for size {}", offset + length, size),
            };
            return Err(ExecSignal::RuntimeError(RuntimeError::new(
                err_type,
                context.clone(),
            )));
        }

        let ptr = ptr as *const u8;
        unsafe {
            let slice = std::slice::from_raw_parts(ptr.add(offset), length);
            let val = if *mode {
                match length {
                    1 => Object::Number(slice[0] as f64),
                    8 => {
                        if (slice.as_ptr() as usize) % std::mem::align_of::<f64>() != 0 {
                            let err_type = RuntimeErrorType::ErrorInNativeFn {
                                msg: "Unaligned f64 read".to_string(),
                            };
                            return Err(ExecSignal::RuntimeError(RuntimeError::new(
                                err_type,
                                context.clone(),
                            )));
                        }
                        let num = *(slice.as_ptr() as *const f64);
                        Object::Number(num)
                    }
                    _ => {
                        let repr = slice
                            .iter()
                            .map(|b| format!("0x{:02X}", b))
                            .collect::<Vec<_>>()
                            .join(" ");
                        let info = format!(
                            "mem[{:p}+{}..+{}]: [{}]",
                            ptr,
                            offset,
                            offset + length,
                            repr
                        );
                        Object::Str(info)
                    }
                }
            } else {
                match std::str::from_utf8(slice) {
                    Ok(s) => Object::Str(s.to_string()),
                    Err(_) => {
                        let repr = slice
                            .iter()
                            .map(|b| format!("0x{:02X}", b))
                            .collect::<Vec<_>>()
                            .join(" ");
                        let info = format!(
                            "mem[{:p}+{}..+{}]: [{}]",
                            ptr,
                            offset,
                            offset + length,
                            repr
                        );
                        Object::Str(info)
                    }
                }
            };

            Ok(val)
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>_M_read".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ReiWrite;
impl ReiCallable for ReiWrite {
    fn arity(&self) -> usize {
        3
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let (ptr, size) = match arguments.get(0) {
            Some(Object::MBlock(p, s)) => (*p, *s),
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected MBlock as first arg".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    context.clone(),
                )));
            }
        };

        let offset = match arguments.get(1) {
            Some(Object::Number(n)) => *n as usize,
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected Number as second arg".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    context.clone(),
                )));
            }
        };

        let value = arguments.get(2).ok_or_else(|| {
            let err_type = RuntimeErrorType::ErrorInNativeFn {
                msg: "Missing third argument".into(),
            };
            ExecSignal::RuntimeError(RuntimeError::new(err_type, context.clone()))
        })?;

        if offset >= size {
            let err_type = RuntimeErrorType::ErrorInNativeFn {
                msg: format!("Offset {} out of bounds for size {}", offset, size),
            };
            return Err(ExecSignal::RuntimeError(RuntimeError::new(
                err_type,
                context.clone(),
            )));
        }

        unsafe {
            match value {
                Object::Number(n) => {
                    if offset + 8 > size {
                        let err_type = RuntimeErrorType::ErrorInNativeFn {
                            msg: format!("offset {}+8 out of bounds", offset),
                        };
                        return Err(ExecSignal::RuntimeError(RuntimeError::new(
                            err_type,
                            context.clone(),
                        )));
                    }
                    let ptr = ptr.add(offset) as *mut f64;
                    *ptr = *n;
                }
                Object::Bool(b) => {
                    if offset >= size {
                        let err_type = RuntimeErrorType::ErrorInNativeFn {
                            msg: format!("write: offset {} out of bounds", offset),
                        };
                        return Err(ExecSignal::RuntimeError(RuntimeError::new(
                            err_type,
                            context.clone(),
                        )));
                    }
                    *ptr.add(offset) = *b as u8;
                }
                Object::Str(s) => {
                    let bytes = s.as_bytes();
                    if offset + bytes.len() > size {
                        let err_type = RuntimeErrorType::ErrorInNativeFn {
                            msg: format!("string too long to fit at offset {}", offset),
                        };
                        return Err(ExecSignal::RuntimeError(RuntimeError::new(
                            err_type,
                            context.clone(),
                        )));
                    }
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.add(offset), bytes.len());
                }
                _ => {
                    let err_type = RuntimeErrorType::ErrorInNativeFn {
                        msg: "unsupported type for write".to_string(),
                    };
                    return Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        context.clone(),
                    )));
                }
            }
        }

        Ok(Object::Null)
    }

    fn to_string(&self) -> String {
        "<native_fn>_M_write".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ReiFree;
impl ReiCallable for ReiFree {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let (ptr, size) = match arguments.get(0) {
            Some(Object::MBlock(p, s)) => (*p, *s),
            _ => {
                let err_type = RuntimeErrorType::ErrorInNativeFn {
                    msg: "expected a MBlock when freeing memory".to_string(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    context.clone(),
                )));
            }
        };

        let layout = Layout::from_size_align(size, 8).map_err(|e| {
            let err_type = RuntimeErrorType::ErrorInNativeFn {
                msg: format!("invalid layout: {}", e),
            };
            ExecSignal::RuntimeError(RuntimeError::new(err_type, context.clone()))
        })?;

        unsafe {
            dealloc(ptr, layout);
        }

        Ok(Object::Null)
    }

    fn to_string(&self) -> String {
        "<native_fn>_M_free".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ReiSizeOf;
impl ReiCallable for ReiSizeOf {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let obj = arguments.get(0).ok_or_else(|| {
            let err_type = RuntimeErrorType::ErrorInNativeFn {
                msg: "Expected one argument to _M_sizeof".to_string(),
            };
            ExecSignal::RuntimeError(RuntimeError::new(err_type, context.clone()))
        })?;

        let size = match obj {
            Object::Number(_) => std::mem::size_of::<f64>(),
            Object::Str(s) => std::mem::size_of::<String>() + s.len(),
            Object::Bool(_) => std::mem::size_of::<bool>(),
            Object::Range(_, _) => std::mem::size_of::<(f64, f64)>(),
            Object::Dummy => 0,
            Object::Null => 0,
            Object::Callable(_) => std::mem::size_of::<Rc<dyn ReiCallable>>(),
            Object::Instance(_) => std::mem::size_of::<Rc<RefCell<ReiInstance>>>(),
            Object::MBlock(_, size) => *size,
            Object::Vec(v) => {
                let object_size = std::mem::size_of::<Object>();
                let total = object_size * v.borrow().len();
                std::mem::size_of::<Vec<Object>>() + total
            }
            Object::Exception(e) => {
                std::mem::size_of::<Box<RuntimeError<Token>>>() + std::mem::size_of_val(&**e)
            }
        };

        Ok(Object::Number(size as f64))
    }

    fn to_string(&self) -> String {
        "<native_fn>_M_sizeof".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
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

    let size_of: Rc<dyn ReiCallable> = Rc::new(ReiSizeOf);
    env.define("_M_size_of".to_string(), Object::Callable(size_of))?;

    Ok(())
}
