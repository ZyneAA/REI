use std::fmt;

use crate::crux::token::Object;

#[derive(Debug)]
pub enum ControlFlow {

    Return(Object),
    Break, Continue

}

impl fmt::Display for ControlFlow {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            ControlFlow::Return(obj) => write!(f, "return {}", obj),
            ControlFlow::Break => write!(f, "break"),
            ControlFlow::Continue => write!(f, "continue"),
        }

    }

}
