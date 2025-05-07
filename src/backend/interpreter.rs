use crate::crux::token::{ Object, Token, TokenType };
use crate::frontend::expr;

pub struct Interpreter;

impl expr::Visitor<Object> for Interpreter {

    fn visit_literal_expr(&mut self, value: &Object) -> Object {
        value.clone()
    }

    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> Object {
        self.evaluate(expression)
    }

    fn visit_unary_expr(&mut self, operator: &Token, expression: &expr::Expr) -> Object {

        let right = self.evaluate(expression);

        match operator.token_type {
            TokenType::Minus => {
                if let Object::Number(n) = right {
                    Object::Number(-n)
                }
                else {
                    panic!("Operand must be a number for unary minus");
                }
            },
            TokenType::Bang => {
                Object::Bool(!self.is_truthy(&right))
            },
            _ => panic!("Invalid")
        }

    }

    fn visit_binary_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> Object {

        let left = self.evaluate(left);
        let right = self.evaluate(right);

        match operator.token_type {

            TokenType::Plus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Object::Number(a + b),
                (Object::Str(a), Object::Str(b)) => Object::Str(a + &b),
                _ => panic!("Runtime error: '+' requires two numbers or two strings."),
            },
            TokenType::Minus => { self.binary_number_operation(left, right, |a, b| a - b) },
            TokenType::Slash => { self.binary_number_operation(left, right, |a, b| a / b) },
            TokenType::Star => { self.binary_number_operation(left, right, |a, b| a * b) },

            TokenType::Greater => { self.compare_number_operation(left, right, |a, b| a > b) },
            TokenType::GreaterEqual => { self.compare_number_operation(left, right, |a, b| a >= b) },
            TokenType::Less => { self.compare_number_operation(left, right, |a, b| a < b) },
            TokenType::LessEqual => { self.compare_number_operation(left, right, |a, b| a <= b) },

            TokenType::EqualEqual => Object::Bool(self.is_equal(left, right)),
            TokenType::BangEqual => Object::Bool(!self.is_equal(left, right)),

            _ => panic!("Invalid Binary Operation")
        }

    }

}

impl Interpreter {

    fn evaluate(&mut self, expression: &expr::Expr) -> Object {
        expression.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {

        match object {
            Object::Null => false,
            Object::Bool(v) => *v,
            _ => true
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
