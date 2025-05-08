use std::{ fs, io::{ self, Write } };

use crate::frontend::lexer;
use crate::frontend::parser::Parser;

use crate::backend::interpreter::Interpreter;

pub struct Runner;

impl Runner {

    pub fn run(source: &str) -> Result<(), Box<dyn std::error::Error>> {

        let lexer = lexer::Lexer::new(source);
        let tokens = lexer.scan_tokens();

        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(v) => {
                let mut interpreter = Interpreter;
                match interpreter.interpret(v) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        eprintln!("{}", e);
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                Err(Box::new(e))
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
