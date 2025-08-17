use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::LazyLock;
use std::sync::Mutex;

use crate::backend::environment::Environment;
use crate::backend::exec_signal::runtime_error::{RuntimeError, RuntimeErrorType};
use crate::backend::exec_signal::ExecSignal;
use crate::backend::interpreter::Interpreter;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::stack_trace::ExecContext;
use crate::crux::token::Object;

static LISTENERS: LazyLock<Mutex<Vec<TcpListener>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static STREAMS: LazyLock<Mutex<Vec<TcpStream>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn push_listener(listener: TcpListener) -> usize {
    let mut l = LISTENERS.lock().unwrap();
    l.push(listener);
    l.len() - 1
}

fn push_stream(stream: TcpStream) -> usize {
    let mut s = STREAMS.lock().unwrap();
    s.push(stream);
    s.len() - 1
}

/// listen(addr: string) -> number (listener id)
#[derive(Clone, Debug)]
pub struct Listen;
impl ReiCallable for Listen {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        match &args[0] {
            Object::Str(addr) => {
                let listener = TcpListener::bind(addr).map_err(|e| {
                    let err = RuntimeErrorType::ErrorInNativeFn { msg: e.to_string() };
                    ExecSignal::RuntimeError(RuntimeError::new(err, context))
                })?;
                let id = push_listener(listener);
                Ok(Object::Number(id as f64))
            }
            _ => {
                let err = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected string for listen()".into(),
                };
                Err(ExecSignal::RuntimeError(RuntimeError::new(err, context)))
            }
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>listen".to_string()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// accept(listener_id) -> conn_id
#[derive(Clone, Debug)]
pub struct Accept;
impl ReiCallable for Accept {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let id = match args[0] {
            Object::Number(n) => n as usize,
            _ => {
                let err = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected listener id".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(err, context)));
            }
        };

        let l = LISTENERS.lock().unwrap();
        let listener = l.get(id);

        if let Some(listener) = listener {
            let (stream, _) = listener.accept().map_err(|e| {
                let err = RuntimeErrorType::ErrorInNativeFn { msg: e.to_string() };
                ExecSignal::RuntimeError(RuntimeError::new(err, context))
            })?;
            let sid = push_stream(stream);
            Ok(Object::Number(sid as f64))
        } else {
            let err = RuntimeErrorType::ErrorInNativeFn {
                msg: "Invalid listener id".into(),
            };
            Err(ExecSignal::RuntimeError(RuntimeError::new(err, context)))
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>accept".to_string()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// recv(conn_id, len) -> string
#[derive(Clone, Debug)]
pub struct Recv;
impl ReiCallable for Recv {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let id = args[0].as_number().unwrap_or(-1.0) as usize;
        let len = args[1].as_number().unwrap_or(0.0) as usize;
        let mut buf = vec![0u8; len];

        let mut streams = STREAMS.lock().unwrap();
        if let Some(stream) = streams.get_mut(id) {
            let n = stream.read(&mut buf).map_err(|e| {
                let err = RuntimeErrorType::ErrorInNativeFn { msg: e.to_string() };
                ExecSignal::RuntimeError(RuntimeError::new(err, context))
            })?;
            Ok(Object::Str(String::from_utf8_lossy(&buf[..n]).to_string()))
        } else {
            let err = RuntimeErrorType::ErrorInNativeFn {
                msg: "Invalid conn id".into(),
            };
            Err(ExecSignal::RuntimeError(RuntimeError::new(err, context)))
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>recv".to_string()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// send(conn_id, data) -> null
#[derive(Clone, Debug)]
pub struct Send;
impl ReiCallable for Send {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let id = args[0].as_number().unwrap_or(-1.0) as usize;
        let data = match &args[1] {
            Object::Str(s) => s.clone(),
            _ => {
                let err = RuntimeErrorType::ErrorInNativeFn {
                    msg: "Expected string for send".into(),
                };
                return Err(ExecSignal::RuntimeError(RuntimeError::new(err, context)));
            }
        };

        let mut streams = STREAMS.lock().unwrap();
        if let Some(stream) = streams.get_mut(id) {
            stream.write_all(data.as_bytes()).map_err(|e| {
                let err = RuntimeErrorType::ErrorInNativeFn { msg: e.to_string() };
                ExecSignal::RuntimeError(RuntimeError::new(err, context))
            })?;
            Ok(Object::Null)
        } else {
            let err = RuntimeErrorType::ErrorInNativeFn {
                msg: "Invalid conn id".into(),
            };
            Err(ExecSignal::RuntimeError(RuntimeError::new(err, context)))
        }
    }

    fn to_string(&self) -> String {
        "<native_fn>send".to_string()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {
    env.define("_NET_listen".to_string(), Object::Callable(Rc::new(Listen)))?;
    env.define("_NET_accept".to_string(), Object::Callable(Rc::new(Accept)))?;
    env.define("_NET_recv".to_string(), Object::Callable(Rc::new(Recv)))?;
    env.define("_NET_send".to_string(), Object::Callable(Rc::new(Send)))?;
    Ok(())
}
