use std::{fmt, process};

use super::token::Token;
use crate::crux::util;

pub trait ReiError<T> {
    fn throw_error(a: &T, msg: &str) -> !;
    fn error(a: &T, msg: &str);
    fn report(line: usize, place: &str, msg: &str);
}

pub struct SyntaxError;

#[derive(Debug, Clone)]
pub enum ParseError {
    SyntaxError { token: Token, message: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::SyntaxError { token, message } => {
                let fmt_err = format!(
                    "Syntax Error | {} at {}:{}",
                    message, token.line, token.place
                );
                write!(f, "{}", util::red_colored(&fmt_err),)
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl ReiError<(usize, usize)> for SyntaxError {
    fn throw_error(pos: &(usize, usize), msg: &str) -> ! {
        Self::error(pos, msg);
        process::exit(65)
    }

    fn error(pos: &(usize, usize), msg: &str) {
        let place = pos.1.to_string();
        Self::report(pos.0, &place, msg);
    }

    fn report(line: usize, place: &str, msg: &str) {
        eprintln!("[Unexpected character at -> {}:{}] {}", line, place, msg);
    }
}
