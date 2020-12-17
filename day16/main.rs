#![feature(destructuring_assignment)]
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{fs, io};

// THIS IS THE WORST CODE I HAVE EVER WRITTEN
// It might take a while to solve depending on luck (random values ftw)

// Note for when I refactor: good way to solve is to have a matrix of valid
// options (ticket_parameter x constraints) and then do gaussian elimination. if
// i'm correct, this should yield an upper triangular matrix. then the answer is trivial.

lazy_static! {
    static ref CONSTRAINT: Regex = Regex::new(r"(\w+ ?\w+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

fn main() -> io::Result<()> {
    let file_contents = fs::read_to_string("day16/input.txt").unwrap();
    let lines = file_contents.lines().map(|s| s.trim());
    // field -> (lower bound, upper bound) or (lower bound, upper bound)
    let mut checks: HashMap<String, [(usize, usize); 2]> = HashMap::new();

    let mut nearbies = false;
    let mut digits: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        if line == "" {
            continue;
        } else if line == "nearby tickets:" {
            nearbies = true;
            continue;
        }

        if nearbies {
            let inputs = line.split(",").map(|d| d.parse::<usize>().unwrap());
            let mut valid = false;
            for digit in inputs.clone() {
                let mut digit_valid = false;
                for (_, check) in checks.iter() {
                    let satisfied = (check[0].0 <= digit && check[0].1 >= digit)
                        || (check[1].0 <= digit && check[1].1 >= digit);
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

        if let Some(caps) = CONSTRAINT.captures(line) {
            let key = caps.get(1).unwrap().as_str();
            let first_low = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let first_high = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let second_low = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let second_high = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();
            checks.insert(
                key.to_string(),
                [(first_low, first_high), (second_low, second_high)],
            );
        }
    }

    let mut ordered_checks: HashMap<String, usize> = HashMap::new();
    let mut assigned_checks: HashSet<String> = HashSet::new();
    let mut asigned_indices: HashSet<usize> = HashSet::new();
    // i is the index within one ticket
    let mut k = 0;
    loop {
        let i = k % 20;
        // println!("i {}", i);
        // for i in 0..digits[0].len() {
        if asigned_indices.contains(&i) {
            k += 1;
            if ordered_checks.len() == 20 && asigned_indices.len() == 20 {
                break;
            }
            continue;
        }

        // j is the index of the ticket
        let mut ok = false;
        for (key, check) in checks.iter() {
            if assigned_checks.contains(key) {
                continue;
            }

            let mut satisfied = false;
            for j in 0..digits.len() {
                // println!("dig {} {} {}", digits[j][i], j, i);
                satisfied = (check[0].0 <= digits[j][i] && check[0].1 >= digits[j][i])
                    || (check[1].0 <= digits[j][i] && check[1].1 >= digits[j][i]);
                if !satisfied {
                    // println!("ticket {} failed {} for {} with {}", j, i, key, digits[j][i]);
                    break;
                }
            }
            if !satisfied {
                ok = false;
                continue;
            } else {
                // println!("SATISFIED? {} {} {}", key, satisfied, i);
                ordered_checks.insert(key.to_string(), i);
                assigned_checks.insert(key.to_string());
                ok = true;
                asigned_indices.insert(i);
                break;
            }
        }

        if !ok {
            // println!("Problem with {}", i);
            let mut satisfies: Vec<String> = Vec::new();
            for (key, check) in checks.iter() {
                let mut satisfied = false;
                for j in 0..digits.len() {
                    satisfied = (check[0].0 <= digits[j][i] && check[0].1 >= digits[j][i])
                        || (check[1].0 <= digits[j][i] && check[1].1 >= digits[j][i]);
                    if !satisfied {
                        break;
                    }
                }

                if !satisfied {
                    continue;
                } else {
                    satisfies.push(key.to_string());
                }
            }

            let rand_key = satisfies.choose(&mut rand::thread_rng()).unwrap().clone();
            let curr_i = ordered_checks.get(&rand_key).unwrap().clone();
            ordered_checks.insert(rand_key.to_string(), i);
            assigned_checks.insert(rand_key.to_string());
            asigned_indices.insert(i);
            asigned_indices.remove(&curr_i);
            // println!("Out of options {:?} chose match {} ", satisfies, rand_key);
        } else {
            // println!("LENGTH {}", ordered_checks.len());
            if ordered_checks.len() == 20 && asigned_indices.len() == 20 {
                break;
            } else {
                k += 1;
            }
        }
    }

    // println!("{:?} {:?}", ordered_checks, asigned_indices);
    let splits: Vec<&str> = "131,103,109,67,127,97,89,79,163,59,73,83,61,107,53,193,167,101,71,197"
        .split(",")
        .collect();
    let mut total = 1;
    for (k, v) in ordered_checks.iter() {
        if k.starts_with("departure") {
            // println!("{} {} {}", k, v, splits[*v]);
            total *= splits[*v].parse::<usize>().unwrap();
        }
    }

    println!("{}", total);
    Ok(())
}
