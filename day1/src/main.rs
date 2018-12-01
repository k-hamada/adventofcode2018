use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let input = File::open("input.txt")?;
    let mut result = 0;
    for line in BufReader::new(input).lines() {
        result += line?.parse::<i32>().unwrap();
    }
    println!("{}", result);
    Ok(())
}
