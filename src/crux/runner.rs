use std::{ fs, io::{ self, Result, Write } };

use crate::frontend::lexer;
use crate::frontend::parser::Parser;
use crate::frontend::ast_printer::AstPrinter;

pub struct Runner;

impl Runner {

    pub fn run(source: &str) {

        let lexer = lexer::Lexer::new(source);
        let tokens = lexer.scan_tokens();

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let output = expr.accept(&mut AstPrinter);

        println!("{}", output)

    }

    pub fn read_file(path: &str) -> Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    pub fn run_prompt() {

        loop {

            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            println!("> {}", input);
        }

    }

}
