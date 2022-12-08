use std::cmp::max;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::cmp::Reverse;

fn main() -> io::Result<()> {
    println!("max elf is {}", solution_one()?);
    println!("sum of top three elf is {}", solution_two()?);
    Ok(())
}


fn load_file() -> io::Result<BufReader<File>> {
    let file = File::open("./one/input.txt")?;
    Ok(BufReader::new(file))
}
fn solution_one() -> io::Result<u32> {
    let reader = load_file()?;
    let mut current_elf: u32 = 0;
    let mut max_elf: u32 = 0;
    for line in reader.lines() {
        let line: String = line?;
        if line.is_empty() {
            //elves.push(current_elf);
            max_elf = max(current_elf, max_elf);
            current_elf = 0;
        } else {
            let number: u32 = line.parse().unwrap();
            current_elf += number;
        }
    }
    Ok(max_elf)
}

fn solution_two() -> io::Result<u32> {
    let reader = load_file()?;
    let mut elves = BinaryHeap::with_capacity(3);
    let mut current_elf: u32 = 0;
    for line in reader.lines() {
        let line: String = line?;
        if line.is_empty() {
            elves.push(Reverse(current_elf));
            while elves.len() > 3 {
                elves.pop();
            }
            current_elf = 0;
        } else {
            let number: u32 = line.parse().unwrap();
            current_elf += number;
        }
    }
    Ok(elves.iter().map(|el| el.0).sum())
}
