use std::fs;

fn count_trees(right: i32, down: i32, file_contents: &str) -> i32 {
    let mut trees = 0;
    let mut x: i32 = 0;
    let mut chars = 0;
    let lines = file_contents.lines();

    for (y, line) in lines.enumerate().step_by(down as usize) {
        if y == 0 {
            chars = line.chars().count() as i32;
        }

        if line.chars().nth((x) as usize).unwrap() == '#' {
            trees += 1
        }

        x = (x + right) % chars;
    }

    trees
}

fn main() {
    let file_contents = fs::read_to_string("day3/input.txt").unwrap();

    let case_one = count_trees(3, 1, &file_contents) as u64;
    let case_two = count_trees(1, 1, &file_contents) as u64;
    let case_three = count_trees(3, 1, &file_contents) as u64;
    let case_four = count_trees(5, 1, &file_contents) as u64;
    let case_five = count_trees(7, 1, &file_contents) as u64;
    let case_six = count_trees(1, 2, &file_contents) as u64;

    println!("Trees in (3, 1): {}", case_one); 
    println!("Product of trees in multple cases: {}", case_two * case_three * case_four * case_five * case_six);
}
