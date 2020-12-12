#![feature(destructuring_assignment)]
use std::{fs::File, io, io::prelude::*, iter::FromIterator};

// This is part 2 only. It sort of simply extends part 1, and I have to go study
// for my finals instead of recreating the original code.
#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    px: i32,
    py: i32,
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day12/input.txt")?.read_to_string(&mut buffer)?;

    let moves = buffer.split("\n").map(|s| {
        let mut chars = s.chars();
        let m = chars.next().unwrap();
        let arg = String::from_iter(chars).parse::<i32>().unwrap();
        (m, arg)
    });

    let mut ship = Ship {
        x: 0,
        y: 0,
        px: 10,
        py: 1,
    };

    for (mov, arg) in moves {
        match (mov, arg) {
            ('N', _) => ship.py += arg,
            ('S', _) => ship.py -= arg,
            ('E', _) => ship.px += arg,
            ('W', _) => ship.px -= arg,
            ('L', 270) | ('R', 90) => {
                let t = ship.px;
                ship.px = ship.py;
                ship.py = -t;
            }
            ('L', 90) | ('R', 270) => {
                let t = ship.px;
                ship.px = -ship.py;
                ship.py = t;
            }
            ('L', 180) | ('R', 180) => {
                ship.px = -1 * ship.px;
                ship.py = -1 * ship.py;
            }
            ('F', _) => {
                ship.x += arg * ship.px;
                ship.y += arg * ship.py;
            }
            _ => {}
        }
    }

    println!("Manhattan {}", ship.x.abs() + ship.y.abs());
    Ok(())
}
