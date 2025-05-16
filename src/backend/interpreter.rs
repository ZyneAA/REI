use crate::crux::token::{ Object, Token, TokenType };
use crate::frontend::expr;
use crate::backend::stmt;
use crate::backend::environment::{ Environment, EnvRef };
use super::runtime_error::RuntimeError;

pub struct Interpreter {

    pub environment: EnvRef

}

impl expr::Visitor<Result<Object, RuntimeError<Token>>> for Interpreter {

    fn visit_literal_expr(&mut self, value: &Object) -> Result<Object, RuntimeError<Token>> {
        Ok(value.clone())
    }

    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> Result<Object, RuntimeError<Token>> {
        self.evaluate(expression)
    }

    fn visit_unary_expr(&mut self, operator: &Token, expression: &expr::Expr) -> Result<Object, RuntimeError<Token>> {

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
            _ => Err(RuntimeError::InvalidOperator { token: operator.clone() })
        }

    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, RuntimeError<Token>> {
        self.environment.borrow_mut().get(name)
    }

    fn visit_binary_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> Result<Object, RuntimeError<Token>> {

        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {

            TokenType::Plus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
                (Object::Str(a), Object::Str(b)) => Ok(Object::Str(a + &b)),
                (Object::Str(a), b) => Ok(Object::Str(a + &b.to_string())),
                (a, Object::Str(b)) => Ok(Object::Str(a.to_string() + &b)),
                _ => Err(RuntimeError::TypeMismatch { token: operator.clone() })
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

            _ => Err(RuntimeError::UnexpectedBinaryOperation { token: operator.clone() })

        }

    }

    fn visit_assign_expr(&mut self, name: &Token, value: &expr::Expr) -> Result<Object, RuntimeError<Token>> {

        let value = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)

    }

    fn visit_logical_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> Result<Object, RuntimeError<Token>> {

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

    fn visit_range_expr(&mut self, start: &expr::Expr, end: &expr::Expr) -> Result<Object, RuntimeError<Token>> {

        let start = self.evaluate(start)?;
        let end = self.evaluate(end)?;
        match (start, end) {
            (Object::Number(s), Object::Number(e)) => {
                if s.fract() != 0.0 || e.fract() != 0.0 {
                    return Err(RuntimeError::InvalidRangeType)
                }
                if e < s {
                    Err(RuntimeError::InvalidRange)
                }
                else {
                    Ok(Object::Range(s, e))
                }
            },
            _ => Err(RuntimeError::InvalidRangeType)
        }

    }

}

impl stmt::Visitor<Result<(), RuntimeError<Token>>> for Interpreter {

    fn visit_expression_stmt(&mut self, expression: &expr::Expr) -> Result<(), RuntimeError<Token>> {

        self.evaluate(expression)?;
        Ok(())

    }

    fn visit_print_stmt(&mut self, expression: &expr::Expr) -> Result<(), RuntimeError<Token>> {

        let value = self.evaluate(expression)?;
        print!("{}", self.stringify(&value));
        Ok(())

    }

    fn visit_println_stmt(&mut self, expression: &expr::Expr) -> Result<(), RuntimeError<Token>> {

        let value = self.evaluate(expression)?;
        println!("{}", self.stringify(&value));
        Ok(())

    }

    fn visit_let_stmt(&mut self, name: &Token, initializer: &expr::Expr) -> Result<(), RuntimeError<Token>> {

        let value = self.evaluate(initializer)?;
        self.environment.borrow_mut().define(name.lexeme.clone(), value)?;
        Ok(())

    }

    fn visit_block_stmt(&mut self, statements: &Vec<stmt::Stmt>) -> Result<(), RuntimeError<Token>> {

        let new_env = Environment::from_enclosing(self.environment.clone());
        self.execute_block(statements, new_env)

    }

    fn visit_if_stmt(&mut self, condition: &expr::Expr, then_branch: &stmt::Stmt, else_branch: &Option<Box<stmt::Stmt>>) -> Result<(), RuntimeError<Token>> {

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

    fn visit_while_stmt(&mut self, condition: &expr::Expr, body: &stmt::Stmt) -> Result<(), RuntimeError<Token>> {

        loop {
            let cond = self.evaluate(condition)?;
            if !self.is_truthy(&cond) {
                break;
            }
            match self.execute(body) {
                Ok(_) => {},
                Err(RuntimeError::Break) => break,
                Err(RuntimeError::Continue) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())

    }

    fn visit_break_stmt(&mut self) -> Result<(), RuntimeError<Token>> {
        Err(RuntimeError::Break)
    }

    fn visit_continue_stmt(&mut self) -> Result<(), RuntimeError<Token>> {
        Err(RuntimeError::Continue)
    }

}

impl Interpreter {

    pub fn new() -> Self {

        let environment = Environment::global();
        Interpreter { environment }

    }

    pub fn interpret(&mut self, statements: Vec<stmt::Stmt>) -> Result<(), RuntimeError<Token>> {

        for stmt in statements {
            self.execute(&stmt)?;
        }
        Ok(())

    }

    fn execute(&mut self, statement: &stmt::Stmt) -> Result<(), RuntimeError<Token>> {
        statement.accept(self)
    }

    fn execute_block(&mut self, statements: &Vec<stmt::Stmt>, env: EnvRef) -> Result<(), RuntimeError<Token>> {

        let previous = self.environment.clone();
        self.environment = env;

        for stmt in statements {
            self.execute(stmt)?;
        }

        self.environment = previous;

        Ok(())

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
        }

    }

    fn evaluate(&mut self, expression: &expr::Expr) -> Result<Object, RuntimeError<Token>> {
        expression.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {

        match object {
            Object::Null => false,
            Object::Bool(v) => *v,
            _ => true
        }

    }

    fn check_number_operand(&self, operator: Token, operand: Object) -> Result<(), RuntimeError<Token>> {

        match operand {
            Object::Number(_) => { Ok(()) },
            _ => Err(RuntimeError::OperandMustBeNumber { token: operator })
        }

    }

    fn check_number_operands(&self, operator: Token, a: Object, b: Object) -> Result<(), RuntimeError<Token>> {

        match (a, b) {
            (Object::Number(_), Object::Number(_)) => { Ok(()) },
            _ => Err(RuntimeError::OperandMustBeNumber { token: operator})
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

    pub fn binary_number_operation<T>(&self, a: Object, b: Object, token: Token, op: T) -> Result<Object, RuntimeError<Token>>
    where T: Fn(f64, f64) -> f64,
    {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => {
                if token.token_type == TokenType::Slash && y == 0.0 {
                    Err(RuntimeError::DividedByZero { token })
                }
                else {
                    Ok(Object::Number(op(x, y)))
                }
            }
            _ => Err(RuntimeError::OperandMustBeNumber{ token }),
        }
    }

    fn compare_number_operation<F>(&self, a: Object, b: Object, token: Token, op: F) -> Result<Object, RuntimeError<Token>>
    where F: Fn(f64, f64) -> bool,
    {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => Ok(Object::Bool(op(x, y))),
            _ => Err(RuntimeError::OperandMustBeNumber{ token })
        }
    }

}
