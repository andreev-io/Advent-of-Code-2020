#![feature(destructuring_assignment)]
use itertools::Itertools;
use std::collections::BTreeMap;
use std::{fs::File, io, io::prelude::*};

fn main() -> io::Result<()> {
    let mut char_bag: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    // let mut subrules: BTreeMap<(usizche, usize), String> = BTreeMap::new();
    let mut buffer = String::new();
    File::open("day19/input.txt")?.read_to_string(&mut buffer)?;
    let lines = buffer.split("\n").map(|s| s.trim()).filter(|l| l != &"");
    let mut grammar_size = 0;
    for line in lines.clone() {
        if line.contains(":") {
            grammar_size += 1;
        }
    }

    let mut rules = vec!["".to_string(); grammar_size];
    for line in lines.clone() {
        if line.contains(":") {
            let chunks: Vec<&str> = line.split(":").collect();
            rules[chunks[0].parse::<usize>().unwrap()] = chunks[1].trim().to_string();
        }
    }

    let mut subrules: Vec<Vec<String>> = vec![vec!["".to_string(); rules.len()]; rules.len()];
    let mut bag: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); rules.len()]; rules.len()];
    for line in lines {
        if !line.contains(":") && line != "" {
            let mut m = vec![vec![Vec::<usize>::new(); line.len()]; line.len()];
            for (i, c) in line.chars().enumerate() {
                let key = c.to_string();
                if let Some(ixs) = char_bag.get(&key) {
                    for ix in ixs {
                        m[i][i].push(*ix);
                    }
                } else {
                    for idx in 0..rules.len() {
                        if rules[idx].contains(c) {
                            if let Some(cache) = char_bag.get_mut(&c.to_string()) {
                                cache.push(idx);
                            } else {
                                char_bag.insert(c.to_string(), vec![idx]);
                            }
                            m[i][i].push(idx);
                        }
                    }
                }
            }

            for l in 1..line.len() {
                for r in 0..line.len() - l {
                    for t in 0..l {
                        let alpha = m[r][r + t].clone();
                        let beta = m[r + t + 1][r + l].clone();
                        let mut pairs =
                            beta.iter()
                                .cartesian_product(alpha.iter())
                                .collect::<Vec<(&usize, &usize)>>();
                        let mut pairs_rev = alpha
                            .iter()
                            .cartesian_product(beta.iter())
                            .collect::<Vec<(&usize, &usize)>>();
                        pairs.append(&mut pairs_rev);
                        pairs.dedup();

                        for pair in pairs {
                            if subrules[*pair.0][*pair.1] == "" {
                                subrules[*pair.0][*pair.1] = make_subrule(pair);
                            }
                            let v = &bag[*pair.0][*pair.1];
                            if v.len() != 0 {
                                for ix in v {
                                    m[r][r + l].push(*ix);
                                }
                            } else {
                                for idx in 0..rules.len() {
                                    if rules[idx].contains(&subrules[*pair.0][*pair.1]) {
                                        bag[*pair.0][*pair.1].push(idx);
                                        m[r][r + l].push(idx);
                                    }
                                }
                            }
                        }
                        m[r][r + l].sort();
                        m[r][r + l].dedup();
                    }
                }
            }

            if m[0][line.len() - 1].len() != 0 {
                println!("ok");
            } else {
                println!("not ok");
            }
        }
    }

    Ok(())
}

fn make_subrule(tuple: (&usize, &usize)) -> String {
    let mut s = tuple.0.to_string();
    s.push_str(&String::from(" "));
    s.push_str(&tuple.1.to_string());
    s
}
