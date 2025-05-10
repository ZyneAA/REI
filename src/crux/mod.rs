use std::{ process, env };

use crate::tools;

pub mod runner;
pub mod error;
pub mod token;

pub struct Rei;

impl Rei {

    #[allow(non_snake_case)]
    pub fn Ayanami() -> Result<(), Box<dyn std::error::Error>> {

        let args: Vec<String> = env::args().collect();
        let args_size = args.len();

        if args_size == 2 {

            if &args[1] == "gen" {
                tools::ast_generator::define_ast(
                "./src/frontend",
                "Expr",
                vec![
                    "Binary : Expr left, Token operator, Expr right",
                    "Grouping : Expr expression",
                    "Literal : Object value",
                    "Unary : Token operator, Expr right",
                    "Variable: Token name"
                ])?;
                tools::ast_generator::define_ast(
                "./src/backend",
                "Stmt",
                vec![
                    "Expression : Expr expression",
                    "Print : Expr expression",
                    "Let : Token name, Expr initializer"
                ])?;
            }
            else {

                let source = runner::Runner::read_file(&args[1])
                    .unwrap_or_else(|_| {
                        eprintln!("File not found");
                        process::exit(65);
                });
                runner::Runner::run(&source)?;

            }

            Ok(())

        }
        else if args_size == 3 && args[1] == "test" {

            let test_file_location = format!("./src/tests/code/{}.reix", &args[2]);
            let source = runner::Runner::read_file(&test_file_location)
                    .unwrap_or_else(|_| {
                        eprintln!("File not found: {}", test_file_location);
                        process::exit(65);
                });
                runner::Runner::run(&source)?;

            Ok(())

        }
        else {

            runner::Runner::run_prompt();
            Ok(())

        }


    }

}
