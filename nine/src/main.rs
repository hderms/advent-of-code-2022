#![feature(io_error_other)]

use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use lazy_static::lazy_static;
use anyhow::{Context, Error, Result};
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex = Regex::new(r"([UDLR]) (\d+)").expect("failed to instantiate regex at startup");
}
type Point = (i32, i32);

fn main() -> Result<()> {
    let file = File::open("./nine/input.txt")?;
    let reader = BufReader::new(file);
    let mut rope = Rope::new();
    let mut track_distinct = HashSet::new();
    track_distinct.insert(rope.tail);

    for line in reader.lines() {
        let line = line?;
        let instruction: Instruction = line.parse()?;
        for _ in 0..instruction.count {
            rope = rope.compute_next(&instruction.direction);
            track_distinct.insert(rope.tail);
        }
    }
    let distinct_spaces = track_distinct.len();
    println!("part one: {}", distinct_spaces);
    Ok(())
}

struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
        }
    }
    fn compute_next(self, direction: &Direction) -> Rope {
        let (head_x, head_y) = self.head;
        let new_head: Point = match direction {
            Direction::Up => {
                (head_x, head_y + 1)
            }
            Direction::Down => {
                (head_x, head_y - 1)
            }
            Direction::Left => {
                (head_x - 1, head_y)
            }
            Direction::Right => {
                (head_x + 1, head_y)
            }
        };

        let new_tail = compute_tail(&new_head, &self.tail);

        Rope {
            head: new_head,
            tail: new_tail,
        }
    }
}

fn compute_tail(new_head: &Point, old_tail: &Point) -> Point {
    let ((head_x, head_y), (tail_x, tail_y)) = (new_head, old_tail);

    let x_delta = head_x - tail_x;
    let y_delta = head_y - tail_y;

    let clamped_x_delta = clamp_int_sign(x_delta);
    let clamped_y_delta = clamp_int_sign(y_delta);

    if touching(x_delta, y_delta) {
        *old_tail
    } else {
        (*tail_x + clamped_x_delta, *tail_y + clamped_y_delta)
    }
}

fn touching(x_delta: i32, y_delta: i32) -> bool {
    x_delta.abs() <= 1 && y_delta.abs() <= 1
}

fn clamp_int_sign(delta: i32) -> i32 {
    min(max(-1, delta), 1)
}

struct Instruction {
    direction: Direction,
    count: i32,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let captures = INSTRUCTION_REGEX.captures(string).ok_or(invalid_data("failed to capture regex"))?;
        Ok(Instruction {
            direction: captures[1].parse()?,
            count: captures[2].parse().with_context(|| format!("failed to parse digit: {:?}", &captures[1]))?,
        })
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('U') => Ok(Direction::Up),
            Some('D') => Ok(Direction::Down),
            Some('R') => Ok(Direction::Right),
            Some('L') => Ok(Direction::Left),
            _ => Err(invalid_data("Failed to parse direction"))
        }
    }
}

fn invalid_data(s: &str) -> io::Error {
    io::Error::other(s)
}