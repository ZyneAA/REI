use std::{ fs, io::{ self, Write } };

use crate::frontend::lexer;
use crate::frontend::parser::Parser;
use crate::frontend::ast_printer::AstPrinter;

use crate::backend::interpreter::Interpreter;

pub struct Runner;

impl Runner {

    pub fn run(source: &str) -> Result<(), Box<dyn std::error::Error>> {

        let lexer = lexer::Lexer::new(source);
        let tokens = lexer.scan_tokens();

        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;

        let mut less_gooo = Interpreter;
        let ast = expr.accept(&mut AstPrinter);

        match less_gooo.interpret(expr) {
            Ok(output) => {
                println!("Ast: {}\nOutput: {}", ast, output);
                Ok(())
            }
            Err(e) => {
                eprintln!("Runtime error: {}", e);
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
