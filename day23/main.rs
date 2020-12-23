use std::collections::VecDeque;
use std::{fs::File, io, io::prelude::*};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day23/input.txt")?.read_to_string(&mut buffer)?;

    // Part 1 approach doesn't work for part 2 because of egregious lookup
    // times.
    let mut nums: VecDeque<usize> = VecDeque::new();
    buffer.split("").for_each(|c| {
        if c != "" {
            nums.push_back(c.parse().unwrap())
        }
    });

    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    for _ in 1..=100 {
        let current_cup = nums[0];
        let next_one = nums[1];
        let next_two = nums[2];
        let next_three = nums[3];

        let mut dst = current_cup - 1;
        while dst == next_one || dst == next_two || dst == next_three || !nums.contains(&dst) {
            if dst <= min {
                dst = max;
            } else {
                dst = dst - 1;
            }
        }

        nums.remove(3);
        nums.remove(2);
        nums.remove(1);
        for i in 0..nums.len() {
            if nums[i] == dst {
                nums.insert(i + 1, next_one);
                nums.insert(i + 2, next_two);
                nums.insert(i + 3, next_three);
                break;
            }
        }

        nums.rotate_left(1);
    }

    for i in 0..nums.len() {
        if nums[i] == 1 {
            nums.rotate_left(i);
        }
    }

    let mut answer = String::from("");
    for i in 1..nums.len() {
        answer.push_str(&nums[i].to_string());
    }
    println!("{}", answer);
    Ok(())
}
