use std::cell::RefCell;
use std::fmt;
use std::io;
use std::rc::Rc;
use std::thread;

use crate::crux::token::Object;
use crate::crux::util;

use crate::backend::stack_trace::ExecContext;

#[derive(Debug)]
pub struct RuntimeError<T> {
    pub err_type: RuntimeErrorType<T>,
    pub stack_trace: Rc<RefCell<ExecContext>>,
}

impl<T> RuntimeError<T> {
    pub fn new(err_type: RuntimeErrorType<T>, stack_trace: Rc<RefCell<ExecContext>>) -> Self {
        RuntimeError {
            err_type,
            stack_trace,
        }
    }
}

impl<T> fmt::Display for RuntimeError<T>
where
    T: fmt::Debug + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let current_thread = thread::current();
        let current_thread_name = current_thread.name().unwrap_or("main");
        let current_thread_id = current_thread.id();
        use RuntimeErrorType::*;

        match &self.err_type {
            TypeMismatch { .. }
            | UndefinedVariable { .. }
            | UndefinedProperty { .. }
            | DividedByZero { .. }
            | OperandMustBeNumber { .. }
            | UnexpectedBinaryOperation { .. }
            | InvalidOperator { .. }
            | InvalidArguments { .. } => {
                let fmt_report = util::red_colored(&format!(
                    "{} '{}' {:?} --- {}",
                    "Exception occured in", current_thread_name, current_thread_id, self.err_type
                ));
                write!(f, "{}", fmt_report)?
            }
            CustomMsg { msg } => {
                let single_line = msg.replace('\n', "\n◼︎ ");
                let fmt_report = util::red_colored(&format!(
                    "{} '{}' {:?} \n\n◼︎ {}\n",
                    "Exception occured in", current_thread_name, current_thread_id, single_line
                ));
                write!(f, "{}", fmt_report)?
            }
            _ => {
                let fmt_report = util::red_colored(&format!(
                    "{} '{}' {:?} --- {}",
                    "Exception occured in", current_thread_name, current_thread_id, self.err_type
                ));
                write!(f, "{}", fmt_report)?
            }
        }

        // Then stack trace
        writeln!(
            f,
            "  {}\n",
            self.stack_trace.borrow_mut().format_stack_trace()
        )
    }
}

impl<T> std::error::Error for RuntimeError<T> where T: fmt::Debug + fmt::Display {}

#[derive(Debug)]
pub enum RuntimeErrorType<T> {
    TypeMismatch { token: T },
    UndefinedVariable { token: T },
    UndefinedProperty { token: T },
    DividedByZero { token: T },
    OperandMustBeNumber { token: T },
    UnexpectedBinaryOperation { token: T },
    InvalidOperator { token: T },
    InvalidRange,
    InvalidRangeType { start: Object, end: Object },
    NotCallable,
    InvalidArguments { token: T },
    PropertyError,
    ErrorInNativeFn { msg: String },
    ErrorInReflection { msg: String },
    IoError { error: io::Error },
    ParentClassError { msg: String },
    CustomMsg { msg: String },
}

impl<T> fmt::Display for RuntimeErrorType<T>
where
    T: fmt::Debug + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeErrorType::IoError { error } => write!(f, "{} {}", util::red_colored("IO Error"), error),
            RuntimeErrorType::ErrorInNativeFn { msg } => write!(f, "{} {}", util::red_colored("Error In Native Function"), msg),
            RuntimeErrorType::ErrorInReflection { msg } => write!(f, "{} {}", util::red_colored("Error In Reflection"), msg),
            RuntimeErrorType::InvalidArguments { token} => write!(f, "{} {}", util::red_colored("Invalid Callable Argument Number | Arguments don't match the callable's parameters"), token),
            RuntimeErrorType::NotCallable => write!(f, "{}", util::red_colored("Invalid Callable | Can only call functions and classes")),
            RuntimeErrorType::InvalidRange => write!(f, "{}", util::red_colored("Invalid Range | The starting point must be smaller than the ending point")),
            RuntimeErrorType::InvalidRangeType { start, end } => write!(f, "{} {}..{}", util::red_colored("Invalid Range Types  Both the start and the end must be Numbers"), start, end),
            RuntimeErrorType::PropertyError => write!(f, "{}", util::red_colored("Property Error | Cannot access property on non-object type")),
            RuntimeErrorType::InvalidOperator { token } => write!(f, "{} {}", util::red_colored("Invalid Operator") ,token),
            RuntimeErrorType::UnexpectedBinaryOperation { token } => write!(f, "{} {}", util::red_colored("Unexpected Binary Operation"), token),
            RuntimeErrorType::TypeMismatch { token } => write!(f, "{} {}", util::red_colored("Type Mismatch | Both operands must be same type"), token),
            RuntimeErrorType::UndefinedVariable { token } => write!(f, "{} {}", util::red_colored("Undefined Variable"), token),
            RuntimeErrorType::UndefinedProperty { token } => write!(f, "{} {}", util::red_colored("Undefined Property"), token),
            RuntimeErrorType::DividedByZero { token } => write!(f, "{} {}", util::red_colored("Divided By Zero"), token),
            RuntimeErrorType::OperandMustBeNumber { token } => write!(f, "{} {}", util::red_colored("Operand must be a number"), token),
            RuntimeErrorType::ParentClassError { msg } => write!(f, "{}", util::red_colored(msg)),
            RuntimeErrorType::CustomMsg { msg } => write!(f, "{}", util::red_colored(msg))
        }
    }
}

impl<T> std::error::Error for RuntimeErrorType<T> where T: fmt::Debug + fmt::Display {}
