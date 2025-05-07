use crate::crux::token::{ Object, Token, TokenType };
use crate::frontend::expr;
use super::runtime_error::RuntimeError;

pub struct Interpreter;

impl expr::Visitor<Result<Object, RuntimeError>> for Interpreter {

    fn visit_literal_expr(&mut self, value: &Object) -> Result<Object, RuntimeError> {
        Ok(value.clone())
    }

    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> Result<Object, RuntimeError> {
        self.evaluate(expression)
    }

    fn visit_unary_expr(&mut self, operator: &Token, expression: &expr::Expr) -> Result<Object, RuntimeError> {

        let right = self.evaluate(expression)?;

        match operator.token_type {
            TokenType::Minus => {
                self.check_number_operand(operator.clone(), right.clone())?;
                match right {
                    Object::Number(v) => Ok(Object::Number(-v)),
                    _ => {  unreachable!("Both operands should be numbers due to prior checks")}
                }
            },
            TokenType::Bang => {
                Ok(Object::Bool(!self.is_truthy(&right)))
            },
            _ => Err(RuntimeError::InvalidOperator { token: operator.clone() })
        }

    }

    fn visit_binary_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> Result<Object, RuntimeError> {

        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {

            TokenType::Plus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
                (Object::Str(a), Object::Str(b)) => Ok(Object::Str(a + &b)),
                _ => Err(RuntimeError::TypeMismatch { token: operator.clone() })
            },
            TokenType::Minus =>{
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                Ok(self.binary_number_operation(left, right, |a, b| a - b))
            },
            TokenType::Slash =>{
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                Ok(self.binary_number_operation(left, right, |a, b| a / b))
            },
            TokenType::Star =>{
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                Ok(self.binary_number_operation(left, right, |a, b| a * b))
            },

            TokenType::Greater => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                Ok(self.compare_number_operation(left, right, |a, b| a > b))
            },
            TokenType::GreaterEqual =>{
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                Ok(self.compare_number_operation(left, right, |a, b| a >= b))
            },
            TokenType::Less => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                Ok(self.compare_number_operation(left, right, |a, b| a < b))
            },
            TokenType::LessEqual =>{
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                Ok(self.compare_number_operation(left, right, |a, b| a <= b))
            },

            TokenType::EqualEqual => Ok(Object::Bool(self.is_equal(left, right))),
            TokenType::BangEqual => Ok(Object::Bool(!self.is_equal(left, right))),

            _ => panic!("Invalid Binary Operation")
        }

    }

}

impl Interpreter {

    fn evaluate(&mut self, expression: &expr::Expr) -> Result<Object, RuntimeError> {
        expression.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {

        match object {
            Object::Null => false,
            Object::Bool(v) => *v,
            _ => true
        }

    }

    fn check_number_operand(&self, operator: Token, operand: Object) -> Result<(), RuntimeError> {

        match operand {
            Object::Number(_) => { Ok(()) },
            _ => Err(RuntimeError::OperandMustBeNumber { token: operator })
        }

    }

    fn check_number_operands(&self, operator: Token, a: Object, b: Object) -> Result<(), RuntimeError> {

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

    pub fn binary_number_operation<F>(&self, a: Object, b: Object, op: F) -> Object
    where F: Fn(f64, f64) -> f64,
    {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => Object::Number(op(x, y)),
            _ => panic!("Runtime error: Both operands must be numbers."),
        }
    }

    fn compare_number_operation<F>(&self, a: Object, b: Object, op: F) -> Object
    where F: Fn(f64, f64) -> bool,
    {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => Object::Bool(op(x, y)),
            _ => panic!("Runtime error: Comparison operands must be numbers."),
        }
    }

}
