#![cfg(test)]

use crate::open_first;

fn distance_of_u32(duration: u32, charge: u32) -> u32 {
    let remainder = duration - charge;
    return remainder * charge;
}

fn distance_of_usize(duration: usize, charge: usize) -> usize {
    let remainder = duration - charge;
    return remainder * charge;
}

#[test]
fn task1() {
    let data = open_first(&[
        "src/day06/input.txt",
        "src/day06/sample.txt",
        //
    ])
    .unwrap();

    let numbers = |str: &str| -> Vec<u32> {
        str.split_once(':')
            .expect("Missing separator")
            .1
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Not a number"))
            .collect()
    };

    let mut iter = data.lines();
    let durations = numbers(iter.next().expect("Missing durations"));
    let distances = numbers(iter.next().expect("Missing distances"));

    let result: u32 = Iterator::zip(distances.iter(), durations.iter())
        .map(|(&distance, &duration)| {
            //
            println!("duration: {:3}", duration);
            println!("distance: {:3}", distance);

            let middle = duration >> 1;
            assert!(
                distance_of_u32(duration, middle) > distance,
                "Middle charge is too short, case is impossible"
            );

            let min = (0..middle)
                .rev()
                .find(|charge| distance_of_u32(duration, *charge) <= distance)
                .unwrap();
            let max = (middle..duration)
                .find(|charge| distance_of_u32(duration, *charge) <= distance)
                .unwrap();

            let result = max - min - 1;

            println!("middle: {middle} result: {result}");

            return result;
        })
        .product();

    println!("Result: {}", result);
}

#[test]
fn task2() {
    let data = open_first(&[
        "src/day06/input.txt",
        "src/day06/sample.txt",
        //
    ])
    .unwrap();

    let number = |str: &str| -> usize {
        str.split_once(':')
            .expect("Missing separator")
            .1
            .trim()
            .split_whitespace()
            .collect::<String>()
            .parse::<usize>()
            .expect("Not a number")
    };

    let mut iter = data.lines();
    let duration = number(iter.next().expect("Missing durations"));
    let distance = number(iter.next().expect("Missing distances"));

    //
    println!("duration: {:3}", duration);
    println!("distance: {:3}", distance);

    let middle = duration >> 1;
    assert!(
        distance_of_usize(duration, middle) > distance,
        "Middle charge is too short, case is impossible"
    );

    let min = (0..middle)
        .rev()
        .find(|charge| distance_of_usize(duration, *charge) <= distance)
        .unwrap();
    let max = (middle..duration)
        .find(|charge| distance_of_usize(duration, *charge) <= distance)
        .unwrap();

    let result = max - min - 1;

    println!("middle: {middle}");

    println!("Result: {}", result);
}
