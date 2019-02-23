use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Id(u32);

#[derive(Debug)]
struct Records {
    records: HashMap<Id, Vec<u32>>,
    current: Option<Id>,
    start_minute: Option<u32>
}

impl Records {
    fn new() -> Self {
        Records { records: HashMap::new(), current: None, start_minute: None }
    }
}

fn main() -> Result<()> {
    let mut lines = BufReader::new(File::open("input.txt")?).lines()
        .filter_map(|line| line.ok().map(|input| parse(input)))
        .collect::<Vec<_>>();
    lines.sort_by_key(|&(dt, _)| dt);

    let mut records = Records::new();
    for (dt, text) in lines {
        if text.starts_with("Guard") {
            if let Some(id) = get_id(text) {
                records.current = Some(Id(id));
            }
        }
        else if text.starts_with("falls asleep") {
            records.start_minute = Some(dt.minute());
        }
        else if text.starts_with("wakes up") {
            if let (Some(id), Some(start_minute)) = (records.current, records.start_minute) {
                let times = records.records.entry(id).or_insert(Vec::new());
                times.append(&mut (start_minute .. dt.minute()).collect::<Vec<_>>());
                records.start_minute = None;
            }
        }
    }

    records.records.iter()
        .map(|(id, times)| (id, frequently(times)))
        .max_by_key(|(_, time_and_count)| time_and_count.map_or(0, |(_, count)| count))
        .and_then(|(id, time_and_count)| time_and_count.map(|(time, _)| id.0 * time))
        .map(|result| println!("{}", result));

    Ok(())
}

fn parse(input: String) -> (DateTime<Utc>, String) {
    (
        input.get(1..17).and_then(|datetime| Utc.datetime_from_str(datetime, "%F %R").ok()).unwrap(),
        input.get(19..).unwrap().to_string()
    )
}

fn get_id(input: String) -> Option<u32> {
    input.split_whitespace().nth(1).unwrap().trim_start_matches("#").parse::<u32>().ok()
}

fn frequently(times: &Vec<u32>) -> Option<(u32, usize)> {
    (0 .. 60)
        .map(|i| (i, times.iter().filter(|&&n| n == i).count()))
        .max_by_key(|&(_, count)| count)
}
