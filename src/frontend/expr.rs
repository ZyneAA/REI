use std::boxed::Box;
use crate::crux::token::{ Token, Object };

pub trait Visitor<T> {

    fn visit_assign_expr(&mut self, id: ExprId, name: &Token, value: &Expr) -> T;
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &Object) -> T;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_variable_expr(&mut self, id: ExprId, name: &Token) -> T;
    fn visit_range_expr(&mut self, start: &Expr, end: &Expr) -> T;

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(pub usize);

#[derive(Clone, Debug)]
pub enum Expr {

    Assign {
        id: ExprId,
        name: Token,
        value: Box<Expr>,
    },

    Binary {
        id: ExprId,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Call {
        id: ExprId,
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },

    Grouping {
        id: ExprId,
        expression: Box<Expr>,
    },

    Literal {
        id: ExprId,
        value: Object,
    },

    Logical {
        id: ExprId,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Unary {
        id: ExprId,
        operator: Token,
        right: Box<Expr>,
    },

    Variable {
        id: ExprId,
        name: Token,
    },

    Range {
        id: ExprId,
        start: Box<Expr>,
        end: Box<Expr>,
    },

}

impl Expr {

    pub fn id(&self) -> ExprId {

        match self {
            Expr::Assign { id, .. }
            | Expr::Binary { id, .. }
            | Expr::Call { id, .. }
            | Expr::Grouping { id, .. }
            | Expr::Literal { id, .. }
            | Expr::Logical { id, .. }
            | Expr::Unary { id, .. }
            | Expr::Variable { id, .. }
            | Expr::Range { id, .. } => id.clone(),
        }

    }

    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {

        match self {
            Expr::Assign { id, name, value } => visitor.visit_assign_expr(id.clone(), name, value),
            Expr::Binary { id: _, left, operator, right } => visitor.visit_binary_expr(left, operator, right),
            Expr::Call {  id: _, callee, paren, arguments } => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Grouping {  id: _, expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal {  id: _, value } => visitor.visit_literal_expr(value),
            Expr::Logical {  id: _, left, operator, right } => visitor.visit_logical_expr(left, operator, right),
            Expr::Unary {  id: _, operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { id, name } => visitor.visit_variable_expr(id.clone(), name),
            Expr::Range {  id: _, start, end } => visitor.visit_range_expr(start, end),
        }

    }

}

