// __ //
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

#[allow(unused)]
pub(crate) fn open_first(paths: &[&str]) -> std::io::Result<String> {
    let Some(path) = paths.iter().find(|path| std::fs::metadata(path).is_ok()) else {
        return Err(std::io::Error::other(format!(
            "No valid path in list: {paths:?}",
        )));
    };
    std::fs::read_to_string(path)
}

fn main() {
    println!("Hello, world!");
}
