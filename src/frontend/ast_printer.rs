use super::expr;
use crate::backend::stmt;
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

    fn visit_assign_expr(&mut self, name: &Token, value: &expr::Expr) -> String {
        let name = format!("assign {}", name.lexeme);
        self.parenthesize(&name, &[value])
    }

    fn visit_variable_expr(&mut self, name: &Token) -> String {
        let name = format!("{}", name.lexeme);
        name
    }

    fn visit_logical_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> String {
        self.parenthesize(&format!("logical {}", operator.lexeme), &[left, right])
    }


}

impl stmt::Visitor<String> for AstPrinter {

    fn visit_block_stmt(&mut self, statements: &Vec<stmt::Stmt>) -> String {
        let mut out = String::from("(block");
        for stmt in statements {
            out.push(' ');
            out.push_str(&stmt.accept(self));
        }
        out.push(')');
        out
    }

    fn visit_expression_stmt(&mut self, expression: &expr::Expr) -> String {
        format!("(expr {})", expression.accept(self))
    }

    fn visit_print_stmt(&mut self, expression: &expr::Expr) -> String {
        format!("(print {})", expression.accept(self))
    }

    fn visit_println_stmt(&mut self, expression: &expr::Expr) -> String {
        format!("(println {})", expression.accept(self))
    }

    fn visit_let_stmt(&mut self, name: &Token, initializer: &expr::Expr) -> String {
        format!("(let {} {})", name.lexeme, initializer.accept(self))
    }
    fn visit_if_stmt(&mut self, condition: &expr::Expr, then_branch: &stmt::Stmt, else_branch: &Option<Box<stmt::Stmt>>) -> String {

        let mut out = String::from("(if ");
        out.push_str(&condition.accept(self));
        out.push(' ');
        out.push_str(&then_branch.accept(self));
        if let Some(else_branch) = else_branch {
            out.push(' ');
            out.push_str(&else_branch.accept(self));
        }
        out.push(')');
        out

    }

    fn visit_while_stmt(&mut self, condition: &expr::Expr, body: &stmt::Stmt) -> String {
        format!("(while {} {})", condition.accept(self), body.accept(self))
    }


}

impl AstPrinter {

    pub fn print_ast(&mut self, statements: Vec<stmt::Stmt>) {

        for stmt in statements {
            let output = self.execute(&stmt);
            println!("{}", output);
        }

    }

    fn execute(&mut self, statement: &stmt::Stmt) -> String{
        statement.accept(self)
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
