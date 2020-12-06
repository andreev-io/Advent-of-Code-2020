use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

// TODO: better solutions is bitmaps with count_ones()
// need u32 for the alphabet

lazy_static! {
    static ref UNIQUES: Mutex<HashSet<char>> = Mutex::new(HashSet::new());
    static ref FREQ: Mutex<HashMap<char, usize>> = Mutex::new(HashMap::new());
}

fn check_group_weak(line: &str) -> usize {
    let mut set = UNIQUES.lock().unwrap();
    for c in line.chars() {
        set.insert(c);
    }

    let mut uniques = set.len();
    if set.contains(&' ') {
        uniques -= 1;
    }

    set.clear();
    return uniques;
}

fn check_group_strong(line: &str, group_size: usize) -> usize {
    let mut map = FREQ.lock().unwrap();

    for c in line.chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }

    let valid_chars = map
        .iter()
        .fold(0, |acc, (_, freq)| acc + (*freq == group_size) as usize);

    map.clear();
    return valid_chars;
}

fn main() {
    let filename = "day6/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let (mut weak_totals, mut strong_totals) = (0, 0);
    let mut group_size = 0;

    let mut owned_str = String::from("");
    for line in reader.lines() {
        let line = line.unwrap();
        if line != "" {
            owned_str.push_str(&line);
            group_size += 1;
            continue;
        } else {
            owned_str = owned_str.replace("\n", " ");
            weak_totals += check_group_weak(&owned_str);
            strong_totals += check_group_strong(&owned_str, group_size);
            group_size = 0;
            owned_str = "".to_string();
        }
    }

    // one more time for the last entry
    weak_totals += check_group_weak(&owned_str);
    strong_totals += check_group_strong(&owned_str, group_size);

    println!("p1: {}, pt2: {}", weak_totals, strong_totals);
}
