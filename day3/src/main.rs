use std::fs::File;
use std::io::{BufRead, BufReader, Result};
#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashSet;

fn main() -> Result<()> {
    let inputs = BufReader::new(File::open("input.txt")?).lines().filter_map(|line| line.ok());
    let mut points = HashSet::new();
    let mut overlap = HashSet::new();
    for input in inputs {
        for point in Claim::new(input).points() {
            if points.contains(&point) {
                overlap.insert(point);
            } else {
                points.insert(point);
            }
        }
    }
    println!("{}", overlap.len());
    Ok(())
}

#[derive(Debug)]
struct Claim {
    id:    usize,
    left:  usize,
    top:   usize,
    width: usize,
    height:usize,
}

impl Claim {
    fn new(text: String) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }

        let caps = RE.captures(&text).unwrap();
        Claim {
            id:    caps[1].parse().unwrap(),
            left:  caps[2].parse().unwrap(),
            top:   caps[3].parse().unwrap(),
            width: caps[4].parse().unwrap(),
            height:caps[5].parse().unwrap()
        }
    }

    fn points(&self) -> Vec<(usize, usize)> {
        (self.left .. self.width + self.left).flat_map(|x|
            (self.top .. self.height + self.top).map(move |y| (x, y))
        ).collect()
    }
}
