pub fn red_colored(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text)
}

pub fn green_colored(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

pub fn yellow_colored(text: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", text)
}

pub fn blue_colored(text: &str) -> String {
    format!("\x1b[94m{}\x1b[0m", text)
}


pub fn white_colored(text: &str) -> String {
    format!("\x1b[1;37m{}\x1b[0m", text)
}
