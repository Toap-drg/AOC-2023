#![cfg(test)]

use crate::open_first;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Vertical,   // |
    Horisontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Creature,   // S
}

impl Pipe {
    fn parse(c: char) -> Option<Self> {
        Some(match c {
            '|' => Self::Vertical,
            '-' => Self::Horisontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Creature,
            _ => return None,
        })
    }

    fn symbol(&self) -> &'static str {
        match self {
            Self::Vertical => "║ ",
            Self::Horisontal => "══",
            Self::NorthEast => "╚═",
            Self::NorthWest => "╝ ",
            Self::SouthWest => "╗ ",
            Self::SouthEast => "╔═",
            Self::Ground => "  ",
            Self::Creature => "S ",
        }
    }
}

impl std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.symbol())
    }
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

struct PipeGrid {
    width: usize,
    pipes: Vec<Pipe>,
}

impl PipeGrid {
    fn parse(text: &str) -> Option<Self> {
        let width = text.lines().next()?.len();
        let pipes = text
            .lines()
            .flat_map(|line| line.chars())
            .map(Pipe::parse)
            .collect::<Option<Vec<_>>>()?;
        Some(Self { width, pipes })
    }
    fn cols(&self) -> usize {
        self.width
    }
    fn rows(&self) -> usize {
        self.pipes.len() / self.width
    }
    fn at(&self, x: usize, y: usize) -> Pipe {
        self.pipes[x + self.width * y]
    }
    fn creature(&self) -> Option<Pos> {
        self.pipes
            .iter()
            .position(|&pipe| pipe == Pipe::Creature)
            .map(|pos| Pos {
                x: pos % self.width,
                y: pos / self.width,
            })
    }
    fn repr(&self) -> String {
        let rows = self.rows() + 2;
        let cols = self.cols() + 1;
        let mut out = String::with_capacity(rows * cols * 2 + rows);
        out.extend((0..cols).map(|_| "# "));
        for line in self.pipes.chunks_exact(self.width) {
            out += "#\n# ";
            out.extend(line.iter().map(Pipe::symbol));
        }
        out += "#\n";
        out.extend((0..cols).map(|_| "# "));
        out += "#";
        return out;
    }
}

