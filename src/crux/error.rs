use std::process;

use super::token::{ Token, TokenType };

pub trait ReiError<T> {

    fn throw_error(a: &T, msg: &str) -> !;
    fn error(a: &T, msg: &str);
    fn report(line: usize, place: &str, msg: &str);

}

pub struct ParseError;

pub struct SyntaxError;

impl ReiError<Token> for ParseError {

    fn throw_error(token: &Token, msg: &str) -> ! {

        Self::error(token, msg);
        process::exit(65)

    }

    fn error(token: &Token, msg: &str) {

        if token.token_type == TokenType::Eof {
            Self::report(token.line, "at end", msg);
        }
        else {
            Self::report(token.line, "at", msg);
        }

    }

    fn report(line: usize, place: &str, msg: &str) {
        eprintln!("[At line {}] Error {}: {}", line, place, msg);
    }

}

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
