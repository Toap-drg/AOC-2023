#![cfg(test)]
use std::{fs, ptr};

#[test]
fn task1() {
    // let path = "src/day03/sample.txt";
    let path = "src/day03/input.txt";
    let data = fs::read_to_string(path).unwrap();
    // println!("[[DATA]] {data:#?}");

    let rows: Vec<&str> = data.lines().collect();
    let r = rows.len() - 1;
    let l = rows.get(0).unwrap_or(&".").len() - 1;

    let has_symbol = |text: &str| text.find(|c: char| c != '.' && !c.is_numeric()).is_some();

    let result: u32 = rows
        .iter()
        .enumerate()
        .map(|(row, line)| -> u32 {
            // println!("[..] '{}'", line);
            let mut sum = 0;
            let mut e = 0;
            while let Some(i) = line[e..]
                .chars()
                .position(|c| c.is_numeric())
                .map(|i| e + i)
            {
                e = line[i..]
                    .chars()
                    .position(|c| !c.is_numeric())
                    .map(|e| i + e)
                    .unwrap_or(line.len());

                // println!("[{}..{}]: '{}'", i, e, &line[i..e]);

                // Column indices of neighborhood
                let ri = if i > 0 { i - 1 } else { i };
                let re = if e < l { e + 1 } else { e };

                // Neighborhood checks
                let r1 = row > 0 && has_symbol(&rows[row - 1][ri..re]);
                let r2 = has_symbol(&line[ri..re]);
                let r3 = row < r && has_symbol(&rows[row + 1][ri..re]);

                // Require one adjecent symbol
                if r1 || r2 || r3 {
                    sum += line[i..e].parse::<u32>().unwrap();
                }
            }
            return sum;
        })
        .sum();

    println!("Result: {result}");
}

#[test]
fn task2() {
    // let path = "src/day03/sample.txt";
    let path = "src/day03/input.txt";
    let data = fs::read_to_string(path).unwrap();
    // println!("[[DATA]] {data:#?}");

    let rows: Vec<&str> = data.lines().collect();
    let r = rows.len();
    let l = rows.get(0).unwrap_or(&".").len();

    let subs = |i, m| if i > m { i - 1 } else { i - 0 };
    let adds = |i, m| if i < m { i + 2 } else { i + 1 };

    let result: u32 = rows
        .iter()
        .enumerate()
        .map(|(row, line)| -> u32 {
            let ri = subs(row, 0);
            let re = adds(row, r);
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| (c == '*').then_some(i))
                .map(|col| -> u32 {
                    let ci = subs(col, 0);
                    let ce = adds(col, l);

                    let mut g1: Option<&str> = None;
                    let mut g2: Option<&str> = None;

                    // square: (ri,ci) -> (re,ce)
                    for row in &rows[ri..re] {
                        // println!("#[{}..{}] {}", ri, re, &row[ci..ce]);
                        for (c, char) in row[ci..ce].char_indices() {
                            if !char.is_numeric() {
                                continue;
                            }
                            let ci = c + ci;
                            let ce = 1 + ci;
                            // println!("HIT: '{}'", &row[ci..ce]);
                            // There is a number here
                            let i = row[..ci]
                                .chars()
                                .rev()
                                .position(|c| !c.is_numeric())
                                .map(|p| ci - p)
                                .unwrap_or(0);
                            let e = row[ce..]
                                .chars()
                                .position(|c| !c.is_numeric())
                                .map(|p| ce + p)
                                .unwrap_or(r);

                            let span = &row[i..e];
                            // println!("[{}..{}] '{}'", i, e, span);

                            // Maybe first gear
                            let Some(that) = g1 else {
                                g1.replace(span);
                                continue;
                            };
                            // Maybe duplicate gear
                            if ptr::eq(that, span) {
                                continue;
                            }

                            // Maybe second gear
                            let Some(that) = g2 else {
                                g2.replace(span);
                                continue;
                            };
                            // Maybe duplicate gear
                            if ptr::eq(that, span) {
                                continue;
                            }

                            // Unexpected third gear
                            return 0;
                        }
                    }

                    // Must have first gear
                    let Some(g1) = g1 else {
                        return 0;
                    };

                    // Must have second gear
                    let Some(g2) = g2 else {
                        return 0;
                    };

                    let g1 = g1.parse::<u32>().unwrap();
                    let g2 = g2.parse::<u32>().unwrap();
                    let ratio = g1 * g2;
                    // println!("* {}", ratio);
                    return ratio;
                })
                .sum()
        })
        .sum();

    println!("Result: {result}");
}
