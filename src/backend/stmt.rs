use std::boxed::Box;

use crate::crux::token::Token;
use crate::frontend::expr::Expr;

pub trait Visitor<T> {

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> T;
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_class_stmt(&mut self, name: &Token, superclass_refs: &Vec<Expr>, methods: &Vec<Stmt>, static_methods: &Vec<Stmt>, expose: &bool) -> T;
    fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> T;
    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Box<Stmt>>) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
    fn visit_println_stmt(&mut self, expression: &Expr) -> T;
    fn visit_let_stmt(&mut self, name: &Token, initializer: &Expr) -> T;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> T;
    fn visit_return_stmt(&mut self, keyword: &Token, value: &Option<Box<Expr>>) -> T;
    fn visit_throw_stmt(&mut self, expression: &Box<Expr>) -> T;
    fn visit_break_stmt(&mut self) -> T;
    fn visit_continue_stmt(&mut self) -> T;

}

#[derive(Clone, Debug)]
pub enum Stmt {

    Block {
        statements: Vec<Stmt>,
    },

    Class {
        name: Token,
        superclass_refs: Vec<Expr>,
        methods: Vec<Stmt>,
        static_methods: Vec<Stmt>,
        expose: bool
    },

    Expression {
        expression: Box<Expr>,
    },

    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },

    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    Print {
        expression: Box<Expr>,
    },

    Return {
        keyword: Token,
        value: Option<Box<Expr>>,
    },

    Throw {
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

    Break, Continue

}

impl Stmt {

    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {

        match self {
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::Class { name, superclass_refs, methods, static_methods, expose } => visitor.visit_class_stmt(name, superclass_refs, methods, static_methods, expose),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Function { name, params, body } => visitor.visit_function_stmt(name, params, body),
            Stmt::If { condition, then_branch, else_branch } => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Return { keyword, value } => visitor.visit_return_stmt(keyword, value),
            Stmt::Throw { expression } => visitor.visit_throw_stmt(expression),
            Stmt::PrintLn { expression } => visitor.visit_println_stmt(expression),
            Stmt::Let { name, initializer } => visitor.visit_let_stmt(name, initializer),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Break => visitor.visit_break_stmt(),
            Stmt::Continue => visitor.visit_continue_stmt(),
        }

    }

}

