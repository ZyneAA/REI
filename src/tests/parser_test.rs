use std::process;

use crate::frontend::{ parser, lexer };
use crate::crux::runner;

#[test]
pub fn lexer_token_test() {

    let test_file_location = "./src/tests/code/1.reix";
    let source = runner::Runner::read_file(test_file_location)
        .unwrap_or_else(|_| {
        process::exit(65);
    });

    let a = lexer::Lexer::new(&source);
    let tokens = a.scan_tokens();
    for i in tokens {
        println!("{}", i);
    }

}
