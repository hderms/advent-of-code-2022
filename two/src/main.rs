use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct RoundOne {
    opponent: Choice,
    suggested: Choice,
}

#[derive(Debug)]
struct RoundTwo {
    opponent: Choice,
    suggested: Outcome,
}

#[derive(Debug)]
enum Outcome {
    OpponentWin,
    YouWin,
    Draw,
}

fn to_ordinal_value(choice: &Choice) -> u32 {
    match choice {
        Choice::Rock => 0,
        Choice::Paper => 1,
        Choice::Scissors => 2,
    }
}

impl From<u32> for Choice {
    fn from(i: u32) -> Self {
            match i {
                0 => Choice::Rock,
                1 => Choice:: Paper,
                2 => Choice::Scissors,
                _ => panic!("can't convert {} to Choice", i)
            }
    }
}

fn determine_outcome(round: &RoundOne) -> Outcome {
    let opponent = to_ordinal_value(&round.opponent);
    let suggested = to_ordinal_value(&round.suggested);
    if opponent == suggested {
        return Outcome::Draw;
    }

    if (opponent % 3) == (suggested + 1) % 3 {
        Outcome::OpponentWin
    } else {
        Outcome::YouWin
    }
}

fn determine_choice(round: &RoundTwo) -> Choice {
    let opponent = to_ordinal_value(&round.opponent);
    let suggested_choice = match round.suggested {
        Outcome::OpponentWin => (opponent + 2) % 3,
        Outcome::YouWin => (opponent + 1) % 3,
        Outcome::Draw => opponent
    };
    suggested_choice.into()
}
fn choice_score(choice: &Choice) -> u32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3
    }
}

fn outcome_score(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::OpponentWin => 0,
        Outcome::Draw => 3,
        Outcome::YouWin => 6,
    }
}

const LINE_REGEX: &str = r"([ABC]) ([XYZ])";
lazy_static! {
    static ref REGEX: Regex = Regex::new(LINE_REGEX).unwrap();
}

impl FromStr for RoundOne {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        REGEX.captures(s).ok_or(io::Error::new(ErrorKind::Other, "failed to parse line")).and_then(|cap|
            Ok(RoundOne {
                opponent: cap[1].parse()?,
                suggested: cap[2].parse()?,
            })
        )
    }
}

impl FromStr for RoundTwo {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        REGEX.captures(s).ok_or(io::Error::new(ErrorKind::Other, "failed to parse line")).and_then(|cap|
            Ok(RoundTwo {
                opponent: cap[1].parse()?,
                suggested: cap[2].parse()?,
            })
        )
    }
}

impl FromStr for Choice {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes()[0] {
            b'A' | b'X' => Ok(Choice::Rock),
            b'B' | b'Y' => Ok(Choice::Paper),
            b'C' | b'Z' => Ok(Choice::Scissors),
            _ => Err(std::io::Error::new(ErrorKind::InvalidInput, "Failed to parse choice {}"))
        }
    }
}
impl FromStr for Outcome {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes()[0] {
            b'X' => Ok(Outcome::OpponentWin),
            b'Y' => Ok(Outcome::Draw),
            b'Z' => Ok(Outcome::YouWin),
            _ => Err(io::Error::new(ErrorKind::InvalidInput, "could not parse choice"))
        }
    }
}

fn one() -> io::Result<u32> {
    let file = File::open("./two/input.txt")?;
    let reader = BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {
        let line = line?;
        let round: RoundOne = line.parse()?;
        let outcome = determine_outcome(&round);
        total_score += outcome_score(&outcome);
        total_score += choice_score(&round.suggested);
    }

    Ok(total_score)
}

fn two() -> io::Result<u32> {
    let file = File::open("./two/input.txt")?;
    let reader = BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {
        let line = line?;
        let round: RoundTwo = line.parse()?;
        let choice = determine_choice(&round);
        total_score += outcome_score(&round.suggested);
        total_score += choice_score(&choice);
    }

    Ok(total_score)
}

fn main() -> io::Result<()> {
    let one = one()?;
    let two = two()?;
    println!("one = {}\ntwo = {}", one, two);
    Ok(())
}