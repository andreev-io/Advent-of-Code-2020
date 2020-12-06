#![feature(iterator_fold_self)]
// nightly only :(

use std::{fs::File, io, io::prelude::*};

// input is multiple lines representing a group. split by lines, then for each
// line take the character as ascii and mark its presence in the bitmap. return
// number of ones in the bitmap.
fn or_count(group: &str) -> u32 {
    let bitmap = group.split("\n").fold(0, |mut acc: u32, line| {
        for c in line.chars() {
            acc |= 1 << (c as u8 - 'a' as u8);
        }

        acc
    });

    bitmap.count_ones()
}

fn and_count(group: &str) -> u32 {
    let bitmap: Option<u32> = group
        .split("\n")
        .map(|line| {
            // transform the line into a bitmap for the person
            let mut person = 0;
            for c in line.chars() {
                person |= 1 << (c as u8 - 'a' as u8);
            }

            person
        })
        // fold bitmaps across people in the group
        .fold_first(|p1, p2| p1 & p2);

    bitmap.unwrap().count_ones()
}

fn main() -> io::Result<()> {
    let mut f = File::open("day6/input.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    // do i really need to clone? what's a better way?
    let s1 = buffer.split("\n\n").map(|s| s.trim());
    let s2 = s1.clone();

    let sum_one: u32 = s1.map(|group| or_count(group)).sum();
    let sum_two: u32 = s2.map(|s| and_count(s)).sum();

    println!("pt 1: {} pt 2: {}", sum_one, sum_two);
    Ok(())
}
