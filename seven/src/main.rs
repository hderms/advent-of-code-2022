use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use slab::Slab;

const CD_REGEX_STR: &str = r"\$ cd ([\w/\.]+)";
const LS_REGEX_STR: &str = r"\$ ls";
const LS_FILE_SIZE_OUTPUT: &str = r"(\d+) (\w+)";
const LS_CHILD_DIR_OUTPUT: &str = r"dir (\w+)";
lazy_static! {
    static ref cd_regex: Regex = Regex::new(CD_REGEX_STR).unwrap();
}
lazy_static! {
    static ref ls_regex: Regex = Regex::new(LS_REGEX_STR).unwrap();
}
lazy_static! {
    static ref ls_file_size_output_regex: Regex = Regex::new(LS_FILE_SIZE_OUTPUT).unwrap();
}
lazy_static! {
    static ref ls_child_dir_output_regex: Regex = Regex::new(LS_CHILD_DIR_OUTPUT).unwrap();
}
#[derive(Clone, Debug)]
struct FileSize {
    name: String,
    size: u64,
}

#[derive(Clone, Debug)]
struct ChildDir {
    name: String,
}

impl FromStr for FileSize {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = ls_file_size_output_regex.captures(s) {
            Ok(
                FileSize {
                    name: captures[2].to_string(),
                    size: captures[1].parse().unwrap(),
                }
            )
        } else {
            Err(io::Error::new(ErrorKind::InvalidData, "invalid filesize"))
        }
    }
}

impl FromStr for ChildDir {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = ls_child_dir_output_regex.captures(s) {
            Ok(
                ChildDir {
                    name: captures[1].to_string(),
                }
            )
        } else {
            Err(io::Error::new(ErrorKind::InvalidData, "invalid child dir"))
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Cd { dir: String },
    Ls {
        output: Vec<FileSize>,
        children: Vec<ChildDir>,
    },
}

impl FromStr for Instruction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cd) = cd_regex.captures(s) {
            Ok(
                Instruction::Cd { dir: cd[1].to_string() }
            )
        } else if ls_regex.is_match(s) {
            Ok(
                Instruction::Ls {
                    output: Vec::new(),
                    children: Vec::new(),
                }
            )
        } else {
            Err(io::Error::new(ErrorKind::InvalidData, "invalid instruction"))
        }
    }
}

struct DirectoryEntry {
    children: HashMap<String, usize>,
    parent: Option<usize>,
    size: u64,
}

impl DirectoryEntry {
    fn new(parent: Option<usize>) -> DirectoryEntry {
        let children = HashMap::new();
        let size = 0;
        DirectoryEntry {
            children,
            size,
            parent,
        }
    }
}

struct FileEntry {
    size: u64,
}

enum FileSystemEntry {
    DirectoryEntry(DirectoryEntry),
    FileEntry(FileEntry),
}

struct FileSystem {
    entries: Slab<FileSystemEntry>,
    index: HashMap<Vec<String>, usize>,
    root: usize,
}

const FILESYSTEM_INITIAL_CAPACITY: usize = 1024;

type Path = Vec<String>;

impl FileSystem {
    fn new(root: usize) -> FileSystem {
        let entries = Slab::with_capacity(FILESYSTEM_INITIAL_CAPACITY);
        let index = HashMap::with_capacity(FILESYSTEM_INITIAL_CAPACITY);
        FileSystem {
            root,
            entries,
            index,
        }
    }
    fn root(&self) -> Option<&FileSystemEntry> {
        self.entries.get(self.root)
    }
    fn get(&self, path: Path) -> Option<(usize, &FileSystemEntry)> {
        let result = self.index.get(&path)?;
        let entry = self.entries.get(*result)?;

        Some((*result, entry))
    }
    fn insert_dir(&mut self, path: Path, parent: Option<usize>) -> usize {
        let new_entry = FileSystemEntry::DirectoryEntry(DirectoryEntry::new(parent));
        let index = self.entries.insert(new_entry);
        self.index.insert(path, index);
        index
    }
}

struct FileSystemCursor {
    stack: Vec<String>,
    root: usize,
    parent: Vec<usize>,
}

impl FileSystemCursor {
    fn new(root: usize) -> FileSystemCursor {
        FileSystemCursor {
            stack: Vec::with_capacity(16),
            root,
            parent: Vec::new()
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.parent.pop();
        ()
    }
    fn push(&mut self, entry: String, parent: usize) {
        self.stack.push(entry);
        self.parent.push(parent);
    }
    fn root(&mut self) {
        self.stack.clear();
        self.parent.clear();
    }

    fn path(&self) -> Vec<String> {
        let mut path = self.stack.clone();
        path
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./seven/input.txt")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<Result<String, io::Error>> = reader.lines().collect();
    lines.reverse();
    let instructions = parse_instructions(lines)?;
    let file_system: FileSystem = interpret_instructions(instructions);

    println!("instructions: {:?}", instructions);
    todo!()
}

fn interpret_instructions(instructions: Vec<Instruction>) -> FileSystem {
    let root = 0;
    let mut filesystem = FileSystem::new(root);
    let mut cursor = FileSystemCursor::new(root);
    for instruction in instructions {
        match instruction {
            Instruction::Cd { dir } if dir.eq("./") => {
                cursor.pop()
            }
            Instruction::Cd { dir } if dir.eq("/") => {
                cursor.root()
            }

            Instruction::Cd { dir } => {
                let last = cursor.parent.last();
                cursor.push(dir, last);
                if let Some((index, current_entry)) = filesystem.get(cursor.path()) {} else {
                    filesystem.insert_dir(cursor.path(), cursor.parent.last());
                }
            }
            Instruction::Ls { output } => {}
        }
    }
    todo!()
}

fn parse_instructions(lines: Vec<Result<String, io::Error>>) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let mut files_buffer: Vec<FileSize> = Vec::with_capacity(64);
    let mut directories_buffer: Vec<ChildDir> = Vec::with_capacity(64);
    let mut instructions: Vec<Instruction> = Vec::with_capacity(128);
    for line in lines {
        let line = line?;
        if line.starts_with("$") {
            let instruction: Instruction = line.parse()?;
            let instruction = match instruction {
                Instruction::Cd { dir } => {
                    Instruction::Cd {
                        dir: dir.clone()
                    }
                }
                Instruction::Ls { .. } => {
                    let result = Instruction::Ls {
                        output: files_buffer.clone(),
                        children: directories_buffer.clone(),
                    };
                    files_buffer.clear();
                    directories_buffer.clear();
                    result
                }
            };
            println!("instruction: {:?}", instruction);
            instructions.push(instruction.clone())
        } else if line.starts_with(|c: char| c.is_ascii_digit()) {
            let file_size: FileSize = line.parse()?;
            files_buffer.push(file_size);
        } else if line.starts_with("dir") {
            let child_dir: ChildDir = line.parse()?;
            directories_buffer.push(child_dir);
        }
    }
    Ok(instructions)
}
