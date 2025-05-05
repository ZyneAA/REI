use std::boxed::Box;

use crate::core::token::Token;

pub enum Expr {

    Binary(Box<BinaryExpr>),

}

pub struct BinaryExpr {

    left: Expr,
    operator: Token,
    right: Expr

}

impl BinaryExpr {

    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        BinaryExpr { left, operator, right }
    }

}
