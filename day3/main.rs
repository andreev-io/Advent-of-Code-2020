use std::fs;

fn count_trees(right: usize, down: usize, file_contents: &str) -> u64 {
    let mut trees = 0;
    let mut x = 0;
    let mut chars = 0;
    let lines = file_contents.lines();

    for (y, line) in lines.enumerate().step_by(down) {
        if y == 0 {
            chars = line.chars().count();
        }

        if line.chars().nth(x).unwrap() == '#' {
            trees += 1
        }

        x = (x + right) % chars;
    }

    trees
}

fn main() {
    let file_contents = fs::read_to_string("day3/input.txt").unwrap();

    let case_one = count_trees(3, 1, &file_contents);
    let case_two = count_trees(1, 1, &file_contents);
    let case_three = count_trees(3, 1, &file_contents);
    let case_four = count_trees(5, 1, &file_contents);
    let case_five = count_trees(7, 1, &file_contents);
    let case_six = count_trees(1, 2, &file_contents);

    println!("Trees in (3, 1): {}", case_one);
    println!(
        "Product of trees in multple cases: {}",
        case_two * case_three * case_four * case_five * case_six
    );
}
