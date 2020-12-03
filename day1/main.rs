use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Finder {
    nums: HashMap<i32, bool>,
    target: i32,
}

impl Finder {
    fn new(target: i32) -> Finder {
        Finder {
            nums: HashMap::new(),
            target: target,
        }
    }

    fn add_and_check(&mut self, num: i32) -> Option<i32> {
        self.nums.insert(num, true);

        let complement = self.target - num;
        if self.nums.contains_key(&complement) {
            return Some(num * complement);
        }

        None
    }

    fn find_triplet(&mut self) -> Option<i32> {
        for (num_outer, _) in &self.nums {
            for (num_inner, _) in &self.nums {
                let complement = self.target - num_outer - num_inner;
                if self.nums.contains_key(&complement) {
                    return Some(complement * num_outer * num_inner);
                }
            }
        }

        None
    }
}

fn main() {
    let filename = "day1/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut finder = Finder::new(2020);

    for (_, line) in reader.lines().enumerate() {
        let num = line.unwrap().parse::<i32>().unwrap();
        if let Some(product) = finder.add_and_check(num) {
            println!("Product of 2 nums summing to 2020: {}", product);
        }
    }

    if let Some(three_product) = finder.find_triplet() {
        println!("Product of 3 nums summing to 2020: {}", three_product);
    }
}
