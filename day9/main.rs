use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io,
    io::prelude::*,
    time::Instant,
};

const SIZE: i64 = 25;

fn main() -> io::Result<()> {
    let now_one = Instant::now();
    let mut buffer = String::new();
    File::open("day9/input.txt")?.read_to_string(&mut buffer)?;
    let mut q: VecDeque<i64> = VecDeque::new();
    let mut s: HashSet<i64> = HashSet::new();
    let mut nums = buffer
        .split("\n")
        .map(|s| s.trim())
        .map(|s| s.parse::<i64>().unwrap());

    // Fill in the preamble.
    for _ in 0..SIZE {
        let num = nums.nth(0).unwrap();
        s.insert(num);
        q.push_back(num);
    }

    let mut pt_one_answer = 0;
    for num in nums {
        if q.iter().all(|n| 2 * n == num || !s.contains(&(num - n))) {
            pt_one_answer = num;
            break;
        }

        s.remove(&q.pop_front().unwrap());
        q.push_back(num);
        s.insert(num);
    }

    println!("Part 1 answer: {}", pt_one_answer);

    let pt_two_answer;
    let nums: Vec<i64> = buffer
        .split("\n")
        .map(|s| s.trim())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let (mut i, mut j) = (0, 1);
    let mut sum = nums[i] + nums[j];
    loop {
        if sum > pt_one_answer {
            sum -= nums[i];
            i += 1;
        }

        if sum < pt_one_answer {
            j += 1;
            sum += nums[j];
        }

        if sum == pt_one_answer {
            let contiguous = &nums[i..j + 1];
            let min = contiguous.iter().min().unwrap();
            let max = contiguous.iter().max().unwrap();
            pt_two_answer = min + max;
            break;
        }
    }

    println!("Part 2 answer: {}", pt_two_answer);
    println!("Runtime: {}μs", now_one.elapsed().as_micros());
    Ok(())
}
