use std::ops::RangeInclusive;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

struct Assignments {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

impl FromStr for Assignments {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        REGEX.captures(s)
            .ok_or(io::Error::new(ErrorKind::InvalidInput, "failed to parse ranges"))
            .and_then(|cap| {
            Ok(
                Assignments {
                    first: RangeInclusive::new(
                         cap[1].parse().unwrap(),
                         cap[2].parse().unwrap(),
                    ),
                    second: RangeInclusive::new (
                         cap[3].parse().unwrap(),
                         cap[4].parse().unwrap(),
                    ),
                }
            )
        })
    }
}

const REGEX_STR: &str = r"(\d+)-(\d+),(\d+)-(\d+)";
lazy_static! {
    static ref REGEX: Regex = Regex::new(REGEX_STR).unwrap();
}
fn one() -> io::Result<()> {
    let file = File::open("./four/input.txt")?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let line = line?;
        let assignments: Assignments = line.parse()?;
        if assignments.first.contains(&assignments.second.start()) && assignments.first.contains(&assignments.second.end())  {
            count += 1;
        } else if assignments.second.contains(&assignments.first.start()) && assignments.second.contains(&assignments.first.end()) {
            count += 1;
        }
    }
    println!("number of redundant assignments: {}", count);
    Ok(())
}
fn two() -> io::Result<()> {
    let file = File::open("./four/input.txt")?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let line = line?;
        let assignments: Assignments = line.parse()?;
        if assignments.first.contains(&assignments.second.start()) || assignments.first.contains(&assignments.second.end())  {
            count += 1;
        } else if assignments.second.contains(&assignments.first.start()) || assignments.second.contains(&assignments.first.end()) {
            count += 1;
        }
    }
    println!("number of redundant assignments: {}", count);
    Ok(())
}

fn main() {
    one().unwrap();
    two().unwrap();
}