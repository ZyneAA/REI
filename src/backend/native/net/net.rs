use reqwest::blocking;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
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
use crate::crux::token;
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

#[derive(Clone, Debug)]
pub struct Route {
    pub method: String,
    pub pattern: String,
}
static ROUTES: LazyLock<Mutex<Vec<Route>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Clone, Debug)]
pub struct Get;
impl ReiCallable for Get {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let url = args[0].as_str().unwrap();

        let response = blocking::get(url).map_err(|e| {
            let err = RuntimeErrorType::ErrorInNativeFn {
                msg: format!("{}", e),
            };
            ExecSignal::RuntimeError(RuntimeError::new(err, context.clone()))
        })?;

        let body = response.text().map_err(|e| {
            let err = RuntimeErrorType::ErrorInNativeFn {
                msg: format!("{}", e),
            };
            ExecSignal::RuntimeError(RuntimeError::new(err, context))
        })?;

        Ok(Object::Str(body))
    }

    fn to_string(&self) -> String {
        "<native_fn>get".into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct Router;
impl ReiCallable for Router {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _args: &Vec<Object>,
        _context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        Ok(Object::Router(Rc::new(RefCell::new(token::Router {
            routes: HashMap::new(),
        }))))
    }

    fn to_string(&self) -> String {
        "<native_fn>router".into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct RouterGet;
impl ReiCallable for RouterGet {
    fn arity(&self) -> usize {
        3
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let router = match &args[0] {
            Object::Router(r) => r.clone(),
            _ => {
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    RuntimeErrorType::ErrorInNativeFn {
                        msg: "Expected router".into(),
                    },
                    context,
                )))
            }
        };

        let path = match &args[1] {
            Object::Str(s) => s.clone(),
            _ => {
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    RuntimeErrorType::ErrorInNativeFn {
                        msg: "Expected string path".into(),
                    },
                    context,
                )))
            }
        };

        let handler = match &args[2] {
            Object::Callable(c) => c.clone(),
            _ => {
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    RuntimeErrorType::ErrorInNativeFn {
                        msg: "Expected function".into(),
                    },
                    context,
                )))
            }
        };

        router
            .borrow_mut()
            .routes
            .insert(("GET".into(), path), handler);
        Ok(Object::Null)
    }

    fn to_string(&self) -> String {
        "<native_fn>router_get".into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct RouterServe;
impl ReiCallable for RouterServe {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let router = match &args[0] {
            Object::Router(r) => r.clone(),
            _ => {
                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                    RuntimeErrorType::ErrorInNativeFn {
                        msg: "Expected router".into(),
                    },
                    context,
                )))
            }
        };

        let addr = args[1].as_str().map_err(|msg| {
            let err = RuntimeErrorType::ErrorInNativeFn { msg };
            ExecSignal::RuntimeError(RuntimeError::new(err, context.clone()))
        })?;

        let listener = std::net::TcpListener::bind(addr).map_err(|e| {
            let err = RuntimeErrorType::ErrorInNativeFn { msg: e.to_string() };
            ExecSignal::RuntimeError(RuntimeError::new(err, context.clone()))
        })?;

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buf = [0; 2048];
                let n = stream.read(&mut buf).unwrap_or(0);
                let req = String::from_utf8_lossy(&buf[..n]);

                // simple parse: first line "GET /path HTTP/1.1"
                let mut parts = req.lines().next().unwrap_or("").split_whitespace();
                let method = parts.next().unwrap_or("").to_string();
                let path = parts.next().unwrap_or("").to_string();

                // handle OPTIONS preflight
                if method == "OPTIONS" {
                    let resp = "HTTP/1.1 204 No Content\r\n\
                        Access-Control-Allow-Origin: *\r\n\
                        Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
                        Access-Control-Allow-Headers: Content-Type\r\n\
                        Content-Length: 0\r\n\r\n";
                    let _ = stream.write_all(resp.as_bytes());
                    continue;
                }

                let handler_opt = {
                    let r = router.borrow();
                    r.routes.get(&(method.clone(), path.clone())).cloned()
                };

                let resp_body = if let Some(handler) = handler_opt {
                    match handler.call(
                        interpreter,
                        &vec![Object::Str(req.to_string())],
                        context.clone(),
                    ) {
                        Ok(Object::Str(s)) => s,
                        Ok(other) => format!("{:?}", other),
                        Err(_) => "Internal Server Error".into(),
                    }
                } else {
                    "404 Not Found".into()
                };

                let resp = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Length: {}\r\n\
                    Content-Type: text/plain\r\n\
                    Access-Control-Allow-Origin: *\r\n\
                    Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
                    Access-Control-Allow-Headers: Content-Type\r\n\r\n{}",
                    resp_body.len(),
                    resp_body
                );

                let _ = stream.write_all(resp.as_bytes());
            }
        }

        Ok(Object::Null)
    }

    fn to_string(&self) -> String {
        "<native_fn>serve".into()
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
    env.define("_NET_router".into(), Object::Callable(Rc::new(Router)))?;
    env.define(
        "_NET_router_get".into(),
        Object::Callable(Rc::new(RouterGet)),
    )?;
    env.define("_NET_serve".into(), Object::Callable(Rc::new(RouterServe)))?;
    env.define("_NET_get".into(), Object::Callable(Rc::new(Get)))?;
    Ok(())
}
