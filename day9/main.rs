use lazy_static::lazy_static;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::sync::Mutex;
use std::{fs::File, io, io::prelude::*};

const SIZE: i64 = 25;

lazy_static! {
    static ref DEQUEUE: Mutex<VecDeque<i64>> = Mutex::new(VecDeque::new());
    static ref SET: Mutex<HashSet<i64>> = Mutex::new(HashSet::new());
}

fn check_num(num: i64) -> bool {
    let q = DEQUEUE.lock().unwrap();
    let s = SET.lock().unwrap();

    q.iter().any(|n| 2 * n != num && s.contains(&(num - n)))
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day9/input.txt")?.read_to_string(&mut buffer)?;
    let mut nums = buffer
        .split("\n")
        .map(|s| s.trim())
        .map(|s| s.parse::<i64>().unwrap());

    // Fill in the preamble.
    for _ in 0..SIZE {
        let (mut q, mut s) = (DEQUEUE.lock().unwrap(), SET.lock().unwrap());
        let num = nums.nth(0).unwrap();
        s.insert(num);
        q.push_back(num);
    }

    let mut pt_one_answer = 0;
    for num in nums {
        if !check_num(num) {
            pt_one_answer = num;
            break;
        }

        let (mut q, mut s) = (DEQUEUE.lock().unwrap(), SET.lock().unwrap());
        s.remove(&q.pop_front().unwrap());
        q.push_back(num);
        s.insert(num);
    }

    println!("Part 1 answer: {}", pt_one_answer);

    let mut pt_two_answer = 0;
    let nums: Vec<i64> = buffer
        .split("\n")
        .map(|s| s.trim())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    for i in 0..nums.len() {
        let (mut j, mut sum) = (i + 1, nums[i]);
        while sum < pt_one_answer {
            sum += nums[j];

            if sum == pt_one_answer {
                let contiguous = Vec::from_iter(nums[i..j + 1].iter().cloned());
                let min = contiguous.iter().min().unwrap();
                let max = contiguous.iter().max().unwrap();
                pt_two_answer = min + max;
                break;
            }

            j += 1;
        }
    }

    println!("Part 2 answer: {}", pt_two_answer);
    Ok(())
}
