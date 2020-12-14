#![feature(destructuring_assignment)]
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::{fs::File, io, io::prelude::*};

// THIS WILL NOT WORK ON 32 BIT SYSTEMS!

lazy_static! {
    static ref ADDRESS: Regex = Regex::new(r"\[(\d+)\]").unwrap();
    static ref VALUE: Regex = Regex::new(r"= (.*)").unwrap();
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day14/input.txt")?.read_to_string(&mut buffer)?;
    let lines = buffer.split("\n").map(|s| s.trim());

    // address -> value
    let mut mem: BTreeMap<usize, usize> = BTreeMap::new();
    // index -> change to what?
    let mut mask: HashMap<usize, bool> = HashMap::new();
    for line in lines {
        if line.starts_with("mask") {
            let mut index = 36;
            mask.clear();
            let line = &line[7..];
            for c in line.chars() {
                index -= 1;
                match c {
                    '0' => {
                        mask.insert(index, false);
                    }
                    '1' => {
                        mask.insert(index, true);
                    }
                    'X' => {}
                    _ => panic!("bad input {}", c),
                }
            }

            continue;
        } else {
            let caps = ADDRESS.captures(line).unwrap();
            let address = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap();

            let caps = VALUE.captures(line).unwrap();
            let mut value = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap();

            for (index, val) in mask.iter() {
                if *val {
                    value |= 1 << index;
                } else {
                    value &= !(1 << index);
                }
            }

            mem.insert(address, value);
        }
    }

    println!("Answer 1: {}", mem.values().sum::<usize>());
    part2()?;

    Ok(())
}

fn part2() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day14/input.txt")?.read_to_string(&mut buffer)?;
    let lines = buffer.split("\n").map(|s| s.trim());

    let mut mem: BTreeMap<usize, usize> = BTreeMap::new();
    let mut mask: String = "".to_string();
    for line in lines {
        if line.starts_with("mask") {
            let line = &line[7..];
            mask = line.trim().to_string();
        } else {
            let caps = ADDRESS.captures(line).unwrap();
            let address = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap();

            let caps = VALUE.captures(line).unwrap();
            let mut value = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap();

            let mut addresses: Vec<usize> = Vec::new();
            addresses.push(address);
            for (i, c) in mask.chars().rev().enumerate() {
                match c {
                    '0' => {}
                    '1' => {
                        for address in addresses.iter_mut() {
                            *address |= 1 << i;
                        }
                    }
                    'X' => {
                        let len = addresses.len();
                        addresses.append(&mut addresses.clone());
                        for (a_i, address) in addresses.iter_mut().enumerate() {
                            if a_i < len {
                                *address ^= 1 << i;
                            }
                        }
                    }
                    _ => {}
                }
            }

            for a in addresses {
                mem.insert(a, value);
            }
        }
    }

    println!("Answer 2: {}", mem.values().sum::<usize>());

    Ok(())
}
