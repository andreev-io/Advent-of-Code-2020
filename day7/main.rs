use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs::File, io, io::prelude::*};

lazy_static! {
    static ref RE_CONTAINER: Regex = Regex::new(r"(\w+ \w+)?").unwrap();
    static ref RE_CONTENT: Regex = Regex::new(r"([0-9]) (\w+ \w+)").unwrap();
}

struct Input {
    contents: HashMap<String, Vec<(String, usize)>>,
}

impl Input {
    fn new(input: &str) -> Input {
        let mut contents = HashMap::new();
        let lines = input.split("\n").map(|s| s.trim());

        for line in lines {
            let key = RE_CONTAINER.find(line).unwrap().as_str().to_string();
            contents.insert(key.clone(), Vec::new());

            for cap in RE_CONTENT.captures_iter(line) {
                let (content, count) = (cap[2].to_string(), cap[1].parse().unwrap());
                contents.get_mut(&key).unwrap().push((content, count));
            }
        }

        Input { contents }
    }

    fn count_containers(&self, inner: &str) -> usize {
        self.contents
            .iter()
            .filter(|(color, _)| self.contains(color, inner))
            .count()
    }

    fn contains(&self, outer: &str, inner: &str) -> bool {
        self.contents[outer]
            .iter()
            .any(|(s, _)| s == inner || self.contains(s, inner))
    }

    fn count_contents(&self, outer: &str) -> usize {
        self.count_contents_inner(outer) - 1
    }

    fn count_contents_inner(&self, outer: &str) -> usize {
        self.contents[outer].iter().fold(1, |acc, (inner, count)| {
            acc + count * self.count_contents_inner(inner)
        })
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day7/input.txt")?.read_to_string(&mut buffer)?;

    let input = Input::new(&buffer);
    let target = String::from("shiny gold");
    println!(
        "Bags containing {}: {}, bags in {}: {}",
        target,
        input.count_containers(&target),
        target,
        input.count_contents(&target)
    );

    Ok(())
}
