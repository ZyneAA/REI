use std::boxed::Box;
use crate::crux::token::{ Token, TokenType, Object, KEYWORDS };
use crate::frontend::expr::Expr;

pub trait Visitor<T> {

    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;

}

pub enum Stmt {

    Expression {
        expression: Box<Expr>,
    },

    Print {
        expression: Box<Expr>,
    },

}

impl Stmt {

    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {

        match self {
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
        }

    }

}

