use crux::Rei;

mod backend;
mod crux;
mod frontend;
mod tools;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Rei::Ayanami()
}
