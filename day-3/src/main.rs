use std::cmp::min;
use std::io::{prelude::*, stdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    let lines: Vec<_> = buf.lines().collect();

    // _part1(&lines)?;
    _part2(&lines)?;

    Ok(())
}

fn _part1(lines: &Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut sum = 0;
    for (row, line) in lines.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            let char = line.chars().nth(col).unwrap();
            if !char.is_ascii_digit() {
                col += 1;
                continue;
            }

            let len = line[col..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .count();
            if is_part_number(lines, row, col, len) {
                println!(
                    "Number {} at {}x{} is a part number",
                    &line[col..col + len],
                    row,
                    col
                );
                sum += line[col..col + len].parse::<u64>()?;
            } else {
                println!(
                    "Number {} at {}x{} is NOT a part number",
                    &line[col..col + len],
                    row,
                    col
                );
            }
            col += len;
        }
    }
    println!("Sum: {}", sum);

    Ok(())
}

fn is_part_number(lines: &Vec<&str>, ln: usize, cn: usize, len: usize) -> bool {
    let lmin = if ln == 0 { 0 } else { ln - 1 };
    let lmax = min(ln + 2, lines.len());
    let cmin = if cn == 0 { 0 } else { cn - 1 };
    let cmax = min(cn + len + 1, lines[ln].len());
    println!(
        "Checking grid around {}x{}..{}x{}, which is {}x{}..{}x{}",
        ln,
        cn,
        ln,
        cn + len,
        lmin,
        cmin,
        lmax,
        cmax
    );

    lines[lmin..lmax].iter().any(|line| {
        line[cmin..cmax]
            .chars()
            .any(|c| c != '.' && !c.is_ascii_digit())
    })
}

fn _part2(lines: &Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut sum: u64 = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '*' {
                let numbers = _find_part_numbers_near(lines, row, col);
                println!(
                    "Gear at {}x{} has {} adjacent numbers: {:?}",
                    row,
                    col,
                    numbers.len(),
                    numbers
                );
                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    }
    println!("Sum: {}", sum);

    Ok(())
}

fn _find_part_numbers_near(lines: &Vec<&str>, row: usize, col: usize) -> Vec<u64> {
    let mut numbers = Vec::new();

    let lmin = if row == 0 { 0 } else { row - 1 };
    let lmax = min(row + 2, lines.len());
    let cmin = if col == 0 { 0 } else { col - 1 };
    let cmax = min(col + 2, lines[row].len());

    println!(
        "Checking grid around {}x{}, which is {}x{}..{}x{}",
        row, col, lmin, cmin, lmax, cmax
    );

    for (ln, line) in lines[lmin..lmax].iter().enumerate() {
        let mut cn = cmin;
        while cn < cmax {
            let mut cstart = cn;
            if line.chars().nth(cstart).unwrap().is_ascii_digit() {
                while cstart > 0
                    && cstart <= cmin
                    && line
                        .chars()
                        .nth(cstart - 1)
                        .and_then(|c| Some(c.is_ascii_digit()))
                        .unwrap_or(false)
                {
                    cstart -= 1;
                }
            }

            let cend = cn
                + line[cn..]
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .count();
            println!("cn = {}, cstart = {}, cend = {}", cn, cstart, cend);

            if cstart < cend {
                let num = line[cstart..cend].parse::<u64>().unwrap();
                println!(
                    "Found adjacent number {} at {}x{}..{}x{}",
                    num,
                    ln + lmin,
                    cstart,
                    ln + lmin,
                    cend
                );
                numbers.push(num);
                cn = cend + 1;
            } else {
                cn += 1;
            }
        }
    }

    numbers
}
