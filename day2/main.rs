use std::fs::File;
use std::io::{BufRead, BufReader};

enum Security {
    Good,
    Bad,
}

fn validate_weak(password: &str, letter: char, min: usize, max: usize) -> Security {
    let occurences = password.matches(letter).count();
    let res = if min <= occurences && occurences <= max {
        Security::Good
    } else {
        Security::Bad
    };

    res
}

fn validate_strong(password: &str, letter: char, pos_one: usize, pos_two: usize) -> Security {
    let first_char = password.chars().nth(pos_one - 1).unwrap();
    let second_char = password.chars().nth(pos_two - 1).unwrap();

    let first_hit = first_char == letter;
    let second_hit = second_char == letter;

    let res = if first_hit ^ second_hit {
        Security::Good
    } else {
        Security::Bad
    };

    res
}

fn main() {
    let filename = "day2/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut weak_count = 0;
    let mut strong_count = 0;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let chunks: Vec<&str> = line.split(" ").collect();
        let criteria: Vec<&str> = chunks[0].split("-").collect();
        let min = criteria[0].parse::<usize>().unwrap();
        let max = criteria[1].parse::<usize>().unwrap();
        let letter = chunks[1].chars().nth(0).unwrap();
        let password = chunks[chunks.len() - 1];

        let weak_security = validate_weak(password, letter, min, max);
        match weak_security {
            Security::Good => weak_count +=1,
            _ => {}
        }
        
        
        let (pos_one, pos_two) = (min, max);
        let strong_security = validate_strong(password, letter, pos_one, pos_two);
        match strong_security {
            Security::Good => strong_count += 1,
            _ => {}
        }
    }

    println!(
        "There are {} weakly valid password, {} strong valid password",
        weak_count, strong_count
    );
}
