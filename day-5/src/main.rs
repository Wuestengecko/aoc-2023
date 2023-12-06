use std::cmp::min;
use std::io::prelude::*;
use std::io::stdin;
use std::ops::Range;

use regex::Regex;

const MAP_HEADER_REGEX: &str = r"^(?P<from>[a-z]+)-to-(?P<to>[a-z]+) map:$";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    let lines: Vec<_> = buf.lines().collect();

    let result = _part2(&lines)?;
    println!("Result: {}", result);

    Ok(())
}

struct Map {
    ranges: Vec<MapRange>,
}

#[derive(Clone)]
struct MapRange {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl Map {
    fn from_iter(
        iter: &mut dyn Iterator<Item = &&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut ranges = vec![];
        while let Some(line) = iter.next() {
            if line.is_empty() {
                break;
            }

            let numbers = line
                .split_whitespace()
                .map(|s| s.parse())
                .collect::<Result<Vec<u64>, _>>()?;
            if numbers.len() != 3 {
                return Err(format!("Invalid map line: {}", line))?;
            }
            ranges.push(MapRange {
                dest_start: numbers[0],
                source_start: numbers[1],
                length: numbers[2],
            });
        }

        Ok(Self { ranges })
    }

    fn map(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if range.has(value) {
                return range.map(value);
            }
        }
        value
    }

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let mut new_ranges = vec![];
        let mut start = range.start;
        while start < range.end {
            let r = self.find_range(start);
            let end = min(range.end, r.source_end());
            new_ranges.push(r.map(start)..r.map(end));
            start = end;
        }

        new_ranges
    }

    fn find_range(&self, value: u64) -> MapRange {
        let mut high = u64::MAX;
        for range in &self.ranges {
            if range.has(value) {
                return range.clone();
            } else {
                if range.source_start > value && range.source_start < high {
                    high = range.source_start;
                }
            }
        }
        MapRange {
            dest_start: value,
            source_start: value,
            length: high - value,
        }
    }
}

impl MapRange {
    fn source_end(&self) -> u64 {
        self.source_start + self.length
    }

    fn dest_end(&self) -> u64 {
        self.dest_start + self.length
    }

    fn has(&self, value: u64) -> bool {
        value >= self.source_start && value < self.source_end()
    }

    fn map(&self, value: u64) -> u64 {
        assert!(value >= self.source_start && value <= self.source_end());
        self.dest_start + (value - self.source_start)
    }
}

fn _part1(lines: &[&str]) -> Result<u64, Box<dyn std::error::Error>> {
    let header_rx = Regex::new(MAP_HEADER_REGEX)?;

    let mut stage = "seed";
    let mut line_iter = lines.iter();
    let mut numbers = {
        let line = line_iter.next().unwrap();
        assert!(line.starts_with("seeds: "));
        line[7..]
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<Vec<u64>, _>>()?
    };
    assert!(line_iter.next() == Some(&""));

    while stage != "location" {
        let header = line_iter.next().unwrap();
        match header_rx.captures(header) {
            Some(c) => {
                let from = c.name("from").unwrap().as_str();
                let to = c.name("to").unwrap().as_str();
                assert!(stage == from);
                stage = to;
            }
            None => Err(format!("Invalid header: {}", header))?,
        }
        let map = Map::from_iter(&mut line_iter)?;
        numbers = numbers.iter().map(|n| map.map(*n)).collect();
    }

    Ok(*numbers.iter().min().unwrap())
}

fn _part2(lines: &[&str]) -> Result<u64, Box<dyn std::error::Error>> {
    let header_rx = Regex::new(MAP_HEADER_REGEX)?;

    let mut stage = "seed";
    let mut line_iter = lines.iter();
    let numbers = {
        let line = line_iter.next().unwrap();
        assert!(line.starts_with("seeds: "));
        line[7..]
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?
    };
    println!("{}, {:?}", numbers.len(), numbers);
    let mut ranges: Vec<Range<u64>> = (0..numbers.len())
        .step_by(2)
        .map(|n| {
            let (l, r) = (numbers[n], numbers[n + 1]);
            l..l + r
        })
        .collect();
    assert!(line_iter.next() == Some(&""));
    assert!(ranges.len() > 0);

    while stage != "location" {
        let header = line_iter.next().unwrap();
        match header_rx.captures(header) {
            Some(c) => {
                let from = c.name("from").unwrap().as_str();
                let to = c.name("to").unwrap().as_str();
                assert!(stage == from);
                println!("Mapping {} ranges from {} to {}", ranges.len(), from, to);
                stage = to;
            }
            None => Err(format!("Invalid header: {}", header))?,
        }
        let map = Map::from_iter(&mut line_iter)?;
        ranges = ranges.iter().map(|n| map.map_range(n)).flatten().collect();
    }

    Ok(ranges.iter().map(|n| n.start).min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [&str; 33] = [
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];

    #[test]
    fn test_part1() {
        let result = _part1(&TEST_INPUT).unwrap();
        assert!(result == 35);
    }

    #[test]
    fn test_part2() {
        let result = _part2(&TEST_INPUT).unwrap();
        assert!(result == 46);
    }
}
