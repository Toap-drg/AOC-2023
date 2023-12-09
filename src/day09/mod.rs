#![cfg(test)]

use crate::open_first;

type Number = isize;
struct Extrapolate;

impl Extrapolate {
    fn some(seq: &Vec<Number>) -> bool {
        seq.windows(2).find(|s| s[1] != s[0]).is_some()
    }

    fn next(seq: &Vec<Number>) -> Vec<Number> {
        seq.windows(2).map(|s| s[1] - s[0]).collect()
    }

    fn extend(set: &mut Vec<Vec<Number>>) -> Number {
        set.iter_mut().rfold(0, |diff, seq| -> Number {
            let next = diff + seq.last().expect("missing last element");
            seq.push(next);
            return next;
        })
    }

    fn reverse(set: &mut Vec<Vec<Number>>) -> Number {
        set.iter_mut().rfold(0, |diff, seq| -> Number {
            let prev = seq.first().expect("missing last element") - diff;
            seq.insert(0, prev);
            return prev;
        })
    }
}

#[test]
fn task1() {
    let text = open_first(&[
        "src/day09/input.txt",
        "src/day09/sample.txt", //
    ])
    .unwrap();

    let result: Number = text
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|val| val.parse::<Number>().expect("not a number"))
                .collect::<Vec<_>>();

            let mut sequences = vec![sequence];
            while Extrapolate::some(sequences.last().unwrap()) {
                sequences.push(Extrapolate::next(sequences.last().unwrap()));
            }

            let result = Extrapolate::extend(&mut sequences);
            // println!("{:?} -> {}", sequences, result);

            result
        })
        .sum();

    println!("Result: {}", result);
}

#[test]
fn task2() {
    let text = open_first(&[
        "src/day09/input.txt",
        "src/day09/sample.txt", //
    ])
    .unwrap();

    let result: Number = text
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|val| val.parse::<Number>().expect("not a number"))
                .collect::<Vec<_>>();

            let mut sequences = vec![sequence];
            while Extrapolate::some(sequences.last().unwrap()) {
                sequences.push(Extrapolate::next(sequences.last().unwrap()));
            }

            let result = Extrapolate::reverse(&mut sequences);
            // println!("{:?} -> {}", sequences, result);

            result
        })
        .sum();

    println!("Result: {}", result);
}
