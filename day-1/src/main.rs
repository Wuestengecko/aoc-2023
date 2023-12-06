use std::fs::File;
use std::io::prelude::*;

const DIGIT_NAMES: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut sum: u64 = 0;
    for line in buf.lines() {
        let mut first: Option<u8> = None;
        let mut last: Option<u8> = None;

        for i in 0..line.len() {
            let dig = extract_digit(&line[i..]);

            if let Some(dig) = dig {
                if first.is_none() {
                    first = Some(dig);
                }
                last = Some(dig);
            }
        }
        let number = first.unwrap() * 10 + last.unwrap();
        println!("{} -> {}", line, number);
        sum += number as u64;
    }

    println!("sum: {}", sum);

    Ok(())
}

fn extract_digit(input: &str) -> Option<u8> {
    if let Some(dig) = input.chars().nth(0).unwrap().to_digit(10) {
        return Some(dig as u8);
    } else {
        for j in 1..DIGIT_NAMES.len() {
            if input.starts_with(DIGIT_NAMES[j]) {
                return Some(j as u8);
            }
        }
    }
    None
}
