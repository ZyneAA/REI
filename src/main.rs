use std::{ process, env, io::Result };

mod frontend;
mod tools;

#[cfg(test)]
mod tests;

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();
    let args_size = args.len();

    let runner = frontend::runner::Runner::new();

    if args_size > 2 {

        println!("Usage: ggs [filename]");
        Ok(())

    }
    else if args_size == 2 {

        let source = frontend::read_file(&args[1])
            .unwrap_or_else(|_| {
                eprintln!("No file found");
                process::exit(65);
        });

        runner.run(&source);
        tools::ast_generator::define_ast(
            "./src/frontend",
            "Expr",
            vec![
                "Binary : Expr left, Token operator, Expr right",
                "Grouping : Expr expression",
                "Literal : Object value",
                "Unary : Token operator, Expr right"
            ]
        )?;
        Ok(())

    }
    else {

        frontend::run_prompt();
        Ok(())

    }

}


