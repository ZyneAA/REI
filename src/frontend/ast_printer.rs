use super::expr;
use crate::crux::token::{ Token, Object };

pub struct AstPrinter;

impl expr::Visitor<String> for AstPrinter {

    fn visit_binary_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&mut self, value: &Object) -> String {
        match value {
            Object::Number(n) => n.to_string(),
            Object::Str(s) => s.clone(),
            Object::Bool(b) => b.to_string(),
            Object::Null => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &expr::Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }

}

impl AstPrinter {

    pub fn print_ast(&mut self, expression: expr::Expr) -> String {

        let output = expression.accept(self);
        output

    }

    fn parenthesize(&mut self, name: &str, exprs: &[&expr::Expr]) -> String {

        let mut result = String::new();
        result.push('(');
        result.push_str(name);

        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self));
        }

        result.push(')');
        result

    }

}
