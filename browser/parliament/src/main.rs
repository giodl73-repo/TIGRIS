use std::io::{self, Read};

fn main() -> io::Result<()> {
    let limit = tigris_parliament_workbench::REQUEST_BYTES;
    let mut bytes = Vec::new();
    io::stdin().take(limit as u64 + 1).read_to_end(&mut bytes)?;
    // Refuse by byte count BEFORE UTF-8 decoding: the admission prefix may end
    // inside a valid caller character. Route through the same Rust refusal.
    if bytes.len() > limit {
        println!(
            "{}",
            tigris_parliament_workbench::evaluate(&" ".repeat(limit + 1))
        );
        return Ok(());
    }
    let request = String::from_utf8(bytes)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    println!("{}", tigris_parliament_workbench::evaluate(&request));
    Ok(())
}
