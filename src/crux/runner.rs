use std::{ fs, io::{ self, Write } };

use super::util;

use crate::frontend::lexer;
use crate::frontend::parser::Parser;

use crate::backend::interpreter::Interpreter;

pub struct Runner;

impl Runner {

    pub fn run(source: &str, location: &str) {

        let lexer = lexer::Lexer::new(source);
        let tokens = lexer.scan_tokens();

        let mut parser = Parser::new(tokens);
        let location =  util::red_colored(&format!("Error in{}", location));

        match parser.parse() {

            Ok(statements) => {
                let mut interpreter = Interpreter::new();
                if let Err(e) = interpreter.interpret(statements) {
                    eprintln!("{}\n{}", source, e);
                }
            }
            Err(e) => {
                eprintln!("{}\n{}", location, e);
            }

        }

    }

    pub fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
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
