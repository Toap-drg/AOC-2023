#![cfg(test)]

use crate::open_first;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Ash,
    Rock,
}

impl Cell {
    fn symbol(&self) -> char {
        match self {
            Cell::Ash => '.',
            Cell::Rock => '#',
        }
    }

    fn parse(c: char) -> Option<Self> {
        match c {
            '.' => Some(Cell::Ash),
            '#' => Some(Cell::Rock),
            _ => None,
        }
    }
}

fn has_mirror(line: &[Cell], at: usize) -> bool {
    line[0..at]
        .iter()
        .rev()
        .zip(&line[at..])
        .all(|(f, r)| *f == *r)
}

fn smudge_mirror(line: &[Cell], at: usize) -> usize {
    line[0..at]
        .iter()
        .rev()
        .zip(&line[at..])
        .filter(|(f, r)| *f != *r)
        .count()
}

struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new_from(cols: usize, cells: Vec<Cell>) -> Self {
        let rows = cells.len() / cols;
        assert_eq!(cols * rows, cells.len());
        Grid { rows, cols, cells }
    }

    fn parse_list(text: &str) -> Vec<Grid> {
        let mut lines = text.lines();
        let mut width = 0;
        let mut cells = Vec::new();
        let mut grids = Vec::new();

        while let Some(line) = lines.next() {
            if !line.is_empty() {
                if cells.is_empty() {
                    width = line.len()
                }
                cells.extend(line.chars().map(|c| Cell::parse(c).expect("invalid cell")));
            } else {
                grids.push(Grid::new_from(width, std::mem::take(&mut cells)));
            }
        }
        if !cells.is_empty() {
            grids.push(Grid::new_from(width, cells));
        }
        return grids;
    }

    fn at(&self, col: usize, row: usize) -> Cell {
        self.cells[row * self.cols + col]
    }

    fn transpose(&self) -> Self {
        let mut cells = Vec::with_capacity(self.cells.len());
        for col in 0..self.cols {
            for row in 0..self.rows {
                cells.push(self.at(col, row));
            }
        }
        Self {
            rows: self.cols,
            cols: self.rows,
            cells,
        }
    }

    fn find_row_mirror(&self) -> Option<usize> {
        for pos in 1..self.cols {
            if self
                .cells
                .chunks_exact(self.cols)
                .all(|row| has_mirror(row, pos))
            {
                return Some(pos);
            }
        }
        return None;
    }

    fn find_row_mirror_smudge(&self) -> Option<usize> {
        for pos in 1..self.cols {
            let count = self.cells.chunks_exact(self.cols).fold(0, |count, row| {
                if count < 2 {
                    count + smudge_mirror(row, pos)
                } else {
                    count
                }
            });
            // println!("count[{}]: {}", pos, count);
            if count == 1 {
                return Some(pos);
            }
        }
        return None;
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(self.rows * self.cols + self.rows);
        for row in self.cells.chunks_exact(self.cols) {
            for cell in row {
                out.push(cell.symbol());
                out.push(' ');
            }
            out.push('\n');
        }
        f.write_str(&out)
    }
}

#[test]
fn task1() {
    let text = open_first(&[
        "src/day13/input.txt",  //
        "src/day13/sample.txt", //
    ])
    .unwrap();

    let result: usize = Grid::parse_list(&text)
        .iter()
        .map(|g| {
            if let Some(pos) = g.find_row_mirror() {
                println!("mirror({}):\n{:?}", pos, g);
                return pos;
            }

            let t = g.transpose();
            if let Some(pos) = t.find_row_mirror() {
                println!("mirror({}):\n{:?}", pos, g);
                return pos * 100;
            }

            panic!("found no mirror");
        })
        .sum();

    println!("Result: {}", result);
}

#[test]
fn task2() {
    let text = open_first(&[
        "src/day13/input.txt",  //
        "src/day13/sample.txt", //
    ])
    .unwrap();

    let result: usize = Grid::parse_list(&text)
        .iter()
        .map(|g| {
            if let Some(pos) = g.find_row_mirror_smudge() {
                println!("mirror({}):\n{:?}", pos, g);
                return pos;
            }

            let t = g.transpose();
            if let Some(pos) = t.find_row_mirror_smudge() {
                println!("mirror({}):\n{:?}", pos, g);
                return pos * 100;
            }

            panic!("found no mirror");
        })
        .sum();

    println!("Result: {}", result);
}
