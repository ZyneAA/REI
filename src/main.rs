use std::io::Result;

mod core;
mod frontend;
mod tools;

#[cfg(test)]
mod tests;

fn main() -> Result<()> {

    core::Rei::Ayanami()

}


