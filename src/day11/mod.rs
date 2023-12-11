#![cfg(test)]

use crate::open_first;

fn order(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

#[test]
fn task1() {
    let text = open_first(&[
        "src/day11/input.txt",  //
        "src/day11/sample.txt", //
    ])
    .unwrap();

    // println!("{}", text);

    let stars = text
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, char)| (char == '#').then(|| (row, col)))
        })
        .collect::<Vec<_>>();

    // println!("{:?}", stars);

    let rows = stars.iter().map(|(row, _)| *row).max().unwrap() + 1;
    let cols = stars.iter().map(|(_, col)| *col).max().unwrap() + 1;

    let mut rows = vec![true; rows];
    let mut cols = vec![true; cols];

    for (row, col) in &stars {
        rows[*row] = false;
        cols[*col] = false;
    }

    // println!("rows: {:?}", rows);
    // println!("cols: {:?}", cols);

    let result: usize = (1..stars.len())
        .map(|idx| -> usize {
            let (a_row, a_col) = stars[idx - 1];
            stars[idx..]
                .iter()
                .map(|&(b_row, b_col)| {
                    let (l_row, h_row) = order(a_row, b_row);
                    let rows = rows[l_row..h_row].iter().filter(|&empty| *empty).count();
                    let rows = rows + h_row - l_row;

                    let (l_col, h_col) = order(a_col, b_col);
                    let cols = cols[l_col..h_col].iter().filter(|&empty| *empty).count();
                    let cols = cols + h_col - l_col;

                    return rows + cols;
                })
                .sum()
        })
        .sum();

    println!("Result: {}", result);
}

#[test]
fn task2() {
    let text = open_first(&[
        "src/day11/input.txt",  //
        "src/day11/sample.txt", //
    ])
    .unwrap();

    const UNIVERSE_AGE: usize = 1_000_000;
    const AGE_TERM: usize = UNIVERSE_AGE - 1;

    // println!("{}", text);

    let stars = text
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, char)| (char == '#').then(|| (row, col)))
        })
        .collect::<Vec<_>>();

    // println!("{:?}", stars);

    let rows = stars.iter().map(|(row, _)| *row).max().unwrap() + 1;
    let cols = stars.iter().map(|(_, col)| *col).max().unwrap() + 1;

    let mut rows = vec![true; rows];
    let mut cols = vec![true; cols];

    for (row, col) in &stars {
        rows[*row] = false;
        cols[*col] = false;
    }

    // println!("rows: {:?}", rows);
    // println!("cols: {:?}", cols);

    let result: usize = (1..stars.len())
        .map(|idx| -> usize {
            let (a_row, a_col) = stars[idx - 1];
            stars[idx..]
                .iter()
                .map(|&(b_row, b_col)| {
                    let (l_row, h_row) = order(a_row, b_row);
                    let rows = rows[l_row..h_row].iter().filter(|&empty| *empty).count();
                    let rows = rows * AGE_TERM + h_row - l_row;

                    let (l_col, h_col) = order(a_col, b_col);
                    let cols = cols[l_col..h_col].iter().filter(|&empty| *empty).count();
                    let cols = cols * AGE_TERM + h_col - l_col;

                    return rows + cols;
                })
                .sum()
        })
        .sum();

    println!("Result: {}", result);
}
