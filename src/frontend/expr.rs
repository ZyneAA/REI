use std::boxed::Box;
use crate::crux::token::{ Token, TokenType, Object, KEYWORDS };

pub trait Visitor<T> {

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &Object) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;

}

pub enum Expr {

    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Grouping {
        expression: Box<Expr>,
    },

    Literal {
        value: Object,
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
    },

}

impl Expr {

    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {

        match self {
            Expr::Binary { left, operator, right } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }

    }

}

