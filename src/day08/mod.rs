#![cfg(test)]

use crate::open_first;

#[derive(Debug, Clone, Copy)]
enum Turn {
    L,
    R,
}

impl Turn {
    fn parse(line: &str) -> Vec<Self> {
        line.chars()
            .map(|c| match c {
                'L' => Turn::L,
                'R' => Turn::R,
                typ => panic!("unexpected turn: {typ}"),
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    l: usize,
    r: usize,
}

impl Node {
    fn get_node(&self, turn: Turn) -> usize {
        match turn {
            Turn::L => self.l,
            Turn::R => self.r,
        }
    }
}

#[derive(Debug)]
struct NameNode<'a> {
    n: &'a str,
    l: &'a str,
    r: &'a str,
}

struct NameNodes<'a>(Vec<NameNode<'a>>);

impl<'a> NameNodes<'a> {
    fn index_of(&self, name: &str) -> Option<usize> {
        self.0.iter().position(|node| node.n == name)
    }

    fn iter(&self) -> std::slice::Iter<'_, NameNode<'_>> {
        self.0.iter()
    }

    fn name(&self, idx: usize) -> &str {
        self.0[idx].n
    }

    fn optimized(&self) -> Vec<Node> {
        self.iter()
            .map(|node| Node {
                l: self.index_of(node.l).expect("invalid left label"),
                r: self.index_of(node.r).expect("invalid right label"),
            })
            .collect()
    }

    fn parse(lines: std::str::Lines<'a>) -> Self {
        let nodes = lines
            .map(|line| {
                let (name, lr) = line.split_once("=").expect("name = (left, right)");
                let (l, r) = lr.split_once(',').expect("(left, right)");
                NameNode {
                    n: name.trim(),
                    l: l.trim().strip_prefix('(').expect("(left"),
                    r: r.trim().strip_suffix(')').expect("right)"),
                }
            })
            .collect();
        Self(nodes)
    }
}

#[test]
fn test1() {
    let text = open_first(&[
        "src/day08/input.txt",
        "src/day08/sample2.txt",
        "src/day08/sample1.txt",
    ])
    .unwrap();

    // println!("{}", text);

    let mut lines = text.lines();

    let turns = Turn::parse(lines.next().expect("no turn order"));

    // println!("Turns: {:?}", turns);

    assert_eq!(lines.next(), Some(""), "empty line before nodes");

    let named = NameNodes::parse(lines);
    // println!("Named: {:#?}", named);

    let start = named.index_of("AAA").expect("no start node");
    let target = named.index_of("ZZZ").expect("no target node");

    let indexed = named.optimized();

    // println!("Indexed: {:?}", indexed);

    let mut current = start;
    let mut count = 0;
    while current != target {
        count += 1;
        for &turn in &turns {
            current = indexed[current].get_node(turn);
        }
    }

    let result = count * turns.len();
    println!("Result: {}", result);
}

#[test]
fn test2() {
    let text = open_first(&[
        "src/day08/input.txt", //
        "src/day08/sample3.txt",
    ])
    .unwrap();

    // println!("{}", text);

    let mut lines = text.lines();

    let turns = Turn::parse(lines.next().expect("no turn order"));

    // println!("Turns: {:?}", turns);

    assert_eq!(lines.next(), Some(""), "empty line before nodes");

    let named = NameNodes::parse(lines);
    // println!("Named: {:#?}", named);

    let indexed = named.optimized();

    // println!("Indexed: {:?}", indexed);

    #[derive(Debug)]
    struct Cycle {
        suffix: usize,
        length: usize,
    }

    impl Cycle {
        fn hit(&self, target: usize) -> bool {
            (target - self.suffix) % self.length == 0
        }
    }

    let mut cycles = named
        .iter()
        .enumerate()
        .filter_map(|(idx, node)| node.n.ends_with('A').then_some(idx))
        .map(|mut current| {
            let mut visited: Vec<usize> = Vec::<usize>::new();
            while !visited.contains(&current) {
                visited.push(current);
                for &turn in &turns {
                    current = indexed[current].get_node(turn);
                }
            }

            let suffix = visited.iter().position(|idx| *idx == current).unwrap();
            let length = visited.len() - suffix;
            let target = visited
                .iter()
                .enumerate()
                .filter_map(|(idx, node)| named.name(*node).ends_with('Z').then_some(idx))
                .collect::<Vec<_>>();
            assert_eq!(target.len(), 1, "expected a single endpoint");

            // Make the first and only target be
            // at the end of the suffix,
            // so that a cycle always has the target at [0]
            let suffix = target[0] + suffix - 1;

            return Cycle { suffix, length };
        })
        .collect::<Vec<_>>();

    cycles.sort_by_key(|cycle| cycle.length);
    let cycles = cycles;

    println!("Cycles: {:#?}", cycles);

    // Should have used lcm (least-common-multiple) to reduce the cycle lengths (?)
    // as that's what this is effectively doing after observing suffix and length being equal
    let long = cycles.iter().max_by_key(|cycle| cycle.length).unwrap();
    let mut count = long.suffix;
    while let Some(_) = cycles.iter().find(|cycle| cycle.suffix > count) {
        count += long.length;
    }
    while let Some(_) = cycles.iter().find(|cycle| !cycle.hit(count)) {
        count += long.length;
    }
    // 10151663816849
    //    36648605837 lcm is too small ?

    let result = count * turns.len();
    println!("Result: {}", result);
}
