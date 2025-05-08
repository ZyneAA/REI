use std::fmt;

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
    }

}

impl<T> fmt::Display for RuntimeError<T>
where T: fmt::Debug + fmt::Display
{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            RuntimeError::InvalidOperator { token } => write!(f, "Invalid operator: {}", token),
            RuntimeError::UnexpectedBinaryOperation { token } => write!(f, "Unexpected binary operation: {}", token),
            RuntimeError::TypeMismatch { token } => write!(f, "Type mismatch: {}", token),
            RuntimeError::UndefinedVariable { token } => write!(f, "Undefined variable: {}", token),
            RuntimeError::DividedByZero { token } => write!(f, "Divide by zero: {}", token),
            RuntimeError::OperandMustBeNumber { token } => write!(f, "Operand mut be a number: {}", token),
        }

    }

}

impl<T> std::error::Error for RuntimeError<T> where T: fmt::Debug + fmt::Display {}
