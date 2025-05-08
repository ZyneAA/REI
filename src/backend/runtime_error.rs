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
            RuntimeError::InvalidOperator { token } => write!(f, "{} | {}", self.red_colored("Invalid operator") ,token),
            RuntimeError::UnexpectedBinaryOperation { token } => write!(f, "{} | {}", self.red_colored("Unexpected binary operation"), token),
            RuntimeError::TypeMismatch { token } => write!(f, "{} | {}", self.red_colored("Type mismatch"), token),
            RuntimeError::UndefinedVariable { token } => write!(f, "{} | {}", self.red_colored("Undefined variable"), token),
            RuntimeError::DividedByZero { token } => write!(f, "{} | {}", self.red_colored("Divided by zero"), token),
            RuntimeError::OperandMustBeNumber { token } => write!(f, "{} | {}", self.red_colored("Operand must be a number"), token),
        }

    }

}

impl<T> RuntimeError<T> {

    fn red_colored(&self, text: &str) -> String{

        format!("\x1b[31m{}\x1b[0m", text)

    }

}

impl<T> std::error::Error for RuntimeError<T> where T: fmt::Debug + fmt::Display {}
