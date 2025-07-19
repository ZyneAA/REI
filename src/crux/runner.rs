use std::env;
use std::path::Path;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use super::util;
use crate::crux::error::ParseError;

use crate::frontend::lexer;
use crate::frontend::parser::Parser;

use crate::backend::interpreter::Interpreter;
use crate::backend::resolver::Resolver;

pub struct Runner;

impl Runner {
    pub fn run(source: &str, location: &str) {
        let mut current_file = Some(PathBuf::from(location));

        let lexer = lexer::Lexer::new(source);
        let tokens = lexer.scan_tokens();

        let mut global_expr_id_counter = 0;
        let mut syntax_errors: Vec<ParseError> = vec![];

        let mut parser = Parser::new(
            tokens,
            &mut current_file,
            &mut global_expr_id_counter,
            &mut syntax_errors,
        );
        let location = util::red_colored(&format!("Error in {}", location));

        let stmts = parser.parse();

        if syntax_errors.len() > 0 {
            for i in syntax_errors {
                println!("{}\n{}\n", location, i);
            }
            return;
        }

        let mut interpreter = match Interpreter::new(current_file) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("{}", e);
                panic!();
            }
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

    pub fn new_project(&self, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir(project_name)?;
        fs::create_dir(format!("./{}/lib", project_name))?;

        let std_src = self.get_rei_std_path();
        let std_dst = format!("{}/lib/std", project_name);
        self.copy_dir_all(&std_src, &std_dst)?;

        let main = format!("{}/main.reix", project_name);
        let git_ignore = format!("{}/.gitignore", project_name);

        fs::write(main, "println \"Hello, world!\";")?;
        fs::write(git_ignore, "/lib/std")?;

        Ok(())
    }

    fn copy_dir_all(&self, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(&src)? {
            let entry = entry?;
            let ty = entry.file_type()?;

            let src_path = entry.path();
            let dst_path = dst.as_ref().join(entry.file_name());

            if ty.is_dir() {
                self.copy_dir_all(src_path, dst_path)?;
            } else {
                fs::copy(src_path, dst_path)?;
            }
        }
        Ok(())
    }

    pub fn install_stdlib(&self) -> io::Result<()> {
        let src = Path::new("src/std");
        let dst = self.get_rei_std_path();
        println!("Installing stdlib to {:?}", dst);
        self.copy_dir_all(src, dst)
    }

    fn get_rei_std_path(&self) -> PathBuf {
        if let Ok(custom_path) = env::var("REI_HOME") {
            return PathBuf::from(custom_path).join("std");
        }

        #[cfg(target_os = "linux")]
        return PathBuf::from("/usr/share/rei/std");

        #[cfg(target_os = "macos")]
        return PathBuf::from("/usr/local/share/rei/std");

        #[cfg(target_os = "windows")]
        return PathBuf::from("C:\\ProgramData\\rei\\std");
    }
}
