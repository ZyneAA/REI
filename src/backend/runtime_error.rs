use std::fmt;

use crate::crux::token::Token;

#[derive(Debug)]
pub enum RuntimeError {

    TypeMismatch {
        token: Token,
    },
    UndefinedVariable {
        token: Token,
    },
    DivideByZero {
        token: Token,
    },
    OperandMustBeNumber {
        token: Token,
    },
    UnexpectedRuntimeError {
        token: Token
    },
    InvalidOperator {
        token: Token
    }

}

impl fmt::Display for RuntimeError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            RuntimeError::InvalidOperator { token } => write!(f, "Invalid operator: {}", token),
            RuntimeError::UnexpectedRuntimeError { token } => write!(f, "Unexpected runtime error: {}", token),
            RuntimeError::TypeMismatch { token } => write!(f, "Type mismatch: {}", token),
            RuntimeError::UndefinedVariable { token } => write!(f, "Undefined variable: {}", token),
            RuntimeError::DivideByZero { token } => write!(f, "Divide by zero: {}", token),
            RuntimeError::OperandMustBeNumber { token } => write!(f, "Operand mut be a number: {}", token),
        }

    }

}

impl std::error::Error for RuntimeError {}
