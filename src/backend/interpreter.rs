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

            TokenType::Minus => { self.binary_number_operation(left, right, |a, b| a - b) },
            TokenType::Plus => { self.binary_number_operation(left, right, |a, b| a + b) },
            TokenType::Slash => { self.binary_number_operation(left, right, |a, b| a / b) },
            TokenType::Star => { self.binary_number_operation(left, right, |a, b| a * b) },

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

    fn number_operation<F>(&self, left: Object, operator: F, right: Object) -> Object
    where F: Fn(f64, f64) -> f64
    {

        if let (Object::Number(a), Object::Number(b)) = (left, right) {
            Object::Number(operator(a, b))
        }
        else {
            panic!("Operands must be numbers");
        }

    }

    fn binary_number_operation<F>(&self, a: Object, b: Object, op: F) -> Object
    where F: Fn(f64, f64) -> f64,
    {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => Object::Number(op(x, y)),
            _ => panic!("Runtime error: Both operands must be numbers."),
        }
    }

}
