use std::fmt;

use crate::crux::token::Token;

pub mod control_flow;
pub mod runtime_error;

#[derive(Debug)]
pub enum ExecSignal {
    RuntimeError(runtime_error::RuntimeError<Token>),
    ControlFlow(control_flow::ControlFlow),
}

impl fmt::Display for ExecSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecSignal::RuntimeError(err) => write!(f, "Runtime Error: {}", err),
            ExecSignal::ControlFlow(flow) => write!(f, "Control Flow: {}", flow),
        }
    }
}
impl std::error::Error for ExecSignal {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExecSignal::RuntimeError(err) => Some(err),
            // ControlFlow is not an actual error so no source
            ExecSignal::ControlFlow(_) => None,
        }
    }
}
