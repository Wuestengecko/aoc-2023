use std::collections::HashMap;
use std::io::prelude::*;
use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    let lines: Vec<_> = buf.lines().collect();

    let result = _part2(&lines)?;
    println!("Result: {}", result);

    Ok(())
}

fn parse_lines(lines: &Vec<&str>) -> Result<HashMap<u64, u64>, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    for line in lines.iter() {
        let (id, line) = line.split_once(": ").unwrap();
        let id = id.split_whitespace().collect::<Vec<_>>()[1].parse()?;
        let line = line
            .split(" | ")
            .map(|l| {
                l.split_whitespace()
                    .filter_map(|n| match n.parse::<u64>() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let winners = line[1].iter().filter(|n| line[0].contains(n)).count();
        map.insert(id, winners as u64);
    }
    Ok(map)
}

fn _part1(lines: &Vec<&str>) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(parse_lines(lines)?
        .values()
        .map(|v| match *v {
            0 => 0,
            v => (2u64).pow(v as u32 - 1),
        })
        .reduce(|a, b| a + b)
        .unwrap())
}

fn _part2(lines: &Vec<&str>) -> Result<u64, Box<dyn std::error::Error>> {
    let map = parse_lines(lines)?;
    let mut copies = HashMap::<u64, u64>::new();

    for id in 1..map.len() as u64 + 1 {
        let winners = map[&id];
        let cards = *copies.entry(id).or_insert(1);

        for i in 0..winners {
            *copies.entry(id + i + 1).or_insert(1) += cards;
        }
    }

    Ok(copies.values().sum())
}
