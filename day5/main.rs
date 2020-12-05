use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = "day5/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut heap = BinaryHeap::new();

    for (_, line) in reader.lines().enumerate() {
        heap.push(line.unwrap().chars().fold(0, |acc, c| {
            (acc << 1) + if c == 'B' || c == 'R' { 1 } else { 0 }
        }));
    }

    println!("max id is {}", heap.peek().unwrap());
    while let (Some(i), Some(j)) = (heap.pop(), heap.peek()) {
        if i != j + 1 {
            println!("my seat is {}!", j + 1);
            break;
        }
    }
}
