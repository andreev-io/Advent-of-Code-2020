use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "day5/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut heap = BinaryHeap::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut id = 0;
        for pos in line.chars() {
            match pos {
                'F' | 'L' => id = id << 1,
                'B' | 'R' => id = (id << 1) | 1,
                _ => panic!("bad input"),
            }
        }

        heap.push(id);
    }

    println!("max id is {}", heap.peek().unwrap());
    while let (Some(i), Some(j)) = (heap.pop(), heap.peek()) {
        if i != j + 1 {
            println!("my seat is {}!", j + 1);
            break;
        }
    }
}
