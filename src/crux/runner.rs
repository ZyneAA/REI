use std::{ fs, io::{ self, Result, Write } };

use crate::frontend::lexer;

pub struct Runner;

impl Runner {

    pub fn run(source: &str) {

        let lexer = lexer::Lexer::new(source);
        let tokens = lexer.scan_tokens();
        for i in tokens {
            println!("{}", i.display())
        }


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
