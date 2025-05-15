use std::fmt;

use crate::crux::util;

#[derive(Debug)]
pub enum RuntimeError<T>
{

    TypeMismatch {
        token: T,
    },
    UndefinedVariable {
        token: T,
    },
    DividedByZero {
        token: T,
    },
    OperandMustBeNumber {
        token: T,
    },
    UnexpectedBinaryOperation {
        token: T
    },
    InvalidOperator {
        token: T
    },
    InvalidRange,
    InvalidRangeType

}

impl<T> fmt::Display for RuntimeError<T>
where T: fmt::Debug + fmt::Display
{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            RuntimeError::InvalidRangeType => write!(f, "{}", util::red_colored("Invalid Range types | Both the start and the end must be Numbers")),
            RuntimeError::InvalidRange => write!(f, "{}", util::red_colored("Invalid Range | The start point is bigger than the end point")),
            RuntimeError::InvalidOperator { token } => write!(f, "{} | {}", util::red_colored("Invalid operator") ,token),
            RuntimeError::UnexpectedBinaryOperation { token } => write!(f, "{} | {}", util::red_colored("Unexpected binary operation"), token),
            RuntimeError::TypeMismatch { token } => write!(f, "{} | {}", util::red_colored("Type mismatch, both operands must be same type"), token),
            RuntimeError::UndefinedVariable { token } => write!(f, "{} | {}", util::red_colored("Undefined variable"), token),
            RuntimeError::DividedByZero { token } => write!(f, "{} | {}", util::red_colored("Divided by zero"), token),
            RuntimeError::OperandMustBeNumber { token } => write!(f, "{} | {}", util::red_colored("Operand must be a number"), token),
        }

    }

}

impl<T> std::error::Error for RuntimeError<T> where T: fmt::Debug + fmt::Display {}
