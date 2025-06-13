use std::{ process, env };

use crate::tools;

pub mod runner;
pub mod error;
pub mod token;
pub mod util;

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
                    "Assign: Token name, Expr value",
                    "Binary : Expr left, Token operator, Expr right",
                    "Call : Expr callee, Token paren, Vec<Expr> arguments",
                    "Get : Expr object, Token name",
                    "Grouping : Expr expression",
                    "Literal : Object value",
                    "Logical : Expr left, Token operator, Expr right",
                    "Set : Expr object, Token name, Expr value",
                    "Base : Token keyword, Token method",
                    "This: Token keyword",
                    "Unary : Token operator, Expr right",
                    "Variable : Token name",
                    "Range : Expr start, Expr end"
                ])?;
                tools::ast_generator::define_ast(
                "./src/backend",
                "Stmt",
                vec![
                    "Block : Vec<Stmt> statements",
                    "Class : Token name, Expr superclass, Vec<Function> methods",
                    "Expression : Expr expression",
                    "Function : Token name, Vec<Token> params, Vec<Stmt> body",
                    "If : Expr condition, Stmt then_branch, Option<Box<Stmt>> else_branch",
                    "Print : Expr expression",
                    "Return : Token keyword, Option<Box<Expr>> value",
                    "PrintLn : Expr expression",
                    "Let : Token name, Expr initializer",
                    "While : Expr condition, Stmt body",
                ])?;
            }

            else if &args[1] == "setup" {

                let runner = runner::Runner;
                runner.install_stdlib()?;

            }

            else {

                let source = runner::Runner::read_file(&args[1])
                    .unwrap_or_else(|_| {
                        eprintln!("File not found");
                        process::exit(65);
                });
                runner::Runner::run(&source, &args[1]);

            }

            Ok(())

        }

        else if args_size == 3 && args[1] == "new" {
            let runner = runner::Runner;
            runner.new_project(&args[2])?;
            Ok(())
        }

        else if args_size == 3 && args[1] == "test" {

            let test_file_location = format!("./src/tests/code/{}.reix", &args[2]);
            let source = runner::Runner::read_file(&test_file_location)
                    .unwrap_or_else(|_| {
                        eprintln!("File not found: {}", test_file_location);
                        process::exit(65);
                });
                runner::Runner::run(&source, &test_file_location);

            Ok(())

        }
        else {

            runner::Runner::run_prompt();
            Ok(())

        }


    }

}
