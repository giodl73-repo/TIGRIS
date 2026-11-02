use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut request = String::new();
    io::stdin().take(524_289).read_to_string(&mut request)?;
    println!("{}", tigris_parliament_workbench::evaluate(&request));
    Ok(())
}
