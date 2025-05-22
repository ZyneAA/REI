use std::rc::Rc;

use super::interpreter::Interpreter;
use super::rei_callable::ReiCallable;
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
            Ok(_) => Ok(Object::Null),
            Err(ExecSignal::ControlFlow(ControlFlow::Return(value))) => Ok(value),
            Err(err) => Err(err),
        }

    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn>{}", self.name.lexeme)
    }

}

impl ReiFunction {

    pub fn new(name: Token, params: Vec<Token>, body: Vec<stmt::Stmt>, closure: EnvRef) -> Self {

        Self { name, params, body, closure }

    }

}
