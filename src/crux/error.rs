use std::{ process, fmt };

use crate::crux::util;
use super::token::Token;

pub trait ReiError<T> {

    fn throw_error(a: &T, msg: &str) -> !;
    fn error(a: &T, msg: &str);
    fn report(line: usize, place: &str, msg: &str);

}

pub struct SyntaxError;

#[derive(Debug)]
pub enum ParseError {

    SyntaxError {
        token: Token,
        message: String,
    },

}

impl fmt::Display for ParseError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            ParseError::SyntaxError { token, message } => {
                write!(f, "{} | {} at {}:{}", util::red_colored("Syntax error"), message, token.line, token.place)
            },

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
