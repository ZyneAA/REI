use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

use super::interpreter::Interpreter;
use super::rei_callable::ReiCallable;
use super::rei_instance::ReiInstance;
use super::stmt;
use super::exec_signal::ExecSignal;
use super::exec_signal::control_flow::ControlFlow;
use super::environment::{ Environment, EnvRef };
use crate::crux::token::{ Token, Object };

#[derive(Debug, Clone)]
pub struct ReiFunction {

    name: Token,
    params: Vec<Token>,
    body: Vec<stmt::Stmt>,
    is_initializer: bool,
    closure: EnvRef

}


impl ReiCallable for ReiFunction {

    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, ExecSignal> {

        let env = Environment::from_enclosing(self.closure.clone());
        env.borrow_mut().define(
            self.name.lexeme.clone(),
            Object::Callable(Rc::new(self.clone()) as Rc<dyn ReiCallable>)
        )?;

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            env.borrow_mut().define(param.lexeme.clone(), arg.clone())?;
        }

        match interpreter.execute_block(&self.body, env) {
            Ok(_) => {
                if self.is_initializer {
                    Environment::get_at(&self.closure, 0, "this")
                }
                else {
                    Ok(Object::Null)
                }
            },
            Err(ExecSignal::ControlFlow(ControlFlow::Return(value))) => {
                if self.is_initializer {
                    Environment::get_at(&self.closure, 0, "this")
                }
                else {
                    Ok(value)
                }
            },
            Err(err) => Err(err),
        }

    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.name.lexeme)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

impl ReiFunction {

    pub fn new(name: Token, params: Vec<Token>, body: Vec<stmt::Stmt>, closure: EnvRef, is_initializer: bool) -> Self {

        Self { name, params, body, closure, is_initializer }

    }

    pub fn bind(&self, instance: ReiInstance) -> Result<ReiFunction, ExecSignal> {

        let env = Environment::from_enclosing(self.closure.clone());
        let instance = Rc::new(RefCell::new(instance));
        env.borrow_mut().define("this".to_string(), Object::Instance(instance))?;

        Ok(ReiFunction {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            closure: env,
            is_initializer: self.is_initializer
        })

    }

}
