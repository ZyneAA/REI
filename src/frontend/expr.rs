use std::boxed::Box;
use crate::crux::token::{ Token, Object };

pub trait Visitor<T> {

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &Object) -> T;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_variable_expr(&mut self, name: &Token) -> T;
    fn visit_range_expr(&mut self, start: &Expr, end: &Expr) -> T;

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(pub usize);

#[derive(Clone, Debug)]
pub enum Expr {

    Assign {
        name: Token,
        value: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },

    Grouping {
        expression: Box<Expr>,
    },

    Literal {
        value: Object,
    },

    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    Variable {
        name: Token,
    },

    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },

}

impl Expr {

    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {

        match self {
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Binary { left, operator, right } => visitor.visit_binary_expr(left, operator, right),
            Expr::Call { callee, paren, arguments } => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Logical { left, operator, right } => visitor.visit_logical_expr(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
            Expr::Range { start, end } => visitor.visit_range_expr(start, end),
        }

    }

}