impl std::fmt::Display for PipeGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    const fn all() -> [Self; 4] {
        [Direction::N, Direction::S, Direction::W, Direction::E]
    }
    fn next(self, pipe: Pipe) -> Option<Self> {
        match (self, pipe) {
            // N
            (Self::N, Pipe::Vertical) => Some(Self::N),
            (Self::N, Pipe::Horisontal) => None,
            (Self::N, Pipe::NorthEast) => None,
            (Self::N, Pipe::NorthWest) => None,
            (Self::N, Pipe::SouthWest) => Some(Self::W),
            (Self::N, Pipe::SouthEast) => Some(Self::E),
            // S
            (Self::S, Pipe::Vertical) => Some(Self::S),
            (Self::S, Pipe::Horisontal) => None,
            (Self::S, Pipe::NorthEast) => Some(Self::E),
            (Self::S, Pipe::NorthWest) => Some(Self::W),
            (Self::S, Pipe::SouthWest) => None,
            (Self::S, Pipe::SouthEast) => None,
            // W
            (Self::W, Pipe::Vertical) => None,
            (Self::W, Pipe::Horisontal) => Some(Self::W),
            (Self::W, Pipe::NorthEast) => Some(Self::N),
            (Self::W, Pipe::NorthWest) => None,
            (Self::W, Pipe::SouthWest) => None,
            (Self::W, Pipe::SouthEast) => Some(Self::S),
            // E
            (Self::E, Pipe::Vertical) => None,
            (Self::E, Pipe::Horisontal) => Some(Self::E),
            (Self::E, Pipe::NorthEast) => None,
            (Self::E, Pipe::NorthWest) => Some(Self::N),
            (Self::E, Pipe::SouthWest) => Some(Self::S),
            (Self::E, Pipe::SouthEast) => None,
            // Rest
            (_, Pipe::Ground) => None,
            (_, Pipe::Creature) => {
                println!("[WARN]: ontop of creature");
                None
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Step {
    x: usize,
    y: usize,
    d: Direction,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Pos {
    fn checked_step(&self, dir: Direction, w: usize, h: usize) -> Option<Step> {
        match dir {
            Direction::N => (self.y > 0).then(|| Step {
                x: self.x,
                y: self.y - 1,
                d: dir,
            }),
            Direction::S => (self.y + 1 < h).then(|| Step {
                x: self.x,
                y: self.y + 1,
                d: dir,
            }),
            Direction::W => (self.x > 0).then(|| Step {
                x: self.x - 1,
                y: self.y,
                d: dir,
            }),
            Direction::E => (self.x + 1 < w).then(|| Step {
                x: self.x + 1,
                y: self.y,
                d: dir,
            }),
        }
    }
}

impl Step {
    fn step(&self, dir: Direction) -> Self {
        match dir {
            Direction::N => Self {
                x: self.x,
                y: self.y - 1,
                d: dir,
            },
            Direction::S => Self {
                x: self.x,
                y: self.y + 1,
                d: dir,
            },
            Direction::W => Self {
                x: self.x - 1,
                y: self.y,
                d: dir,
            },
            Direction::E => Self {
                x: self.x + 1,
                y: self.y,
                d: dir,
            },
        }
    }
}

#[test]
fn task1() {
    let text = open_first(&[
        "src/day10/input.txt",   //
        "src/day10/sample2.txt", //
        "src/day10/sample1.txt", //
    ])
    .unwrap();

    let grid = PipeGrid::parse(&text).expect("invalid grid");
    // println!("{}", grid);
    let pos = grid.creature().expect("no creature");
    println!("Creature {:?}", pos);

    let init = Direction::all()
        .into_iter()
        .filter_map(|dir| {
            let step = pos.checked_step(dir, grid.rows(), grid.cols())?;
            let pipe = grid.at(step.x, step.y);
            dir.next(pipe).map(|_| step)
        })
        .collect::<Vec<_>>();
    assert_eq!(init.len(), 2, "expected exactly 2 paths");
    let mut s1 = init[0];
    let mut s2 = init[1];

    let next = |s: &mut Step| {
        let pipe = grid.at(s.x, s.y);
        let dir = s.d.next(pipe).expect("invalid path");
        *s = s.step(dir);
    };

    let mut count = 1;
    while s1 != s2 {
        count += 1;
        next(&mut s1);
        next(&mut s2);
    }

    println!("Result: {}", count);
}

#[test]
fn task2() {
    let text = open_first(&[
        "src/day10/input.txt",   //
        "src/day10/sample4.txt", //
        "src/day10/sample3.txt", //
        "src/day10/sample1.txt", //
        "src/day10/sample2.txt", //
    ])
    .unwrap();

    let grid = PipeGrid::parse(&text).expect("invalid grid");
    // println!("{}", grid);
    let pos = grid.creature().expect("no creature");
    println!("Creature {:?}", pos);

    let init = Direction::all()
        .into_iter()
        .filter_map(|dir| {
            let step = pos.checked_step(dir, grid.cols(), grid.rows())?;
            let pipe = grid.at(step.x, step.y);
            dir.next(pipe)?;
            Some(step)
        })
        .collect::<Vec<_>>();
    assert_eq!(init.len(), 2, "expected exactly 2 paths");
    let mut s1 = init[0];
    let mut s2 = init[1];

    #[derive(PartialEq, Eq)]
    enum PathType {
        Up,
        Dn,
        Cont,
        Wall,
        None,
    }

    let mut path = grid
        .pipes
        .iter()
        .map(|_| PathType::None)
        .collect::<Vec<_>>();

    path[pos.x + grid.width * pos.y] = match (s1.d, s2.d) {
        (Direction::N, Direction::S) => PathType::Wall,
        (Direction::N, Direction::W) => PathType::Up,
        (Direction::N, Direction::E) => PathType::Up,
        (Direction::S, Direction::N) => PathType::Wall,
        (Direction::S, Direction::W) => PathType::Dn,
        (Direction::S, Direction::E) => PathType::Dn,
        (Direction::W, Direction::N) => PathType::Up,
        (Direction::W, Direction::S) => PathType::Dn,
        (Direction::W, Direction::E) => PathType::Cont,
        (Direction::E, Direction::N) => PathType::Up,
        (Direction::E, Direction::S) => PathType::Dn,
        (Direction::E, Direction::W) => PathType::Cont,
        _ => panic!("unexpected equal direction"),
    };

    let mut next = |s: &mut Step| {
        let pipe = grid.at(s.x, s.y);
        path[s.x + grid.width * s.y] = match pipe {
            Pipe::Vertical => PathType::Wall,
            Pipe::Horisontal => PathType::Cont,
            Pipe::NorthEast => PathType::Up,
            Pipe::NorthWest => PathType::Up,
            Pipe::SouthWest => PathType::Dn,
            Pipe::SouthEast => PathType::Dn,
            _ => panic!("unexpected pipe"),
        };
        let dir = s.d.next(pipe).expect("invalid path");
        *s = s.step(dir);
    };

    while s1 != s2 {
        next(&mut s1);
        next(&mut s2);
    }

    // Register last path
    next(&mut s1);

    // release mutable borrow of path
    drop(next);

    let count: usize = path
        .chunks_exact(grid.width)
        .map(|row| -> usize {
            // for cell in row {
            //     match cell {
            //         PathType::Once => print!("1 "),
            //         PathType::Twice => print!("2 "),
            //         PathType::None => print!("  "),
            //     }
            // }
            let count = (0..grid.width)
                .map(|pos| {
                    match row[pos] {
                        PathType::Up => {
                            // print!("^ ");
                            return 0;
                        }
                        PathType::Dn => {
                            // print!("v ");
                            return 0;
                        }
                        PathType::Cont => {
                            // print!("- ");
                            return 0;
                        }
                        PathType::Wall => {
                            // print!("| ");
                            return 0;
                        }
                        PathType::None => (),
                    }
                    enum C {
                        Up(bool),
                        Dn(bool),
                        Ground(bool),
                    }
                    let result = row[pos + 1..]
                        .iter()
                        .fold(C::Ground(false), |curr, path| match (curr, path) {
                            (C::Up(state), PathType::Up) => C::Ground(state),
                            (C::Up(state), PathType::Dn) => C::Ground(!state),
                            (C::Dn(state), PathType::Up) => C::Ground(!state),
                            (C::Dn(state), PathType::Dn) => C::Ground(state),
                            (C::Ground(state), PathType::Up) => C::Up(state),
                            (C::Ground(state), PathType::Dn) => C::Dn(state),
                            (C::Ground(state), PathType::Wall) => C::Ground(!state),
                            (C::Ground(state), PathType::None) => C::Ground(state),
                            (curr, PathType::Cont) => curr,
                            _ => panic!("unexpected combination"),
                        });

                    let C::Ground(inside) = result else {
                        panic!("invalid result");
                    };

                    if inside {
                        // print!("I ");
                        return 1;
                    } else {
                        // print!("O ");
                        return 0;
                    }
                })
                .sum();
            println!("");
            return count;
        })
        .sum();

    println!("Result: {}", count);
}
