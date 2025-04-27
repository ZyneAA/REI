use std::process;

pub struct Runner {
    pub had_err: bool
}

impl Runner {

    pub fn new() -> Self {
        Runner{ had_err: false }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
        process::exit(65);
    }

    fn report(&mut self, line: usize, place: &str, message: &str) {

        println!("At line {} | Error {}: {}", line, place, message);
        self.had_err = true;

    }

    pub fn run(&self, source: &str) {

        if self.had_err {
            process::exit(65);
        }

    }

}
