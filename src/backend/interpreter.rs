use std::rc::Rc;

use crate::crux::token::{ Object, Token, TokenType };
use crate::frontend::expr;
use crate::backend::stmt;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::environment::{ Environment, EnvRef };
use super::native;
use super::rei_function::ReiFunction;
use super::exec_signal::ExecSignal;
use super::exec_signal::runtime_error::RuntimeError;
use super::exec_signal::control_flow::ControlFlow;

pub struct Interpreter {

    pub environment: EnvRef

}

impl expr::Visitor<Result<Object, ExecSignal>> for Interpreter {

    fn visit_literal_expr(&mut self, value: &Object) -> Result<Object, ExecSignal> {
        Ok(value.clone())
    }

    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> Result<Object, ExecSignal> {
        self.evaluate(expression)
    }

    fn visit_unary_expr(&mut self, operator: &Token, expression: &expr::Expr) -> Result<Object, ExecSignal> {

        let right = self.evaluate(expression)?;

        match operator.token_type {
            TokenType::Minus => {
                self.check_number_operand(operator.clone(), right.clone())?;
                match right {
                    Object::Number(v) => Ok(Object::Number(-v)),
                    _ => {  unreachable!("Both operands should be numbers due to prior checks") }
                }
            },
            TokenType::Bang => {
                Ok(Object::Bool(!self.is_truthy(&right)))
            },
            _ => Err(ExecSignal::RuntimeError(RuntimeError::InvalidOperator { token: operator.clone() }))
        }

    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, ExecSignal> {
        self.environment.borrow_mut().get(name)
    }

    fn visit_binary_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> Result<Object, ExecSignal> {

        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {

            TokenType::Plus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
                (Object::Str(a), Object::Str(b)) => Ok(Object::Str(a + &b)),
                (Object::Str(a), b) => Ok(Object::Str(a + &b.to_string())),
                (a, Object::Str(b)) => Ok(Object::Str(a.to_string() + &b)),
                _ => Err(ExecSignal::RuntimeError(RuntimeError::TypeMismatch { token: operator.clone() }))
            },
            TokenType::Minus => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.binary_number_operation(left, right, operator.clone(), |a, b| a - b)
            },
            TokenType::Slash => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.binary_number_operation(left, right, operator.clone(), |a, b| a / b)
            },
            TokenType::Star => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.binary_number_operation(left, right, operator.clone(), |a, b| a * b)
            },

            TokenType::Greater => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.compare_number_operation(left, right, operator.clone(), |a, b| a > b)
            },
            TokenType::GreaterEqual => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.compare_number_operation(left, right, operator.clone(), |a, b| a >= b)
            },
            TokenType::Less => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.compare_number_operation(left, right, operator.clone(), |a, b| a < b)
            },
            TokenType::LessEqual => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.compare_number_operation(left, right, operator.clone(), |a, b| a <= b)
            },

            TokenType::EqualEqual => Ok(Object::Bool(self.is_equal(left, right))),
            TokenType::BangEqual => Ok(Object::Bool(!self.is_equal(left, right))),

            _ => Err( ExecSignal::RuntimeError(RuntimeError::UnexpectedBinaryOperation { token: operator.clone() }))

        }

    }

    fn visit_assign_expr(&mut self, name: &Token, value: &expr::Expr) -> Result<Object, ExecSignal> {

        let value = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)

    }

    fn visit_logical_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> Result<Object, ExecSignal> {

        let left = self.evaluate(left)?;

        if operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left)
            }
        }
        else {
            if !self.is_truthy(&left) {
                return Ok(left)
            }
        }

        self.evaluate(right)

    }

    fn visit_range_expr(&mut self, start: &expr::Expr, end: &expr::Expr) -> Result<Object, ExecSignal> {

        let start = self.evaluate(start)?;
        let end = self.evaluate(end)?;
        match (start, end) {
            (Object::Number(s), Object::Number(e)) => {
                if s.fract() != 0.0 || e.fract() != 0.0 {
                    return Err(ExecSignal::RuntimeError(RuntimeError::InvalidRangeType))
                }
                if e < s {
                    Err(ExecSignal::RuntimeError(RuntimeError::InvalidRange))
                }
                else {
                    Ok(Object::Range(s, e))
                }
            },
            _ => Err(ExecSignal::RuntimeError(RuntimeError::InvalidRangeType))
        }

    }

    fn visit_call_expr(&mut self, callee: &expr::Expr, paren: &Token, arguments: &Vec<expr::Expr>) -> Result<Object, ExecSignal> {

        let callee= self.evaluate(callee)?;
        let mut args = vec![];
        for arg in arguments {
            args.push(self.evaluate(&arg)?);
        }

        match callee {
            Object::Callable(ref function) => {
                if arguments.len() != function.arity() {
                    return Err(ExecSignal::RuntimeError(RuntimeError:: InvalidArguments { token: paren.clone() }))
                }
                function.call(self, &args)
            }
            _ => Err(ExecSignal::RuntimeError(RuntimeError::NotCallable))
        }

    }


}

