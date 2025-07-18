use std::process;

use crate::crux::runner;
use crate::frontend::{ast_printer, lexer, parser};

#[test]
pub fn ast_test() {
    let test_file_location = "./src/tests/code/2.reix";
    let source = runner::Runner::read_file(test_file_location).unwrap_or_else(|_| {
        process::exit(65);
    });

    let a = lexer::Lexer::new(&source);
    let tokens = a.scan_tokens();
    let mut p = parser::Parser::new(tokens);
    let p = p.parse().unwrap();
    let mut printer = ast_printer::AstPrinter;
    printer.print_ast(p);
}
