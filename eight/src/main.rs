use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::slice::Iter;

type Tree = u64;

type Point = (usize, usize);

#[derive(Debug)]
struct Grid {
    entries: Vec<Vec<Tree>>,
    width: usize,
    height: usize,
}

struct Column<'a> {
    column_index: usize,
    row_index: usize,
    grid: &'a Grid,
}

impl<'a> Iterator for Column<'a> {
    type Item = (Point, Tree);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index >= self.grid.height {
            return None;
        }
        let element = self.grid.entries[self.row_index][self.column_index];
        let return_value = Some(((self.row_index, self.column_index), element));
        self.row_index += 1;
        return_value
    }
}

struct Row<'a> {
    column_index: usize,
    row_index: usize,
    grid: &'a Grid,
}

impl<'a> Iterator for Row<'a> {
    type Item = (Point, Tree);

    fn next(&mut self) -> Option<Self::Item> {
        if self.column_index >= self.grid.width {
            return None;
        }
        let element = self.grid.entries[self.row_index][self.column_index];
        let return_value = Some(((self.row_index, self.column_index), element));
        self.column_index += 1;
        return_value
    }
}

impl<'a> Grid {
    fn row(&self, row_index: usize) -> impl Iterator<Item=(Point, Tree)> + '_ {
        let row_iterator = Row {
            column_index: 0,
            grid: self,
            row_index,
        };
        row_iterator.into_iter()
    }

    fn column(&self, column_index: usize) -> impl Iterator<Item=(Point, Tree)> + '_ {
        let column_iterator = Column {
            column_index,
            grid: self,
            row_index: 0,
        };
        column_iterator.into_iter()
    }
}

fn count_visible(visibility: &mut HashSet<(usize, usize)>, entries: Vec<(Point, Tree)>) {
    println!("segment is : {:?}", entries);
    let mut max_seen: i64 = -1;
    for (point, entry) in entries.iter() {
        if (*entry as i64) > max_seen {
            println!("entry: {:?} greater than max_seen: {:?} at point: {:?}", entry, max_seen, point);
            visibility.insert(*point);
            max_seen = (*entry as i64);
        }
    }
    max_seen = -1;

    for (point, entry) in entries.iter().rev() {
        if (*entry as i64) > max_seen {
            println!("entry: {:?} greater than max_seen: {:?} at point: {:?} (backwards)", entry, max_seen, point);
            visibility.insert(*point);
            max_seen = (*entry as i64);
        }
    }
}

fn scenic_score( scores: &mut Vec<Vec<usize>>, entries: Vec<(Point, Tree)>)  {
    let mut sightlines: Vec<(u64, usize)> = Vec::new();
    for (point, entry) in entries.iter() {
        let mut new_count = 1;
        let (i, j) = point;

        let mut individual_score = 0;
        println!("sightlines: {:?}", sightlines);
        loop {
            if let Some((previous_height, count)) = sightlines.pop() {
                if *entry > previous_height {

                    individual_score += count;
                    new_count += count;
                } else {
                    individual_score += 1;
                    sightlines.push((previous_height, count));
                    sightlines.push((*entry, new_count));
                    break
                }
            } else {
                sightlines.push((*entry, new_count));
                break
            }
        }

        println!("individual score: {}", individual_score);
        scores[*i][*j] *= individual_score;
    }
    let mut sightlines: Vec<(u64, usize)> = Vec::new();

    println!("reverse: ");
    for (point, entry) in entries.iter().rev() {
        let mut new_count = 1;
        let (i, j) = point;

        let mut individual_score = 0;
        println!("sightlines: {:?}", sightlines);
        loop {
            if let Some((previous_height, count)) = sightlines.pop() {
                if *entry > previous_height {

                    individual_score += count;
                    new_count += count;
                } else {
                    individual_score += 1;
                    sightlines.push((previous_height, count));
                    sightlines.push((*entry, new_count));
                    break
                }
            } else {
                sightlines.push((*entry, new_count));
                break
            }
        }

        println!("individual score: {}", individual_score);
        scores[*i][*j] *= individual_score;
    }


}


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./eight/input.txt")?;
    let reader = BufReader::new(file);
    let mut rows = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::new();
        for c in line.chars() {
            let num: u64 = c.to_digit(10).expect("failed to parse char as digit") as u64;
            row.push(num)
        }
        rows.push(row);
    }
    let height = rows.len();
    let width = rows.first().expect("must have at least one row in data format").len();
    let grid = Grid {
        entries: rows,
        height,
        width,
    };
    println!("grid: {:?}", grid);
    let mut visibility: HashSet<(usize, usize)> = HashSet::new();
    let mut scores = Vec::new();
    for i in 0..grid.height {
        if i == 0  || i == grid.height - 1{
            scores.push(vec![0; grid.width])
        } else {
            let mut row = vec![1; grid.width];
            row[0] = 0;
            row[grid.width - 1] = 0;
            scores.push(row)
        }
    }
    println!("scores: {:?}", scores);
    println!("rows:");
    for row_index in 0..grid.height {
        println!("row: {}", row_index);
        count_visible(&mut visibility, grid.row(row_index).collect());
        scenic_score(&mut scores, grid.row(row_index).collect());
    }

    println!("columns:");
    for col_index in 0..grid.width {
        println!("column: {}", col_index);
        count_visible(&mut visibility, grid.column(col_index).collect());
        scenic_score(&mut scores, grid.column(col_index).collect());
    }
    let max_score = scores.iter().map(|v| v.iter().max()).max();
    println!("sum is {}", visibility.len());
    println!("scores: {:?}", scores);
    println!("max score is: {:?}", max_score  );
    Ok(())
}
