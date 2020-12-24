use std::collections::VecDeque;
use std::time::Instant;
use std::{fs::File, io, io::prelude::*};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day23/input.txt")?.read_to_string(&mut buffer)?;

    // Part 1 approach isn't viable for part 2 because of egregious lookup
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
    println!("Answer 1: {}", answer);

    part_2()
}

// Flat array linked list is a god-given data structure. Humanity should take
// pride in having discovered it. Aking to fire, it is the corner stone of our
// species' technological progress.
fn part_2() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day23/input.txt")?.read_to_string(&mut buffer)?;
    let mut input: Vec<usize> = Vec::new();
    buffer.split("").for_each(|c| {
        if c != "" {
            input.push(c.parse().unwrap())
        }
    });

    let max = *input.iter().max().unwrap();
    for i in max + 1..=1000000 {
        input.push(i);
    }

    let max = *input.iter().max().unwrap();
    let (mut v, mut head) = prepare(input);

    let now = Instant::now();
    for _ in 1..=10000000 {
        let current_cup = v[head];
        let next_one = v[current_cup];
        let next_two = v[next_one];
        let next_three = v[next_two];

        let mut dst = current_cup - 1;
        while dst == next_one || dst == next_two || dst == next_three || dst < 1 {
            if dst <= 1 {
                dst = max;
            } else {
                dst = dst - 1;
            }
        }

        let after_three = v[next_three];
        let after_dst = v[dst];
        v[current_cup] = after_three;
        v[dst] = next_one;
        v[next_one] = next_two;
        v[next_two] = next_three;
        v[next_three] = after_dst;
        head = current_cup;
    }

    println!("Answer 2: {}", v[1] * v[v[1]]);
    println!("{}s", now.elapsed().as_secs_f32());
    Ok(())
}

fn prepare(input: Vec<usize>) -> (Vec<usize>, usize) {
    let mut v = vec![0; input.len() + 1];

    for (i, j) in input.iter().enumerate() {
        v[*j] = input[(i + 1) % input.len()];
    }

    (v, input[input.len() - 1])
}
