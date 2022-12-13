use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::slice::Iter;

type Tree = u64;
#[derive(Debug)]
struct Grid {
    entries: Vec<Vec<Tree>>,
    width: usize,
    height: usize
}

struct Column<'a> {
    column_index: usize,
    row_index: usize,
    grid: &'a Grid
}

impl<'a> Iterator for Column<'a> {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index >= self.grid.height {
            return None
        }
        let element = self.grid.entries[self.row_index][self.column_index];
        self.row_index += 1;
        Some(element)
    }
}

struct Row<'a> {
    column_index: usize,
    row_index: usize,
    grid: &'a Grid
}

impl<'a> Iterator for Row<'a> {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column_index >= self.grid.width {
            return None
        }
        let element = self.grid.entries[self.row_index][self.column_index];
        self.column_index += 1;
        Some(element)
    }
}
impl<'a> Grid {
    fn row(&self, row_index: usize) -> impl Iterator<Item = Tree> + '_ {
        let row_iterator = Row{
            column_index: 0,
            grid: self,
            row_index
        };
        row_iterator.into_iter()
    }

    fn column(&self, column_index: usize) -> impl Iterator< Item=Tree> + '_ {
        let column_iterator = Column{
            column_index,
            grid: self,
            row_index: 0
        };
        column_iterator.into_iter()
    }
}
fn count_visible(entries: Vec<Tree>) -> usize {
    println!("segment is : {:?}", entries);
    let mut count = 0;
    let mut max_seen: i64 = -1;
    for entry in entries.iter() {
        if (*entry as i64) > max_seen {
            count += 1;
        }
        max_seen = (*entry as i64);
    }
    max_seen = -1;

    for entry in entries.iter().rev() {
        if (*entry as i64) > max_seen {
            count += 1;
        }
        max_seen = (*entry as i64);
    }
    println!("found count: {}", count);

    count
}


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./eight/input.txt")?;
    let reader = BufReader::new(file);
    let mut rows = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let num: u64 = c.to_digit(10).expect("failed to parse char as digit") as u64;
            row.push(num)
        }
        rows.push(row);
    }
    let height = rows.len();
    let width = rows.first().expect("must have at least one row in data format").len();
    let grid = Grid{
        entries: rows,
        height,
        width
    };
    println!("grid: {:?}", grid);
    let mut sum = 0;
    for row_index in 0..grid.height {
        sum += count_visible(grid.row(row_index).collect());
    }

    for col_index in 0..grid.width {
        sum += count_visible(grid.column(col_index).collect());
    }
    println!("sum is {}", sum);
    Ok(())
}
