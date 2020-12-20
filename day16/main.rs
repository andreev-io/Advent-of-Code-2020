#![feature(destructuring_assignment)]
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::File, io, io::prelude::*};

// Note for when I refactor: good way to solve is to have a matrix of valid
// options (ticket_parameter x constraints) and then do gaussian elimination. if
// i'm correct, this should yield an upper triangular matrix. then the answer is trivial.

lazy_static! {
    static ref CONSTRAINT: Regex = Regex::new(r"(\w+ ?\w+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

struct Check {
    name: String,
    bounds: [(usize, usize); 2],
    idx: i32,
}

fn main() -> io::Result<()> {
    // field -> (lower bound, upper bound) or (lower bound, upper bound)
    let mut checks: Vec<Check> = Vec::new();

    let mut buffer = String::new();
    File::open("day16/input.txt")?.read_to_string(&mut buffer)?;
    let lines: Vec<&str> = buffer.split("\n\n").collect();

    for line in lines[0].split("\n") {
        if let Some(caps) = CONSTRAINT.captures(line) {
            let key = caps.get(1).unwrap().as_str();
            let first_low = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let first_high = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let second_low = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let second_high = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();
            checks.push(Check {
                name: key.to_string(),
                bounds: [(first_low, first_high), (second_low, second_high)],
                idx: -1,
            });
        }
    }

    let my_ticket: &str = lines[1].split("\n").collect::<Vec<&str>>()[1];
    let mut digits: Vec<Vec<usize>> = Vec::new();

    // Filter out corrupt tickets
    for (i, line) in lines[2].split("\n").enumerate() {
        if i == 0 {
            continue;
        }

        let inputs = line.split(",").map(|d| d.parse::<usize>().unwrap());
        let mut valid = false;
        for digit in inputs.clone() {
            let mut digit_valid = false;
            for check in checks.iter() {
                let satisfied = (check.bounds[0].0 <= digit && check.bounds[0].1 >= digit)
                    || (check.bounds[1].0 <= digit && check.bounds[1].1 >= digit);
                if satisfied {
                    digit_valid = true;
                    break;
                }
            }

            if digit_valid {
                valid = true;
            } else {
                valid = false;
                break;
            }
        }

        if valid {
            digits.push(inputs.collect());
        }
    }

    let mut matrix: Vec<Vec<usize>> = vec![vec![0; digits[0].len()]; checks.len()];

    // i is the index within a ticket
    for i in 0..digits[0].len() {
        // j is the index of the ticket
        for (idx, check) in checks.iter().enumerate() {
            let mut matched = true;
            // find criteria that are matched by digits at this index in all
            // ticket
            for j in 0..digits.len() {
                let satisfied = (check.bounds[0].0 <= digits[j][i]
                    && check.bounds[0].1 >= digits[j][i])
                    || (check.bounds[1].0 <= digits[j][i] && check.bounds[1].1 >= digits[j][i]);
                if !satisfied {
                    matched = false;
                    break;
                }
            }

            if matched {
                matrix[idx][i] = 1;
            }
        }
    }

    let mut removed_count = 0;
    loop {
        let mut removed = 0;
        for (check_index, row) in matrix.iter().enumerate() {
            let ones = row.iter().filter(|&i| *i == 1).count();
            if ones == 1 {
                let idx = row.iter().position(|&dig| dig == 1).unwrap();
                checks[check_index].idx = idx as i32;
                removed = idx;
                removed_count += 1;
                break;
            }
        }

        for i in 0..matrix.len() {
            matrix[i][removed] = 0;
        }

        if removed_count == matrix[0].len() {
            break;
        }
    }

    let splits: Vec<&str> = my_ticket.split(",").collect();
    let mut total = 1;
    for check in checks.iter() {
        if check.name.starts_with("departure") {
            total *= splits[check.idx as usize].parse::<usize>().unwrap();
        }
    }

    println!("{}", total);
    Ok(())
}
