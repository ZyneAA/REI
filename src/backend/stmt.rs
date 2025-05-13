use std::boxed::Box;
use crate::crux::token::{ Token, TokenType, Object, KEYWORDS };
use crate::frontend::expr::Expr;

pub trait Visitor<T> {

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> T;
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt) -> T;
    fn visit_else_stmt(&mut self, else_branch: &Stmt) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
    fn visit_let_stmt(&mut self, name: &Token, initializer: &Expr) -> T;

}

pub enum Stmt {

    Block {
        statements: Vec<Stmt>,
    },

    Expression {
        expression: Box<Expr>,
    },

    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
    },

    Else {
        else_branch: Box<Stmt>,
    },

    Print {
        expression: Box<Expr>,
    },

    Let {
        name: Token,
        initializer: Box<Expr>,
    },

}

impl Stmt {

    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {

        match self {
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::If { condition, then_branch } => visitor.visit_if_stmt(condition, then_branch),
            Stmt::Else { else_branch } => visitor.visit_else_stmt(else_branch),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Let { name, initializer } => visitor.visit_let_stmt(name, initializer),
        }

    }

}

