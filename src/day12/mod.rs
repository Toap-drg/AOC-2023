#![cfg(test)]

use crate::open_first;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "#"),
            Self::Damaged => write!(f, "."),
            Self::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug)]
struct Info {
    states: Vec<State>,
    runs: Vec<u32>,
}

impl Info {
    fn parse(line: &str) -> Self {
        let (states, runs) = line.split_once(|c: char| c.is_whitespace()).unwrap();

        let states = states
            .chars()
            .map(|c| match c {
                '#' => State::Operational,
                '.' => State::Damaged,
                '?' => State::Unknown,
                _ => panic!("unexpected state"),
            })
            .collect();

        let runs = runs
            .split(',')
            .map(|val| val.parse::<u32>().expect("not a number"))
            .collect();

        Self { states, runs }
    }

    fn unfold(&self, times: usize) -> Self {
        let mut states = Vec::with_capacity(self.states.len() * times + times - 1);
        let mut runs = Vec::with_capacity(self.runs.len() * times);
        states.extend(&self.states);
        runs.extend(&self.runs);
        for _ in 1..times {
            states.push(State::Unknown);
            states.extend(&self.states);
            runs.extend(&self.runs);
        }
        Self { states, runs }
    }

    fn possible_arrangments(&self) -> usize {
        fn recursive(states: &[State], runs: &[u32]) -> usize {
            let Some((&run, runs)) = runs.split_first() else {
                if states.contains(&State::Operational) {
                    return 0;
                } else {
                    return 1;
                }
            };
            let run = run as usize;

            let mut pos = 0;
            let mut sum = 0;
            loop {
                let end = pos + run;
                if end > states.len() {
                    // Out of range
                    break;
                }

                // Final Run
                if end == states.len() {
                    // Must be last run
                    if !runs.is_empty() {
                        break;
                    }

                    // Must not be damaged
                    if states[pos..end].contains(&State::Damaged) {
                        break;
                    }

                    // Count the final run
                    sum += 1;
                    break;
                }

                // Find first significant position
                let offset = states[pos..end]
                    .iter()
                    .position(|&state| state != State::Unknown)
                    .unwrap_or(0)
                    + pos;

                match states[offset] {
                    State::Operational => {
                        // Run must be valid
                        if states[offset..end].contains(&State::Damaged) {
                            break;
                        }

                        let must_move = states[end] == State::Operational;
                        let must_stay = states[pos] == State::Operational;
                        match (must_move, must_stay) {
                            (true, true) => {
                                // Contradition
                                break;
                            }
                            (true, false) => {
                                // Must move
                                pos += 1;
                                continue;
                            }
                            (false, true) => {
                                // Must stay
                                sum += recursive(&states[end + 1..], runs);
                                break;
                            }
                            (false, false) => {
                                // No limitation
                                sum += recursive(&states[end + 1..], runs);
                                pos += 1;
                                continue;
                            }
                        }
                    }
                    State::Damaged => {
                        // Must move past damage
                        pos = offset + 1;
                        continue;
                    }
                    State::Unknown => {
                        // Must be a complete run
                        if states[end] != State::Operational {
                            sum += recursive(&states[end + 1..], runs);
                        }

                        pos += 1;
                        continue;
                    }
                }
            }
            return sum;
        }

        recursive(&self.states, &self.runs)
    }

    fn possible_arrangments_dynamic(&self) -> usize {
        // Just inject some dynamic programming to make it faster
        struct Location {
            state: usize,
            run: usize,
            sum: usize,
        }

        type Visited<'a> = &'a mut Vec<Location>;

        fn recursive(states: &[State], runs: &[u32], visited: Visited) -> usize {
            if let Some(prev) = visited
                .iter()
                .find(|loc| loc.state == states.len() && loc.run == runs.len())
            {
                return prev.sum;
            }

            let Some((&run, runs)) = runs.split_first() else {
                if states.contains(&State::Operational) {
                    return 0;
                } else {
                    return 1;
                }
            };
            let run = run as usize;

            let mut pos = 0;
            let mut sum = 0;
            loop {
                let end = pos + run;
                if end > states.len() {
                    // Out of range
                    break;
                }

                // Final Run
                if end == states.len() {
                    // Must be last run
                    if !runs.is_empty() {
                        break;
                    }

                    // Must not be damaged
                    if states[pos..end].contains(&State::Damaged) {
                        break;
                    }

                    // Count the final run
                    sum += 1;
                    break;
                }

                // Find first significant position
                let offset = states[pos..end]
                    .iter()
                    .position(|&state| state != State::Unknown)
                    .unwrap_or(0)
                    + pos;

                match states[offset] {
                    State::Operational => {
                        // Run must be valid
                        if states[offset..end].contains(&State::Damaged) {
                            break;
                        }

                        let must_move = states[end] == State::Operational;
                        let must_stay = states[pos] == State::Operational;
                        match (must_move, must_stay) {
                            (true, true) => {
                                // Contradition
                                break;
                            }
                            (true, false) => {
                                // Must move
                                pos += 1;
                                continue;
                            }
                            (false, true) => {
                                // Must stay
                                sum += recursive(&states[end + 1..], runs, visited);
                                break;
                            }
                            (false, false) => {
                                // No limitation
                                sum += recursive(&states[end + 1..], runs, visited);
                                pos += 1;
                                continue;
                            }
                        }
                    }
                    State::Damaged => {
                        // Must move past damage
                        pos = offset + 1;
                        continue;
                    }
                    State::Unknown => {
                        // Must be a complete run
                        if states[end] != State::Operational {
                            sum += recursive(&states[end + 1..], runs, visited);
                        }

                        pos += 1;
                        continue;
                    }
                }
            }
            visited.push(Location {
                state: states.len(),
                run: runs.len() + 1,
                sum,
            });
            return sum;
        }

        recursive(&self.states, &self.runs, &mut Vec::new())
    }
}

#[test]
fn task1() {
    let text = open_first(&[
        "src/day12/input.txt",  //
        "src/day12/sample.txt", //
    ])
    .unwrap();

    let result: usize = text
        .lines()
        .map(|line| {
            let count = Info::parse(line).possible_arrangments();
            assert_ne!(count, 0, "impossible to arrange");
            return count;
        })
        .sum();

    println!("Result: {}", result);
}

#[test]
fn task2() {
    let text = open_first(&[
        "src/day12/input.txt",  //
        "src/day12/sample.txt", //
    ])
    .unwrap();

    let result: usize = text
        .lines()
        .map(|line| {
            let count = Info::parse(line).unfold(5).possible_arrangments_dynamic();
            assert_ne!(count, 0, "impossible to arrange");
            return count;
        })
        .sum();

    println!("Result: {}", result);
}
