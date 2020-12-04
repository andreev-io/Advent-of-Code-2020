use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

//TODO: REFACTOR BIG TIME
// REFACTOR MACHINE GO brrrrrr!

enum Entry {
    Weak,
    Strong,
    Bad,
}

struct Checker {
    re_line: regex::Regex,
    re_entry: regex::Regex,
    dict: HashMap<String, String>,
}

impl Checker {
    fn new() -> Checker {
        Checker {
            re_line: Regex::new(r"([a-z]{3}:#?\w+)").unwrap(),
            re_entry: Regex::new(r"(.*):(.*)").unwrap(),
            dict: HashMap::new(),
        }
    }

    fn check_entry_weak(&self) -> bool {
        let is_good = self.dict.contains_key("byr") 
                    && self.dict.contains_key("iyr")
                    && self.dict.contains_key("eyr")
                    && self.dict.contains_key("hgt")
                    && self.dict.contains_key("hcl")
                    && self.dict.contains_key("pid")
                    && self.dict.contains_key("ecl");

        is_good
    }

    fn check_byr(byr: &str) -> bool {
        match byr.parse::<i32>() {
            Ok(yr) => return 1920 <= yr && yr <= 2002,
            Err(_) => return false
        }
    }

    fn check_iyr(iyr: &str) -> bool {
        match iyr.parse::<i32>() {
            Ok(yr) => return 2010 <= yr && yr <= 2020,
            Err(_) => return false
        }
    }

    fn check_eyr(eyr: &str) -> bool {
        match eyr.parse::<i32>() {
            Ok(yr) => return 2020 <= yr && yr <= 2030,
            Err(_) => return false
        }
    }

    fn check_hgt(hgt: &str) -> bool {
        let re_hgt = Regex::new(r"([0-9]{2,3})([a-z]{2})").unwrap();
        match re_hgt.captures(hgt) {
            Some(captures) => {
                let height = captures.get(1).map_or("", |m| m.as_str());
                let units = captures.get(2).map_or("", |m| m.as_str());
                match height.parse::<i32>() {
                    Ok(h) => {
                        if units == "in" {
                            return 59 <= h && h <= 76;
                        } else if units == "cm" {
                            return 150 <= h && h <= 193;
                        } else {
                            return false;
                        }
                    }
                    Err(_) => return false
                }
            },
            None => return false
        };
    }

    fn check_hcl(hcl: &str) -> bool {
        let re_hcl = Regex::new(r"#[a-f0-9]{6}$").unwrap();
        return re_hcl.is_match(hcl);
    }

    fn check_ecl(ecl: &str) -> bool {
        let eyes = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        eyes.iter().any(|&eye| eye == ecl)
    }

    fn check_pid(pid: &str) -> bool {
        match pid.parse::<i32>() {
            Ok(_) => return pid.len() == 9,
            Err(_) => return false
        }
    }

    fn check_entry_strong(&self) -> bool {
        let byr_ok =  match self.dict.get("byr") {
            Some(byr) => Checker::check_byr(byr),
            None => false
        };

        let iyr_ok =  match self.dict.get("iyr") {
            Some(iyr) => Checker::check_iyr(iyr),
            None => false
        };

        let eyr_ok =  match self.dict.get("eyr") {
            Some(eyr) => Checker::check_eyr(eyr),
            None => false
        };

        let hgt_ok =  match self.dict.get("hgt") {
            Some(hgt) => Checker::check_hgt(hgt),
            None => false
        };

        let hcl_ok =  match self.dict.get("hcl") {
            Some(hcl) => Checker::check_hcl(hcl),
            None => false
        };

        let ecl_ok =  match self.dict.get("ecl") {
            Some(ecl) => Checker::check_ecl(ecl),
            None => false
        };

        let pid_ok =  match self.dict.get("pid") {
            Some(pid) => Checker::check_pid(pid),
            None => false
        };

        byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok
    }

    fn check_entry(&mut self, entry: &String) -> Entry {
        for parameter in self.re_line.captures_iter(entry) {
            let param = parameter.get(1).map_or("", |m| m.as_str());
            let captures = self.re_entry.captures(param).unwrap();
            let key = captures.get(1).map_or("", |m| m.as_str());
            let val = captures.get(2).map_or("", |m| m.as_str());
            self.dict.insert(key.to_string(), val.to_string());
        }

        if !self.check_entry_weak() {
            self.dict.clear();
            return Entry::Bad;
        }

        if !self.check_entry_strong() {
            self.dict.clear();
            return Entry::Weak;
        }

        self.dict.clear();
        Entry::Strong
    }
}

fn main() {
    let filename = "day4/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut weak_count = 0;
    let mut strong_count = 0;

    let mut owned_string: String = String::from("");
    let mut checker = Checker::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line != "" {
            owned_string.push_str(&line);
            owned_string.push_str(&" ");
        } else {
            match checker.check_entry(&owned_string) {
                Entry::Bad => {},
                Entry::Weak => weak_count += 1,
                Entry::Strong => strong_count += 1,
            }

            owned_string = String::from("");
        }
    }

    // Check the last entry
    match checker.check_entry(&owned_string) {
        Entry::Bad => {},
        Entry::Weak => weak_count += 1,
        Entry::Strong => strong_count += 1,
    }

    println!("There are {} solid passports and {} meh (but still fine) passports, total {} valid", strong_count, weak_count, strong_count+weak_count);
}
