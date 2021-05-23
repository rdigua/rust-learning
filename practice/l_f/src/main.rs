use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    let f=File::open("Cargo.toml")?;
    let mut buf=BufReader::new(f);
    let mut con=String::new();
    buf.read_to_string(&mut con)?;
    println!("{}",con);
    Ok(())
}
