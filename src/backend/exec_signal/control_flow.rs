use std::fmt;

use crate::crux::token::Object;

#[derive(Debug)]
pub enum ControlFlow {
    Return(Object),
    Break, Continue
    // you can add Break/Continue later if you're into loops and existential dread
}

impl fmt::Display for ControlFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ControlFlow::Return(obj) => write!(f, "return {}", obj),
            ControlFlow::Break => write!(f, "break"),
            ControlFlow::Continue => write!(f, "continue"),

            // if you add Break/Continue later:
            // ControlFlow::Break => write!(f, "break"),
            // ControlFlow::Continue => write!(f, "continue"),
        }
    }
}
