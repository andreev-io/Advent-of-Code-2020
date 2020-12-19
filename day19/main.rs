#![feature(destructuring_assignment)]
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{fs::File, io, io::prelude::*};

lazy_static! {
    static ref MAP: Mutex<HashMap<usize, (String, bool)>> = Mutex::new(HashMap::new());
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day19/input.txt")?.read_to_string(&mut buffer)?;
    let lines = buffer.split("\n").map(|s| s.trim());
    for line in lines {
        if line.contains(":") {
            let parts = line.split(":");
            let parts = parts.map(|s| s.trim()).collect::<Vec<&str>>();
            let key = parts[0];
            let mut val = parts[1].to_string();
            let len_raw = val.len();
            val.retain(|c| c != '\"');
            let len_filtered = val.len();
            MAP.lock()
                .unwrap()
                .insert(key.parse().unwrap(), (val, len_raw != len_filtered));
        } else if line != "" {
            println!("{:?}", check_message(line, 0));
            break;
        }
    }

    Ok(())
}

fn check_message(message: &str, idx: usize) -> (Vec<usize>, usize) {
    // Need this scope to unlock destroyed map object when it goes out of scope.
    let rule = {
        let m = MAP.lock().unwrap();
        m.get(&idx).unwrap().clone()
    };

    // If the rule is a terminator, find match indices
    if rule.1 {
        let matches: Vec<(usize, &str)> = message.match_indices(&rule.0).collect();
        let (indices, _): (Vec<usize>, Vec<&str>) = matches.iter().cloned().unzip();
        return (indices, 1);
    } else {
        let mut answer = Vec::new();
        let rule_groups = rule.0.split("|");
        for mut group in rule_groups {
            group = group.trim();
            let to_check = group.split(" ");
            let mut options: Vec<(Vec<usize>, usize)> = Vec::new();
            let mut has_empty = false;
            for i in to_check {
                let i = i.trim().parse::<usize>().unwrap();
                let new_option = check_message(message, i);
                if new_option.0.len() == 0 {
                    has_empty = true;
                    break;
                } else {
                    options.push(new_option);
                }
            }

            println!("rule {} group {} -> {:?}", rule.0, group, options);
            if !has_empty {
                let mut reps = find_consec_representatives(options);
                if reps.len() != 0 {
                    answer.append(&mut reps);
                }
            }
        }

        answer.sort();
        println!("msg {} match {}: {:?}", message, rule.0, answer);
        return answer;
    }
}

fn find_consec_representatives(v: Vec<(Vec<usize>, usize)>) -> Vec<usize> {
    if v.len() == 0 {
        return Vec::new();
    }

    let mut representatives: Vec<usize> = Vec::new();
    for i in &v[0].0 {
        let mut ok = true;
        for j in 1..v.len() {
            if !v[j].0.contains(&(i + v[j].1 + j)) {
                ok = false;
                break;
            }
        }

        if ok {
            representatives.push(*i);
        }
    }

    representatives
}
