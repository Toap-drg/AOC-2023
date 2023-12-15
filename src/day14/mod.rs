#![cfg(test)]

use crate::open_first;

#[derive(Clone, Copy)]
enum Rock {
    Ball,
    Square,
    Empty,
}

impl Rock {
    fn parse(c: char) -> Option<Self> {
        match c {
            'O' => Some(Rock::Ball),
            '#' => Some(Rock::Square),
            '.' => Some(Rock::Empty),
            _ => None,
        }
    }

    fn symbol(&self) -> &str {
        match self {
            Rock::Ball => "O ",
            Rock::Square => "# ",
            Rock::Empty => ". ",
        }
    }
}

fn transpose(grid: &[Rock], cols: usize) -> Vec<Rock> {
    let mut out = Vec::with_capacity(grid.len());
    for col in 0..cols {
        for row in grid.chunks_exact(cols) {
            out.push(row[col])
        }
    }
    return out;
}

#[test]
fn task1() {
    let text = open_first(&[
        "src/day14/input.txt",  //
        "src/day14/sample.txt", //
    ])
    .unwrap();

    // println!("{}", text);

    let rocks = text
        .lines()
        .flat_map(|line| line.chars().map(|c| Rock::parse(c).expect("not a rock")))
        .collect::<Vec<_>>();

    let rows = text.lines().count();
    let cols = rocks.len() / rows;
    assert_eq!(rows * cols, rocks.len());

    println!("rocks:");
    for row in rocks.chunks_exact(cols) {
        let line = row.iter().map(|r| r.symbol()).collect::<String>();
        println!("{}", line);
    }

    let rocks = transpose(&rocks, cols);
    let cols = rows;

    let result: usize = rocks
        .chunks_exact(cols)
        .map(|row| {
            let mut next = row.len();
            let mut sum = 0;
            for (index, rock) in row.iter().enumerate() {
                match rock {
                    Rock::Ball => {
                        sum += next;
                        next -= 1;
                    }
                    Rock::Square => next = row.len() - index - 1,
                    Rock::Empty => (),
                }
            }
            return sum;
        })
        .sum();

    println!("Result: {}", result);
}
