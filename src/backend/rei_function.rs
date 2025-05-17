use super::interpreter::Interpreter;
use super::rei_callable::ReiCallable;
use super::stmt;
use super::runtime_error::RuntimeError;
use super::environment::Environment;
use crate::crux::token::{ Token, Object };

#[derive(Debug)]
pub struct ReiFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<stmt::Stmt>,
}


impl<'a> ReiCallable for ReiFunction {

    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Result<Object, RuntimeError<Token>> {

        let env = Environment::from_enclosing(interpreter.environment.clone());

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            env.borrow_mut().define(param.lexeme.clone(), arg.clone())?;
        }

        interpreter.execute_block(&self.body, env)?;
        Ok(Object::Null)

    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn>{}", self.name.lexeme)
    }

}

impl ReiFunction {

    pub fn new(name: Token, params: Vec<Token>, body: Vec<stmt::Stmt>) -> Self {

        Self { name, params, body }

    }

}
