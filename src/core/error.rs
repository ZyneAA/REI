use std::process;

pub struct Error;

impl Error {

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
        process::exit(65);
    }

    fn report(&mut self, line: usize, place: &str, message: &str) {

        println!("At line {} | Error {}: {}", line, place, message);

    }

}
