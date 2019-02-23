use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
struct AocBox {
    i: usize,
    id: String
}

fn main() -> Result<()> {
    let inputs = BufReader::new(File::open("input.txt")?).lines().filter_map(|line| line.ok());
    let mut hashset = HashSet::new();

    for input in inputs {
        for i in 0..input.len()-1 {
            let (head, tail) = input.split_at(i);
            let id =  head.to_owned() + tail.get(1..).unwrap();
            let aocbox = AocBox { i, id };
            if hashset.contains(&aocbox) {
                println!("{}", aocbox.id);
                return Ok(());
            }
            hashset.insert(aocbox);
        }
    }

    Ok(())
}
