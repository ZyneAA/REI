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
    InvalidRangeType,
    NotCallable, InvalidArguments { token: T},
    ErrorInNativeFn { msg: String }

}

impl<T> fmt::Display for RuntimeError<T>
where T: fmt::Debug + fmt::Display
{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            RuntimeError::ErrorInNativeFn { msg } => write!(f, "{} | {}", util::red_colored("Error In Native Function"), msg),
            RuntimeError::InvalidArguments { token} => write!(f, "{} | {}", util::red_colored("Invalid Callable Argument Number | Argument don't match the callable's parameters"), token),
            RuntimeError::NotCallable => write!(f, "{}", util::red_colored("Invalid Callable | Can only call functions and classes")),
            RuntimeError::InvalidRange => write!(f, "{}", util::red_colored("Invalid Range | The starting point must be samller than the ending point")),
            RuntimeError::InvalidRangeType => write!(f, "{}", util::red_colored("Invalid Range Types | Both the start and the end must be Numbers")),
            RuntimeError::InvalidOperator { token } => write!(f, "{} | {}", util::red_colored("Invalid Operator") ,token),
            RuntimeError::UnexpectedBinaryOperation { token } => write!(f, "{} | {}", util::red_colored("Unexpected Binary Operation"), token),
            RuntimeError::TypeMismatch { token } => write!(f, "{} | {}", util::red_colored("Type Mismatch | Both operands must be same type"), token),
            RuntimeError::UndefinedVariable { token } => write!(f, "{} | {}", util::red_colored("Undefined Variable"), token),
            RuntimeError::DividedByZero { token } => write!(f, "{} | {}", util::red_colored("Divided By Zero"), token),
            RuntimeError::OperandMustBeNumber { token } => write!(f, "{} | {}", util::red_colored("Operand must be a number"), token),
        }

    }

}

impl<T> std::error::Error for RuntimeError<T> where T: fmt::Debug + fmt::Display {}
