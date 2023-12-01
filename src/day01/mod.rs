#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn task1() {
        // let path = "src/day01/sample.txt";
        let path = "src/day01/input.txt";
        let content = fs::read_to_string(path).unwrap();
        // println!("[[FILE]]\n{content}");

        let result: u32 = content
            .lines()
            .map(|line| {
                let first = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                let last = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .last()
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                first * 10 + last
            })
            .sum();

        println!("Result: {result}");
    }

    #[test]
    fn task2() {
        // let path = "src/day01/sample2.txt";
        let path = "src/day01/input.txt";
        let content = fs::read_to_string(path).unwrap();

        let digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let next_digit = |line: &str| {
            // Forwards search
            line.chars().enumerate().find_map(|(i, c)| {
                if c.is_numeric() {
                    c.to_digit(10)
                } else {
                    let text = &line[i..];
                    // println!("-> {text}");
                    digits
                        .iter()
                        .enumerate()
                        .find_map(|(idx, num)| text.starts_with(num).then_some((idx + 1) as u32))
                }
            })
        };
        let last_digit = |line: &str| {
            // Reverse search
            line.chars().rev().enumerate().find_map(|(i, c)| {
                if c.is_numeric() {
                    c.to_digit(10)
                } else {
                    let i = line.len() - i;
                    let text = &line[0..i];
                    // println!("<- {text}");
                    digits
                        .iter()
                        .enumerate()
                        .find_map(|(idx, num)| text.ends_with(num).then_some((idx + 1) as u32))
                }
            })
        };

        let result: u32 = content
            .lines()
            .map(|line| {
                // println!("# {line}");

                let first = next_digit(line).unwrap();
                let last = last_digit(line).unwrap();
                let value = first * 10 + last;

                // println!("! {value}");
                return value;
            })
            .sum();

        println!("Result: {result}");
    }
}
