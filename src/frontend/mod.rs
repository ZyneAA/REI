use std::{ fs, io::{ self, Result, Write } };

pub mod token;
pub mod runner;
pub mod lexer;
pub mod ast;
pub mod expr;
pub mod ast_printer;

/// Reading the file from the command line args
pub fn read_file(path: &str) -> Result<String> {

    let content = fs::read_to_string(path)?;
    let lexer = lexer::Lexer::new(&content);
    let tokens = lexer.scan_tokens();
    for i in tokens {
        i.show();
    }

    Ok(content)

}

/// For REPL
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

