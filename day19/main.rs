#![feature(destructuring_assignment)]
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{fs::File, io, io::prelude::*};

lazy_static! {
    static ref MAP: Mutex<HashMap<usize, String>> = Mutex::new(HashMap::new());
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day19/input.txt")?.read_to_string(&mut buffer)?;
    let lines = buffer.split("\n").map(|s| s.trim()).filter(|l| l != &"");
    for line in lines {
        if line.contains(":") {
            let chunks: Vec<&str> = line.split(":").map(|chunk| chunk.trim()).collect();
            let mut rule = chunks[1].to_string();
            rule.retain(|c| c != '\"');
            MAP.lock().unwrap().insert(chunks[0].parse().unwrap(), rule);
        } else {
            println!("{}", line);
            let mut production =
                vec![
                    vec![vec![false; MAP.lock().unwrap().len() + 1]; line.len() + 1];
                    line.len() + 1
                ];
            let m = MAP.lock().unwrap();
            for (i, symbol) in line.chars().enumerate() {
                let symbol = symbol.to_string();
                for (key, val) in m.iter() {
                    if *val == symbol {
                        production[0][i + 1][*key + 1] = true;
                    }
                }
            }

            for l in 2..line.len() + 1 {
                for s in 1..line.len() + 1 - l + 1 {
                    for p in 1..l - 1 {
                        let mut keys = m.keys().copied().collect::<Vec<usize>>();
                        keys.sort();
                        for key in keys {
                            let rule = &m[&key];
                            if rule.chars().all(|c| c.is_alphabetic()) {
                                continue;
                            }

                            let chunks: Vec<&str> = rule.split("|").map(|ch| ch.trim()).collect();
                            for mut chunk in chunks {
                                chunk = chunk.trim();
                                let sub_chunks: Vec<usize> = chunk
                                    .split(" ")
                                    .map(|ch| ch.parse::<usize>().unwrap())
                                    .collect();
                                if production[p][s][sub_chunks[0]]
                                    && production[l - p][s + p][sub_chunks[1]]
                                {
                                    production[l][s][key] = true;
                                }
                            }
                        }
                    }
                }
            }

            println!("{:?}", production);
            println!("{}", production[line.len()][1][1]);
            break;
        }
    }

    Ok(())
}
