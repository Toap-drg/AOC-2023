#![allow(unused)]
#![cfg(test)]
use std::fs;

#[derive(Debug, Default, Clone, Copy)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Default)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

type Res<T> = Result<T, String>;

impl CubeSet {
    fn parse(text: &str) -> Res<Self> {
        let mut set = CubeSet::default();

        for cube in text.split(',') {
            let (count, color) = cube
                .trim()
                .split_once(' ')
                .ok_or_else(|| "Unable to split color and count".to_string())?;
            let count = count.parse::<u32>().map_err(|e| format!("{e:?}"))?;
            match color {
                "red" => {
                    set.red = count;
                }
                "green" => {
                    set.green = count;
                }
                "blue" => {
                    set.blue = count;
                }
                _ => {
                    return Err(format!("Invalid color: '{color}'"));
                }
            }
        }

        Ok(set)
    }

    fn bigger_than(&self, other: &Self) -> bool {
        self.red > other.red || self.blue > other.blue || self.green > other.green
    }

    fn max(&self, other: &Self) -> Self {
        let f = std::cmp::max;
        Self {
            red: f(self.red, other.red),
            green: f(self.green, other.green),
            blue: f(self.blue, other.blue),
        }
    }
}

impl Game {
    fn parse(line: &str) -> Res<Self> {
        let (name, sets) = line
            .split_once(':')
            .ok_or_else(|| "Unable to split indentifier from cube sets".to_string())?;
        let id = name
            .split_whitespace()
            .last()
            .ok_or_else(|| "Unable to split id".to_string())?
            .parse::<u32>()
            .map_err(|e| format!("{e:?}"))?;
        let sets = sets.split(';').map(CubeSet::parse).collect::<Res<_>>()?;
        Ok(Game { id, sets })
    }
}

fn parse(file: &str) -> Result<Vec<Game>, String> {
    fs::read_to_string(file)
        .map_err(|e| format!("{e:?}"))?
        .lines()
        .map(Game::parse)
        .collect()
}

#[test]
fn task1() {
    // let path = "src/day02/sample.txt";
    let path = "src/day02/input.txt";
    let data = parse(path).unwrap();
    // println!("[[DATA]] {data:#?}");

    let max = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result: u32 = data
        .iter()
        .filter(|game| game.sets.iter().find(|set| set.bigger_than(&max)).is_none())
        .map(|game| game.id)
        .sum();

    println!("Result: {result}");
}

#[test]
fn task2() {
    // let path = "src/day02/sample.txt";
    let path = "src/day02/input.txt";
    let data = parse(path).unwrap();
    // println!("[[DATA]] {data:#?}");

    let result: u32 = data
        .iter()
        .map(|game| {
            let mut iter = game.sets.iter();
            let init = iter.next().unwrap().clone();
            let set = iter.fold(init, |a, b| a.max(b));
            let power = set.red * set.green * set.blue;
            // println!("[{}]: {:?} => {}", game.id, set, power);
            return power;
        })
        .sum();

    println!("Result: {result}");
}
