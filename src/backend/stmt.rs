use std::boxed::Box;
use crate::crux::token::{ Token, TokenType, Object, KEYWORDS };
use crate::frontend::expr::Expr;

pub trait Visitor<T> {

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> T;
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Box<Stmt>>) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
    fn visit_println_stmt(&mut self, expression: &Expr) -> T;
    fn visit_let_stmt(&mut self, name: &Token, initializer: &Expr) -> T;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> T;

}

#[derive(Clone)]

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
        else_branch: Option<Box<Stmt>>,
    },

    Print {
        expression: Box<Expr>,
    },

    PrintLn {
        expression: Box<Expr>,
    },

    Let {
        name: Token,
        initializer: Box<Expr>,
    },

    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },

}

impl Stmt {

    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {

        match self {
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::If { condition, then_branch, else_branch } => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::PrintLn { expression } => visitor.visit_println_stmt(expression),
            Stmt::Let { name, initializer } => visitor.visit_let_stmt(name, initializer),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
        }

    }

}

