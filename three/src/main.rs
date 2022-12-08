#![feature(iter_array_chunks)]
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
fn priority(choice: &u8) -> u8 {
    if (b'A'..=b'Z').contains(&choice) {
        (choice - b'A') + 27

    } else {
        (choice - b'a') + 1
    }
}

fn one() -> io::Result<()>{
    let file = File::open("./three/input.txt")?;
    let reader = BufReader::new(file);
    let mut all = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let len = line.len();
        let half = len / 2;
        let mut first = HashSet::with_capacity(half);
        let mut second = HashSet::with_capacity(half);
        let bytes = line.as_bytes();
        for i in 0..half {
            first.insert( bytes[i]);
        }

        for i in half..len {
            second.insert( bytes[i]);
        }
        let common: Vec<u8> = first.intersection(&second).map(|el| *el).collect();
        all.extend(common);

    }

    let sum: u32 = all.iter().map(|el| priority(el) as u32).sum();
    println!("1: sum is {}", sum);

    Ok(())
}

fn produce_set(s: &str) -> HashSet<u8> {
    s.as_bytes().iter().map(|el| *el).collect()
}
fn two() -> io::Result<()>{
    let file = File::open("./three/input.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for [one, two, three] in reader.lines().array_chunks() {
        let (one, two, three) = (one?, two?, three?);
        let (one, two, three) = (produce_set(&one), produce_set(&two), produce_set(&three));
        let intersection: HashSet<u8> = one.intersection(&two).map(|el| *el).collect();
        let intersection = intersection.intersection(&three);
        sum += intersection.map(|el| priority(el) as u32).sum::<u32>()
    }

    println!("2: sum is {}", sum);

    Ok(())
}

fn main() -> io::Result<()> {
    one();
    two();
    Ok(())

}