impl stmt::Visitor<Result<(), ExecSignal>> for Interpreter {

    fn visit_expression_stmt(&mut self, expression: &expr::Expr) -> Result<(), ExecSignal> {

        self.evaluate(expression)?;
        Ok(())

    }

    fn visit_print_stmt(&mut self, expression: &expr::Expr) -> Result<(), ExecSignal> {

        let value = self.evaluate(expression)?;
        print!("{}", self.stringify(&value));
        Ok(())

    }

    fn visit_println_stmt(&mut self, expression: &expr::Expr) -> Result<(), ExecSignal> {

        let value = self.evaluate(expression)?;
        println!("{}", self.stringify(&value));
        Ok(())

    }

    fn visit_let_stmt(&mut self, name: &Token, initializer: &expr::Expr) -> Result<(), ExecSignal> {

        let value = self.evaluate(initializer)?;
        self.environment.borrow_mut().define(name.lexeme.clone(), value)?;
        Ok(())

    }

    fn visit_block_stmt(&mut self, statements: &Vec<stmt::Stmt>) -> Result<(), ExecSignal> {

        let new_env = Environment::from_enclosing(self.environment.clone());
        self.execute_block(statements, new_env)

    }

    fn visit_if_stmt(&mut self, condition: &expr::Expr, then_branch: &stmt::Stmt, else_branch: &Option<Box<stmt::Stmt>>) -> Result<(), ExecSignal> {

        let obj = self.evaluate(condition)?;

        if self.is_truthy(&obj) {
            self.execute(then_branch)
        }
        else {
            match else_branch {
                Some(v) => self.execute(v),
                None => Ok(())
            }
        }

    }

    fn visit_while_stmt(&mut self, condition: &expr::Expr, body: &stmt::Stmt) -> Result<(), ExecSignal> {

        loop {
            let cond = self.evaluate(condition)?;
            if !self.is_truthy(&cond) {
                break;
            }
            match self.execute(body) {
                Ok(_) => {},
                Err(ExecSignal::ControlFlow(ControlFlow::Break)) => break,
                Err(ExecSignal::ControlFlow(ControlFlow::Continue)) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())

    }

    fn visit_break_stmt(&mut self) -> Result<(), ExecSignal> {
        Err(ExecSignal::ControlFlow(ControlFlow::Break))
    }

    fn visit_continue_stmt(&mut self) -> Result<(), ExecSignal> {
        Err(ExecSignal::ControlFlow(ControlFlow::Continue))
    }

    fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<stmt::Stmt>) -> Result<(), ExecSignal> {

        let function = ReiFunction::new(name.clone(), params.clone(), body.clone());
        let callable: Rc<dyn ReiCallable> = Rc::new(function);
        self.environment.borrow_mut().define(name.lexeme.clone(), Object::Callable(callable))?;
        Ok(())

    }

