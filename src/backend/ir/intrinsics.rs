use crate::frontend::*;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, Linkage, Module};
use std::collections::HashMap;
use std::slice;

use crate::crux::token::{ Object, TokenType, Token };
use crate::frontend::expr::Expr;
use crate::backend::environment::Environment;

struct FunctionTranslator<'a> {

    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,

}

impl <'a> FunctionTranslator<'a> {

    fn translate_expr(&mut self, expr: Expr) -> Value {

        match expr {

            Expr::Literal{ value: l } => {

                match l {
                    Object::Number(n) => {
                        self.builder.ins().f64const(n)
                    },
                    Object::Str(s) => {
                        let ptr = s.as_ptr() as i64;
                        self.builder.ins().iconst(types::I64, ptr)
                    },
                    Object::Bool(b) => {
                        let b = b as i64;
                        self.builder.ins().iconst(types::I8, Imm64::new(b))
                    },
                    Object::Null => {
                        self.builder.ins().iconst(types::I64, 0)
                    }
                }

            },

            Expr::Binary { left: lhs, operator: op, right: rhs } => {

                let lhs = self.translate_expr(*lhs);
                let rhs = self.translate_expr(*rhs);

                match op.token_type {
                    TokenType::Plus => {
                        self.builder.ins().iadd(lhs, rhs)
                    },
                    TokenType::Minus => {
                        self.builder.ins().isub(lhs, rhs)
                    },
                    TokenType::Star => {
                        self.builder.ins().imul(lhs, rhs)
                    },
                    TokenType::Slash => {
                        self.builder.ins().udiv(lhs, rhs)
                    },

                    TokenType::Greater => {
                        self.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs)
                    },
                    TokenType::Less => {
                        self.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs)
                    },
                    TokenType::EqualEqual => {
                        self.builder.ins().icmp(IntCC::Equal, lhs, rhs)
                    },
                    TokenType::BangEqual => {
                        self.builder.ins().icmp(IntCC::NotEqual, lhs, rhs)
                    },
                    TokenType::GreaterEqual => {
                        self.builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs)
                    },
                    TokenType::LessEqual => {
                        self.builder.ins().icmp(IntCC::SignedLessThanOrEqual, lhs, rhs)
                    },
                    _ => panic!("SHIT")
                }

            },

            Expr::Grouping { expression: g } => self.translate_expr(*g),

            Expr::Unary { operator: op, right: r } => {

                let r = self.translate_expr(*r);

                match op.token_type {
                    TokenType::Minus => {
                        let ty = self.builder.func.dfg.value_type(r);
                        if ty == types::F64 {
                            self.builder.ins().fneg(r)
                        }
                        else if ty == types::I64 {
                            self.builder.ins().ineg(r)
                        }
                        else {
                            panic!("Can't negate type: {:?}", ty);
                        }
                    }
                    TokenType::Bang => {
                        let ty = self.builder.func.dfg.value_type(r);
                        if ty == types::I8 {
                            let zero = self.builder.ins().iconst(types::I8, 0);
                            self.builder.ins().icmp(IntCC::Equal, r, zero)
                        }
                        else {
                            panic!("Logical NOT only valid on boolean (i8), got: {:?}", ty);
                        }
                    }
                    _ => panic!("Unknown unary operator: {:?}", op.token_type)
                }

            },

            Expr::Variable { name: Token } => {

            }

        }

    }

    fn check_number_operand(&self, operator: Token, operand: Object) -> bool {

        match operand {
            Object::Number(_) => true,
            _ => false
        }

    }

}
