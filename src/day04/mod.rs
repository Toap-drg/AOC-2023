#![cfg(test)]
use std::fs;

#[derive(Debug)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    guesses: Vec<u32>,
}

fn parse_numbers(text: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    text.split_whitespace()
        .map(|num| num.parse::<u32>())
        .collect()
}

fn parse(path: &str) -> Vec<Card> {
    fs::read_to_string(path)
        .expect("Unable to open file")
        .lines()
        .map(|line| -> Card {
            let (card, numbers) = line.split_once(':').expect("Unable to split card id");
            let id = card
                .trim()
                .split_whitespace()
                .last()
                .expect("No card id")
                .parse::<u32>()
                .expect("Card id is not a number");

            let (numbers, guesses) = numbers.split_once('|').expect("Unable to split numbers");

            Card {
                id,
                numbers: parse_numbers(numbers).expect("Unable to parse numbers"),
                guesses: parse_numbers(guesses).expect("Unable to parse guesses"),
            }
        })
        .collect()
}

#[test]
fn task1() {
    // let path = "src/day04/sample.txt";
    let path = "src/day04/input.txt";
    let data = parse(path);
    // println!("[[DATA]] {data:#?}");
    let result: i32 = data
        .iter()
        .map(|card| {
            let count = card
                .numbers
                .iter()
                .filter(|n| card.guesses.contains(n))
                .count();

            // println!("[{}] -> {}", card.id, count);
            1 << count >> 1
        })
        .sum();
    println!("Result: {result}");
}

#[test]
fn task2() {
    // let path = "src/day04/sample.txt";
    let path = "src/day04/input.txt";
    let data = parse(path);
    // println!("[[DATA]] {data:#?}");

    let mut computed: Vec<usize> = Vec::with_capacity(data.len());

    let result: usize = data
        .iter()
        .rev()
        .map(|card| {
            let count = card
                .numbers
                .iter()
                .filter(|n| card.guesses.contains(n))
                .count();

            let rpos = data.len() - card.id as usize;
            let aggregate: usize = (0..count).map(|i| computed[rpos - i - 1]).sum();
            let total = count + aggregate;
            computed.push(total);
            return total + 1;
        })
        .sum();

    println!("Result: {result}");
}