    fn visit_return_stmt(&mut self, _keyword: &Token, value: &Option<Box<expr::Expr>>) -> Result<(), ExecSignal> {

        let value = match value {
            Some(v) => self.evaluate(v)?,
            None => Object::Null
        };
        Err(ExecSignal::ControlFlow(ControlFlow::Return(value)))

    }

}

impl Interpreter {

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {

        let environment = Environment::global();
        native::register_all_native_fns(environment.borrow_mut())?;
        Ok(Interpreter { environment })

    }

    pub fn interpret(&mut self, statements: Vec<stmt::Stmt>) -> Result<(), ExecSignal> {

        for stmt in statements {
            self.execute(&stmt)?;
        }

        Ok(())

    }

    fn execute(&mut self, statement: &stmt::Stmt) -> Result<(), ExecSignal> {
        statement.accept(self)
    }

    pub fn execute_block(&mut self, statements: &Vec<stmt::Stmt>, env: EnvRef) -> Result<(), ExecSignal> {

        self.with_env(env, |interpreter| {
            for stmt in statements {
                interpreter.execute(stmt)?;
            }
            Ok(())
        })

    }

    pub fn stringify(&mut self, object: &Object) -> String {

        match object {
            Object::Null => "null".to_string(),
            Object::Number(n) => {
                let mut s = n.to_string();
                if s.ends_with(".0") {
                    s.truncate(s.len() - 2); // yeet the ".0"
                }
                s
            },
            Object::Range(s, e) => {
                format!("Range {}:{}", s, e)
            }
            Object::Bool(b) => b.to_string(),
            Object::Dummy => "dummy".to_string(),
            Object::Str(s) => s.clone(),
            Object::Callable(c) => c.to_string()
        }

    }

    fn evaluate(&mut self, expression: &expr::Expr) -> Result<Object, ExecSignal> {
        expression.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {

        match object {
            Object::Null => false,
            Object::Bool(v) => *v,
            _ => true
        }

    }

    fn check_number_operand(&self, operator: Token, operand: Object) -> Result<(), ExecSignal> {

        match operand {
            Object::Number(_) => { Ok(()) },
            _ => Err(ExecSignal::RuntimeError(RuntimeError::OperandMustBeNumber { token: operator }))
        }

    }

    fn check_number_operands(&self, operator: Token, a: Object, b: Object) -> Result<(), ExecSignal> {

        match (a, b) {
            (Object::Number(_), Object::Number(_)) => { Ok(()) },
            _ => Err(ExecSignal::RuntimeError(RuntimeError::OperandMustBeNumber { token: operator}))
        }

    }

    pub fn is_equal(&self, a: Object, b: Object) -> bool {

        match (a, b) {
            (Object::Null, Object::Null) => true,
            (Object::Bool(a), Object::Bool(b)) => a == b,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::Str(a), Object::Str(b)) => a == b,
            _ => false,
        }

    }

    pub fn binary_number_operation<T>(&self, a: Object, b: Object, token: Token, op: T) -> Result<Object, ExecSignal>
    where T: Fn(f64, f64) -> f64,
    {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => {
                if token.token_type == TokenType::Slash && y == 0.0 {
                    Err(ExecSignal::RuntimeError(RuntimeError::DividedByZero { token }))
                }
                else {
                    Ok(Object::Number(op(x, y)))
                }
            }
            _ => Err(ExecSignal::RuntimeError(RuntimeError::OperandMustBeNumber{ token })),
        }
    }

    fn compare_number_operation<F>(&self, a: Object, b: Object, token: Token, op: F) -> Result<Object, ExecSignal>
    where F: Fn(f64, f64) -> bool,
    {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => Ok(Object::Bool(op(x, y))),
            _ => Err(ExecSignal::RuntimeError(RuntimeError::OperandMustBeNumber{ token }))
        }
    }

    pub fn with_env<F, R>(&mut self, env: EnvRef, f: F) -> R
    where F: FnOnce(&mut Interpreter) -> R,
    {
        let previous = self.environment.clone();
        self.environment = env;
        let result = f(self);
        self.environment = previous;
        result
    }

}
