use std::{fs::File, io, io::prelude::*};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day10/input.txt")?.read_to_string(&mut buffer)?;
    let mut nums: Vec<usize> = buffer
        .split("\n")
        .map(|s| s.trim())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    nums.push(0);
    nums.sort();

    let (ones, threes, _) = nums
        .iter()
        .rev()
        .fold((0, 1, 0), |(ones, threes, prev), n| {
            if prev - n == 1 {
                (ones + 1, threes, *n)
            } else if prev - n == 3 {
                (ones, threes + 1, *n)
            } else {
                (ones, threes, *n)
            }
        });

    println!("Part 1 answer: {}", ones * threes);

    let mut connections = vec![0 as usize; nums.len()];
    connections[0] = 1;
    for i in 1..nums.len() {
        let num = nums[i];

        // how many ways can we connect to num?
        if i as i32 - 3 >= 0 && num - nums[i - 3] <= 3 {
            connections[i] += connections[i - 3] + connections[i - 2] + connections[i - 1];
        } else if i as i32 - 2 >= 0 && num - nums[i - 2] <= 3 {
            connections[i] += connections[i - 2] + connections[i - 1];
        } else if i as i32 - 1 >= 0 && num - nums[i - 1] <= 3 {
            connections[i] += connections[i - 1];
        }
    }

    println!("Part 2 answer: {}", connections[nums.len() - 1]);
    Ok(())
}
