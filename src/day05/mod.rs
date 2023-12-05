#![cfg(test)]
use std::fs;

#[derive(Debug)]
struct Range {
    src: usize,
    len: usize,
}

impl Range {
    fn end(&self) -> usize {
        self.src + self.len
    }
}

#[derive(Debug)]
struct RangeMap {
    dst: usize,
    src: usize,
    len: usize,
}

impl RangeMap {
    fn parse(line: &str) -> Self {
        let mut iter = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().expect("Unable to parse number"));
        let dst = iter.next().expect("No destination");
        let src = iter.next().expect("No source");
        let len = iter.next().expect("No length");
        assert_eq!(iter.count(), 0, "Unexpected numbers");
        return Self { dst, src, len };
    }

    fn map(&self, src: usize) -> Option<usize> {
        src.checked_sub(self.src)
            .filter(|&len| len < self.len)
            .map(|len| self.dst + len)
    }

    fn src_end(&self) -> usize {
        self.src + self.len
    }

    fn overlap_range(&self, range: &Range) -> bool {
        self.src < range.end() && range.src < self.src_end()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum EntryType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl EntryType {
    fn parse(str: &str) -> Option<Self> {
        match str.trim() {
            "seed" => Some(Self::Seed),
            "soil" => Some(Self::Soil),
            "fertilizer" => Some(Self::Fertilizer),
            "water" => Some(Self::Water),
            "light" => Some(Self::Light),
            "temperature" => Some(Self::Temperature),
            "humidity" => Some(Self::Humidity),
            "location" => Some(Self::Location),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct ConversionMap {
    src: EntryType,
    dst: EntryType,
    map: Vec<RangeMap>,
}

impl ConversionMap {
    fn parse(header: &str) -> Self {
        assert!(header.ends_with(" map:"), "Missing map declaration");
        let header = header.trim_end_matches(" map:");
        let (src, dst) = header.split_once("-to-").expect("Missing type declaraion");
        let src = EntryType::parse(src).expect("Invalid source");
        let dst = EntryType::parse(dst).expect("Unable to parse destination");
        let map = Vec::new();
        return ConversionMap { src, dst, map };
    }

    fn convert(&self, src: usize) -> usize {
        self.map
            .iter()
            .find_map(|range| range.map(src))
            .unwrap_or(src)
    }

    fn convert_ranges(&self, src: &mut Vec<Range>, dst: &mut Vec<Range>) {
        let min = std::cmp::min;
        let max = std::cmp::max;
        while let Some(range) = src.pop() {
            // Find first range with overlap
            let Some(map) = self.map.iter().find(|map| map.overlap_range(&range)) else {
                dst.push(range);
                continue;
            };

            // Slice ranges
            let pos = max(range.src, map.src);
            let end = min(range.end(), map.src_end());

            /*
            println!(
                "{}..{} </> {}..{}",
                map.src,
                map.src_end(),
                range.src,
                range.end(),
            );
            */

            // println!("overlap: {}..{}", pos, end);

            if pos > range.src {
                // println!("prefix: {}..{}", range.src, pos);
                src.push(Range {
                    src: range.src,
                    len: pos - range.src,
                });
            }

            if end < range.end() {
                // println!("postfix: {}..{}", end, range.end());
                src.push(Range {
                    src: end,
                    len: range.end() - end,
                });
            }

            let offset = pos - map.src;
            dst.push(Range {
                src: map.dst + offset,
                len: end - pos,
            });
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<ConversionMap>,
}

impl Almanac {
    fn parse(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let mut iter = content.lines();
        let seeds = iter
            .next()
            .expect("Missing entry line")
            .split_once(':')
            .expect("Missing seeds declaration")
            .1
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<usize>().expect("Unable to parse seed"))
            .collect::<Vec<_>>();

        let mut maps = Vec::new();
        let mut map: Option<ConversionMap> = None;

        while let Some(line) = iter.next() {
            let line = line.trim();

            // Separation
            if line.is_empty() {
                if let Some(map) = map.take() {
                    maps.push(map);
                }
                continue;
            }

            // Parse header if no map
            let Some(map) = &mut map else {
                map.replace(ConversionMap::parse(line));
                continue;
            };

            map.map.push(RangeMap::parse(line));
        }

        if let Some(map) = map.take() {
            maps.push(map);
        }

        return Self { seeds, maps };
    }
}

#[test]
fn task1() {
    // let path = "src/day05/sample.txt";
    let path = "src/day05/input.txt";
    let data = Almanac::parse(path);
    // println!("Data: {data:#?}");

    let mut curr = EntryType::Seed;
    let mut vals = data.seeds.clone();

    while let Some(conv) = data.maps.iter().find(|conv| conv.src == curr) {
        println!("{:?} -> {:?}", conv.src, conv.dst);
        for val in &mut vals {
            *val = conv.convert(*val);
        }
        curr = conv.dst;
    }

    assert_eq!(curr, EntryType::Location);

    let result = vals.iter().min().expect("Missing output value");
    println!("Result: {}", result);
}

#[test]
fn task2() {
    // let path = "src/day05/sample.txt";
    let path = "src/day05/input.txt";
    let data = Almanac::parse(path);
    // println!("Data: {data:#?}");

    let mut curr = EntryType::Seed;
    let mut src = data
        .seeds
        .chunks(2)
        .map(|view| Range {
            src: view[0],
            len: view[1],
        })
        .collect::<Vec<_>>();
    let mut dst = Vec::with_capacity(src.capacity());

    while let Some(conv) = data.maps.iter().find(|conv| conv.src == curr) {
        println!("{:?} -> {:?}", conv.src, conv.dst);
        conv.convert_ranges(&mut src, &mut dst);
        std::mem::swap(&mut src, &mut dst);
        dst.clear();
        curr = conv.dst;
    }

    assert_eq!(curr, EntryType::Location);

    let result = src
        .iter()
        .map(|range| range.src)
        .min()
        .expect("Missing output value");
    println!("Result: {}", result);
}
