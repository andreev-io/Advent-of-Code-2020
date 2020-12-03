use std::fs::File;
use std::io::{BufRead, BufReader};

enum Validity {
    Valid,
    Invalid,
}

fn validate_weak(password: &str, letter: char, min: i32, max: i32) -> Validity {
    let occurences = password.matches(letter).count() as i32;
    let res = if min <= occurences && occurences <= max {
        Validity::Valid
    } else {
        Validity::Invalid
    };

    res
}

fn validate_strong(password: &str, letter: char, pos_one: usize, pos_two: usize) -> Validity {
    let first_char = password.chars().nth(pos_one - 1).unwrap();
    let second_char = password.chars().nth(pos_two - 1).unwrap();

    let res = if (first_char == letter && second_char != letter)
        || (first_char != letter && second_char == letter)
    {
        Validity::Valid
    } else {
        Validity::Invalid
    };

    res
}

fn main() {
    let filename = "day2/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut weakly_valid = 0;
    let mut strong_valid = 0;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let chunks: Vec<&str> = line.split(" ").collect();
        let criteria: Vec<&str> = chunks[0].split("-").collect();
        let min = criteria[0].parse::<i32>().unwrap();
        let max = criteria[1].parse::<i32>().unwrap();
        let letter = chunks[1].chars().nth(0).unwrap();
        let password = chunks[chunks.len() - 1];

        let weak_validity = validate_weak(password, letter, min, max);
        let (pos_one, pos_two) = (min as usize, max as usize);
        let strong_validity = validate_strong(password, letter, pos_one, pos_two);
        match (weak_validity, strong_validity) {
            (Validity::Valid, Validity::Valid) => {
                weakly_valid += 1;
                strong_valid += 1
            }
            (Validity::Invalid, Validity::Valid) => strong_valid += 1,
            (Validity::Valid, Validity::Invalid) => weakly_valid += 1,
            _ => {}
        }
    }

    println!(
        "There are {} weakly valid password, {} strong valid password",
        weakly_valid, strong_valid
    );
}
