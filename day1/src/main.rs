use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut result = 0;
    let mut frequencies = HashSet::new();

    let input = File::open("input.txt")?;
    let numbers = BufReader::new(input).lines()
        .filter_map(|line| line.ok())
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for number in numbers.iter().cycle() {
        result += number;
        if frequencies.contains(&result) {
            println!("{}", result);
            break
        }
        frequencies.insert(result);
    }

    Ok(())
}

fn _slove1() -> Result<()> {
    let input = File::open("input.txt")?;
    let mut result = 0;
    for line in BufReader::new(input).lines() {
        result += line?.parse::<i32>().unwrap();
    }
    println!("{}", result);

    Ok(())
}
