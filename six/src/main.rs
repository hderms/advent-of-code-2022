use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> Result<(), Box<dyn Error>>{
    let mut file = File::open("./six/input.txt")?;
    let mut string = String::new();
     file.read_to_string(&mut string)?;
    let vec: Vec<char> = string.chars().collect();
    let sol_one = find_distinct(&vec, 4);
    let sol_two = find_distinct(&vec, 14);
    println!("solution 1: {:?}", sol_one);
    println!("solution 2: {:?}", sol_two);



    Ok(())
}

fn find_distinct(vec: &Vec<char>, count: usize) -> Option<usize> {
    for (i, chunk) in vec.windows(count).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(chunk);
        if set.len() == count {
            return Some(count + i);
        }
    }
    None
}
