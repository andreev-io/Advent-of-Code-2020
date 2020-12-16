#![feature(destructuring_assignment)]
use std::collections::HashMap;
use std::{fs, io};

fn main() -> io::Result<()> {
    let file_contents = fs::read_to_string("day15/input.txt").unwrap();
    let mut lines = file_contents.lines();
    let mut v: HashMap<usize, usize> = HashMap::new();

    let line = lines.next().unwrap();
    let mut digits = line.split(",").map(|s| s.parse::<usize>().unwrap());

    let mut i = 0;
    let mut prev: usize = 0;
    let mut is_prev_new: bool = false;
    loop {
        i += 1;
        if let Some(d) = digits.next() {
            prev = d;
            is_prev_new = true;
            v.insert(d, i);
        } else {
            if is_prev_new && prev != 0 {
                if v.contains_key(&0) {
                    is_prev_new = false;
                } else {
                    is_prev_new = true;
                }

                v.insert(prev, i - 1);
                prev = 0;
            } else {
                // let prev_index = v.get(&prev).unwrap();
                let mut prev_index = 0;
                if let Some(prev_i) = v.get(&prev) {
                    prev_index = *prev_i;
                } else {
                    prev_index = i - 1;
                }
                // println!("prev index {}", prev_index);
                let new_num = i - 1 - prev_index;
                // println!("saying {}", new_num);
                v.insert(prev, i - 1);
                prev = new_num;
                is_prev_new = !v.contains_key(&new_num);
            }
        }
        // println!("index: {} saying: {} was new: {}", i, prev, is_prev_new);
        if i == 30000000 {
            break;
        }
    }
    println!("{}", prev);

    Ok(())
}
