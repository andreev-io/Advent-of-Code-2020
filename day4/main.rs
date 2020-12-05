use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum Entry {
    Bad,
    Weak,
    Strong,
}

lazy_static! {
    static ref RE_HCL: Regex = Regex::new(r"#[a-f0-9]{6}$").unwrap();
    static ref EYES: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    static ref CHECKS: [(&'static str, fn(&str) -> bool); 7] = [
        ("ecl", |ecl| EYES.iter().any(|&eye| eye == ecl)),
        ("hcl", |hcl| RE_HCL.is_match(hcl)),
        ("pid", |pid| pid.len() == 9),
        ("byr", |byr| {
            let yr = byr.parse::<i32>().unwrap();
            1920 <= yr && yr <= 2002
        }),
        ("iyr", |iyr| {
            let yr = iyr.parse::<i32>().unwrap();
            2010 <= yr && yr <= 2020
        }),
        ("eyr", |eyr| {
            let yr = eyr.parse::<i32>().unwrap();
            2020 <= yr && yr <= 2030
        }),
        ("hgt", |hgt| {
            if let Some(height) = hgt.strip_suffix("cm") {
                let h = height.parse().unwrap();
                150 <= h && h <= 193
            } else if let Some(height) = hgt.strip_suffix("in") {
                let h = height.parse().unwrap();
                59 <= h && h <= 76
            } else {
                false
            }
        }),
    ];
}

fn check_line(line: &str) -> Entry {
    if CHECKS.iter().all(|(field, check)| {
        if line.contains(field) {
            let postfix = line.split(field).last().unwrap();
            let value = postfix
                .split_whitespace()
                .next()
                .unwrap()
                .trim_start_matches(':');
            return check(value);
        }

        false
    }) {
        return Entry::Strong;
    }

    if CHECKS.iter().all(|(field, _)| line.contains(field)) {
        Entry::Weak
    } else {
        Entry::Bad
    }
}

fn main() {
    let filename = "day4/input.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let (mut weak_count, mut strong_count) = (0, 0);

    while let Ok(len) = reader.read_line(&mut buf) {
        // If a single symbol, it's a new line. Meaning we passed a block with
        // an entry, so check the buffer and then clear it. Otherwise keep
        // reading.
        if len <= 1 {
            let line = &buf.replace("\n", " ");
            match check_line(line) {
                Entry::Bad => {}
                Entry::Weak => weak_count += 1,
                Entry::Strong => strong_count += 1,
            }

            buf.clear();
            if len == 0 {
                break;
            }
        }
    }
    println!(
        "There are {} solid passports and {} meh (but still fine) passports, total {} valid",
        strong_count,
        weak_count,
        strong_count + weak_count
    );
}
