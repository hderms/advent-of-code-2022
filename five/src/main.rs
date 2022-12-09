use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Lines};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

const INSTRUCTION_REGEX_STR: &str = r"^move (\d+) from (\d+) to (\d+)$";
lazy_static! {
    static ref instruction_regex: Regex = Regex::new(INSTRUCTION_REGEX_STR).unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./five/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut crates = build_crates(&mut lines, 9)?;
    let mut crates_copy = crates.clone();
    let instructions = build_instructions(&mut lines)?;
    for instruction in instructions.iter() {
        interpret_instruction(instruction, &mut crates, false)
    }
    let mut result = Vec::with_capacity(9);
    for c in crates.crates.iter_mut() {
        result.push(c.pop().unwrap())
    }
    println!("answer 1: {}", String::from_utf8(result).unwrap());

    for instruction in instructions.iter() {
        interpret_instruction(instruction, &mut crates_copy, true)
    }
    let mut result = Vec::with_capacity(9);
    for c in crates_copy.crates.iter_mut() {
        result.push(c.pop().unwrap())
    }
    println!("answer 2: {}", String::from_utf8(result).unwrap());

    Ok(())
}
#[derive(Clone, Debug)]
struct Crates {
    crates: Vec<Vec<u8>>
}

fn build_crates(reader: &mut Lines<BufReader<File>>, expected_number: usize) -> io::Result<Crates> {
    let mut crates: Vec<Vec<u8>> = Vec::with_capacity(expected_number);
    for i in 0..expected_number {
        crates.push(Vec::with_capacity(32));
    }
    while let Some(next_line) = reader.next() {
        let line = next_line?;
        let bytes = line.as_bytes();
        if bytes.is_empty() || bytes[0].is_ascii_digit() {
            break
        }

        let mut i = 0;
        let mut index = 0;
        while i < bytes.len() {
            let old_index = i;
            while i < bytes.len() && bytes[i] == b' ' {
                i += 1;
            }
            let delta = i - old_index;
            let count = delta / 3;
            index += count;
            if i < bytes.len() && bytes[i] != b'[' {
                break
            } else {
                i += 1; // drop [
                if i >= bytes.len() {
                    break
                }
                let next_num = bytes[i];
                i += 2; //advance past number and closing ]

                crates[index].push(next_num);

                index += 1
            }


        }

    };
    for mut c in crates.iter_mut() {
        c.reverse()
    }

    Ok(Crates{
        crates
    })

}
#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize
}
impl FromStr for Instruction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = instruction_regex.captures(s).ok_or(io::Error::new(ErrorKind::InvalidData, "failed to parse instruction"))?;
        let instruction = Instruction{
            count: captures[1].parse().unwrap(),
            from: captures[2].parse().unwrap(),
            to: captures[3].parse().unwrap(),

        };
        Ok(instruction)
    }
}

fn build_instructions(reader: &mut Lines<BufReader<File>>) -> io::Result<Vec<Instruction>> {
    let mut instructions: Vec<Instruction> = Vec::with_capacity(128);
    while let Some(next_line) = reader.next() {
        let next_line = next_line?;
        let instruction: Instruction = next_line.parse()?;
        instructions.push(instruction);

    }
    Ok(instructions)
}

fn interpret_instruction(instruction: &Instruction, crates: &mut Crates, in_place: bool)  {

    let from = instruction.from - 1;
    let to = instruction.to - 1;
    if in_place {
        let mut temp = Vec::with_capacity(instruction.count);
        for i in 0..instruction.count {
            temp.push( crates.crates[from].pop().unwrap());
        }
        temp.reverse();
        crates.crates[to].extend(temp);

    } else {
        for i in 0..instruction.count {
            let temp = crates.crates[from].pop().unwrap();
            crates.crates[to].push(temp);
        }

    }
}
