use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let inputs = BufReader::new(File::open("input.txt")?).lines().filter_map(|line| line.ok());

    let mut count_two = 0;
    let mut count_three = 0;
    for input in inputs {
        let (two, three) = count(input);
        if two { count_two += 1 };
        if three { count_three += 1 };
    }
    println!("{}", count_two * count_three);

    Ok(())
}

fn count(input: String) -> (bool, bool) {
    let mut counter = HashMap::new();
    for char in input.chars() {
        let counter = counter.entry(char).or_insert(0);
        *counter += 1;
    }
    let values = counter.values().collect::<Vec<_>>();

    (values.contains(&&2), values.contains(&&3))
}
