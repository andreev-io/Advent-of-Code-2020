#![feature(destructuring_assignment)]
use std::{fs::File, io, io::prelude::*};

// TODO: fix i32 <-> usize shenanigans

const MOVES: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day11/input.txt")?.read_to_string(&mut buffer)?;
    let mut seats_one: Vec<Vec<char>> = buffer
        .split("\n")
        .map(|s| s.trim().chars().collect())
        .collect();
    let mut seats_two = seats_one.clone();

    let c1;
    loop {
        let transformed = transform_one(&seats_one);
        if transformed == seats_one {
            c1 = count(&transformed);
            break;
        } else {
            seats_one = transformed;
        }
    }

    let c2;
    loop {
        let transformed = transform_two(&seats_two);
        if transformed == seats_two {
            c2 = count(&transformed);
            break;
        } else {
            seats_two = transformed;
        }
    }

    println!("Answer 1: {}, 2: {}", c1, c2);
    Ok(())
}

// in general, i is row, j is column
fn exists(seats: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    i >= 0 && j >= 0 && i < seats.len() as i32 && j < seats[0].len() as i32
}

fn transform_one(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut copy = seats.clone();
    for (i, row) in seats.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            if *seat == '.' {
                continue;
            }

            let mut neighbors: Vec<&char> = Vec::new();
            for (di, dj) in MOVES.iter() {
                if exists(&seats, i as i32 + di, j as i32 + dj) {
                    neighbors.push(&seats[(i as i32 + di) as usize][(j as i32 + dj) as usize]);
                }
            }

            let occupied_self = *seat == '#';
            let occupied_neighbors = neighbors
                .iter()
                .fold(0, |acc, &c| acc + if *c == '#' { 1 } else { 0 });
            if occupied_neighbors == 0 && !occupied_self {
                copy[i][j] = '#';
            } else if occupied_self && occupied_neighbors >= 4 {
                copy[i][j] = 'L';
            }
        }
    }

    copy
}

fn transform_two(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut copy = seats.clone();
    for (i, row) in seats.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            if *seat == '.' {
                continue;
            }

            let mut closest_neighbors: Vec<&char> = Vec::new();
            for (di, dj) in MOVES.iter() {
                let (mut m, mut k) = (i as i32 + di, j as i32 + dj);
                while exists(&seats, m, k) && seats[m as usize][k as usize] == '.' {
                    (m, k) = (m + di, k + dj);
                }

                if exists(&seats, m, k) {
                    closest_neighbors.push(&seats[m as usize][k as usize]);
                }
            }

            let occupied_self = *seat == '#';
            let occupied_neighbors = closest_neighbors
                .iter()
                .fold(0, |acc, &c| acc + if *c == '#' { 1 } else { 0 });
            if occupied_neighbors == 0 && !occupied_self {
                copy[i][j] = '#';
            } else if occupied_self && occupied_neighbors >= 5 {
                copy[i][j] = 'L';
            }
        }
    }

    copy
}

fn count(seats: &Vec<Vec<char>>) -> usize {
    seats.iter().flatten().filter(|&&c| c == '#').count()
}
