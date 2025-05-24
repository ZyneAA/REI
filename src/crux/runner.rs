use std::{ fs, io::{ self, Write } };

use super::util;

use crate::frontend::lexer;
use crate::frontend::parser::Parser;

use crate::backend::interpreter::Interpreter;
use crate::backend::resolver::Resolver;

pub struct Runner;

impl Runner {

    pub fn run(source: &str, location: &str) {

        let lexer = lexer::Lexer::new(source);
        let tokens = lexer.scan_tokens();

//        for i in &tokens {
//            println!("{}", i);
//        }

        let mut parser = Parser::new(tokens);
        let location =  util::red_colored(&format!("Error in {}", location));

        let stmts = parser.parse();

        if parser.is_error {
            for i in parser.errors {
                println!("{}\n{}\n", location, i);
            }
            return;
        }

        let mut interpreter = match Interpreter::new() {
            Ok(i) => i,
            Err(e) => { eprintln!("{}", e); panic!(); }
        };
        let mut resolver = Resolver::new(&mut interpreter);
        resolver.resolve(&stmts);
        if let Err(e) = interpreter.interpret(stmts) {
            eprintln!("{}", e);
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
