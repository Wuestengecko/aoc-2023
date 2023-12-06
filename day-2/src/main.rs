use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    // _part1(&buf)?;
    _part2(&buf)?;

    Ok(())
}

fn _part1(buf: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut sum = 0;
    for (i, game) in (1..).zip(buf.lines()) {
        let (gid, game) = game.split_once(": ").unwrap();
        assert!(gid == format!("Game {}", i));

        let mut possible = true;
        'set: for set in game.split("; ") {
            for (num, color) in set
                .split(", ")
                .map(|s| s.split_once(" ").unwrap())
                .map(|(n, c)| (n.parse::<usize>().unwrap(), c))
            {
                if color == "red" {
                    if num > 12 {
                        possible = false;
                        break 'set;
                    }
                } else if color == "green" {
                    if num > 13 {
                        possible = false;
                        break 'set;
                    }
                } else if color == "blue" {
                    if num > 14 {
                        possible = false;
                        break 'set;
                    }
                } else {
                    possible = false;
                    break 'set;
                }
            }
        }

        println!(
            "{} Game {:>3}: {}",
            if possible { "✔" } else { "✘" },
            i,
            game
        );
        if possible {
            sum += i;
        }
    }
    println!("Sum: {}", sum);

    Ok(())
}

fn _part2(buf: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut sum = 0;
    for game in buf.lines() {
        let (_, game) = game.split_once(": ").unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for set in game.split("; ") {
            for (num, color) in set
                .split(", ")
                .map(|s| s.split_once(" ").unwrap())
                .map(|(n, c)| (n.parse::<usize>().unwrap(), c))
            {
                if color == "red" {
                    red = max(red, num);
                } else if color == "green" {
                    green = max(green, num);
                } else if color == "blue" {
                    blue = max(blue, num);
                }
            }
        }
        let power = red * green * blue;
        sum += power;
    }
    println!("Sum of powers: {}", sum);

    Ok(())
}
