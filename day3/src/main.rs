use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, HashMap};

fn main() -> Result<()> {
    slove2()
}

fn slove2() -> Result<()> {
    let claims = BufReader::new(File::open("input.txt")?).lines()
        .filter_map(|line| line.ok().map(|input| Claim::new(input)))
        .collect::<Vec<_>>();

    let mut points = HashMap::new();
    for claim in claims.iter() {
        for point in claim.points() {
            points.entry(point).or_insert_with(Vec::new).push(claim.id);
        }
    }

    let claim_ids = claims.iter().map(|claim| &claim.id).collect::<HashSet<_>>();
    let dup_ids = points.iter().filter(|(_, ids)| ids.len() > 1).flat_map(|(_, ids)| ids).collect::<HashSet<_>>();
    for single_id in claim_ids.difference(&dup_ids) {
        println!("{:?}", single_id);
    }

    Ok(())
}

fn slove1() -> Result<()> {
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

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    id: usize,
}

#[derive(Clone, Debug)]
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
