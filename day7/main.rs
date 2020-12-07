use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::{fs::File, io, io::prelude::*};

lazy_static! {
    static ref RE_CONTAINER: Regex = Regex::new(r"(\w+ \w+)?").unwrap();
    static ref RE_CONTENT: Regex = Regex::new(r"([0-9]) (\w+ \w+)").unwrap();
}

// !!!!!!topological sort???????

// TODO: fix string types
// map each bag to the vector of (bag, num_bags) tuples that are contained in it
// map each bag to the vector of bags where it is directly contained
fn preprocess(
    s: &str,
) -> (
    HashMap<String, Vec<(String, usize)>>,
    HashMap<String, Vec<String>>,
) {
    let (mut contents_map, mut containers_map) = (HashMap::new(), HashMap::new());
    let map = s.split("\n").map(|s| s.trim());
    for line in map {
        let container = RE_CONTAINER.find(line).unwrap().as_str();
        for cap in RE_CONTENT.captures_iter(line) {
            let (content, count) = (cap[2].to_string(), cap[1].parse().unwrap());
            let contents_v = contents_map.entry(container.to_string()).or_insert(Vec::new());
            contents_v.push((content.clone(), count));
            let containers_v = containers_map.entry(content).or_insert(Vec::new());
            // TODO: duplicates possible anywhere?
            containers_v.push(container.to_string());
        }
    }

    (contents_map, containers_map)
}

fn main() -> io::Result<()> {
    let mut f = File::open("day7/input.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let (contents_map, containers_map) = preprocess(&buffer);

    let mut queue: VecDeque<String> = VecDeque::new();
    let query = "shiny gold".to_string();
    queue.push_back(query);

    let mut storage_options = HashSet::new();
    while queue.len() != 0 {
        let el = queue.pop_front().unwrap();
        match containers_map.get(&el) {
            Some(containers) => {
                for container in containers.into_iter() {
                    queue.push_back(container.to_string());
                    storage_options.insert(container);
                }
            },
            None => continue,
        }
    }

    println!("Storage options in part 1: {}", storage_options.len());
    println!("Contents max in part2: {}", count_contents("shiny gold", &contents_map)-1);

    Ok(())
}

fn count_contents(s: &str, map: &HashMap<String, Vec<(String, usize)>>,) -> usize {
    let mut total = 1;
    match map.get(s) {
        Some(contents) => {
            for (content, num) in contents.into_iter() {
                total += num * count_contents(content, map);
            }
        }
        _ => {}
    }

    total
}
