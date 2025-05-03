use std::boxed::Box;
use super::token::{ Token, TokenType, Object, KEYWORDS };

pub trait Visitor<T> {

    fn visit_binary_expr(&mut self, expr: &Binary) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary) -> T;

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

    pub fn accept<R>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary { .. } => visitor.visit_binary_expr(self),
            Expr::Grouping { .. } => visitor.visit_grouping_expr(self),
            Expr::Literal { .. } => visitor.visit_literal_expr(self),
            Expr::Unary { .. } => visitor.visit_unary_expr(self),
        }
    }

}

